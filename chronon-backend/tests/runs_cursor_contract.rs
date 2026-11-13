//! Contracts for runs `DataTable` cursor paging (`read_runs_page`) over an in-memory coordinator.

#![allow(missing_docs)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::cell::RefCell;
use std::collections::HashSet;
use std::convert::Infallible;
use std::future::{ready, Future};
use std::pin::pin;
use std::task::{Context, Poll, Waker};

use chrono::{DateTime, Duration, TimeZone, Utc};
use chronon_backend::{
    decode_run_key, encode_run_key, read_runs_page, runs_cursor_scope, RunsPageRows, RunsRead,
    RUNS_FILTER_SCAN_CAP,
};
use chronon_core::{Run as CoreRun, RunKey, RunStatus as CoreRunStatus};
use orbital_data::DataValue;
use orbital_paging::{
    FilterLogicWire, FilterQuery, FilterRuleParam, Page, PageCursor, PageRequest,
};

const PAGE: u32 = 20;
const STATUSES: [CoreRunStatus; 5] = [
    CoreRunStatus::Success,
    CoreRunStatus::Failed,
    CoreRunStatus::Timeout,
    CoreRunStatus::Queued,
    CoreRunStatus::Running,
];

/// Every future here is ready on first poll; the fake coordinator never waits.
fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(out) = future.as_mut().poll(&mut cx) {
            return out;
        }
    }
}

fn base_time() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap()
}

/// `count` runs: every three share a timestamp (so `run_id` breaks ties), ids are a
/// permutation of the index, and statuses cycle through [`STATUSES`].
fn seed_runs(count: usize) -> Vec<CoreRun> {
    (0..count)
        .map(|i| {
            let at = base_time() + Duration::seconds(i64::try_from(i / 3).unwrap());
            let mut run = CoreRun::new("demo_script", at);
            run.run_id = format!("run-{:05}", (i * 7919) % count);
            run.job_id = Some(if i % 4 == 0 { "job-b" } else { "job-a" }.to_string());
            run.status = STATUSES[i % STATUSES.len()];
            run
        })
        .collect()
}

fn newest_first(runs: &mut [CoreRun]) {
    runs.sort_by(|a, b| {
        b.scheduled_for
            .cmp(&a.scheduled_for)
            .then_with(|| b.run_id.cmp(&a.run_id))
    });
}

/// In-memory coordinator with the `list_runs` / `list_runs_after` contract; logs every read.
struct FakeCoordinator {
    runs: Vec<CoreRun>,
    job_id: Option<String>,
    reads: RefCell<Vec<RunsRead>>,
}

impl FakeCoordinator {
    fn new(runs: Vec<CoreRun>, job_id: Option<&str>) -> Self {
        let mut runs = runs;
        newest_first(&mut runs);
        Self {
            runs,
            job_id: job_id.map(String::from),
            reads: RefCell::new(Vec::new()),
        }
    }

    fn matching<'a>(&'a self, status: Option<&'a str>) -> impl Iterator<Item = &'a CoreRun> + 'a {
        self.runs.iter().filter(move |r| {
            self.job_id
                .as_deref()
                .is_none_or(|id| r.job_id.as_deref() == Some(id))
                && status.is_none_or(|s| r.status.to_string() == s)
        })
    }

    fn read(&self, read: RunsRead) -> std::future::Ready<Result<Vec<CoreRun>, Infallible>> {
        let rows = match &read {
            RunsRead::Offset {
                status,
                offset,
                limit,
            } => self
                .matching(*status)
                .skip(*offset)
                .take(*limit)
                .cloned()
                .collect(),
            RunsRead::After {
                status,
                after,
                limit,
            } => self
                .matching(*status)
                .filter(|r| {
                    (r.scheduled_for, r.run_id.as_str())
                        < (after.scheduled_for, after.run_id.as_str())
                })
                .take(*limit)
                .cloned()
                .collect(),
        };
        self.reads.borrow_mut().push(read);
        ready(Ok(rows))
    }

    fn take_reads(&self) -> Vec<RunsRead> {
        std::mem::take(&mut *self.reads.borrow_mut())
    }

    fn page(&self, request: &PageRequest) -> RunsPageRows {
        block_on(read_runs_page(request, self.job_id.as_deref(), |r| {
            self.read(r)
        }))
        .unwrap()
    }
}

fn status_filter(value: &str) -> FilterQuery {
    FilterQuery {
        logic: FilterLogicWire::And,
        items: vec![FilterRuleParam {
            field: "status".into(),
            operator: "equals".into(),
            value: DataValue::Text(value.into()),
        }],
    }
}

fn request(offset: u32, filter: Option<FilterQuery>, cursor: Option<String>) -> PageRequest {
    PageRequest::with_query(offset, PAGE, None, filter, None).with_cursor(cursor)
}

fn expect_page(rows: RunsPageRows) -> (Vec<CoreRun>, Option<String>) {
    match rows {
        RunsPageRows::Page { rows, next_cursor } => (rows, next_cursor),
        RunsPageRows::Scanned(_) => panic!("expected the cursor path, got an offset scan"),
    }
}

/// Pages with Next until `has_more` is false; returns run ids in order and every read.
fn page_through(
    fake: &FakeCoordinator,
    filter: Option<&FilterQuery>,
) -> (Vec<String>, Vec<RunsRead>) {
    let mut ids = Vec::new();
    let mut reads = Vec::new();
    let mut offset = 0;
    let mut cursor = None;
    loop {
        let req = request(offset, filter.cloned(), cursor.take());
        let (rows, next_cursor) = expect_page(fake.page(&req));
        let page = Page::from_oversized(rows, PAGE, None).with_next_cursor(next_cursor);
        ids.extend(page.items.iter().map(|r| r.run_id.clone()));
        reads.extend(fake.take_reads());
        if !page.has_more {
            assert!(
                page.next_cursor.is_none(),
                "last page must not carry a cursor"
            );
            break;
        }
        assert!(
            page.next_cursor.is_some(),
            "a page with more rows carries a cursor"
        );
        cursor = page.next_cursor;
        offset += PAGE;
    }
    (ids, reads)
}

fn expected_ids(runs: &[CoreRun], keep: impl Fn(&CoreRun) -> bool) -> Vec<String> {
    let mut runs: Vec<CoreRun> = runs.iter().filter(|r| keep(r)).cloned().collect();
    newest_first(&mut runs);
    runs.into_iter().map(|r| r.run_id).collect()
}

fn assert_each_once(ids: &[String]) {
    let unique: HashSet<&String> = ids.iter().collect();
    assert_eq!(unique.len(), ids.len(), "a run came back twice");
}

#[test]
fn runs_cursor_pages_2500_runs_each_once_in_order_happy() {
    let runs = seed_runs(2_500);
    let fake = FakeCoordinator::new(runs.clone(), None);
    let (ids, reads) = page_through(&fake, None);

    assert_eq!(ids, expected_ids(&runs, |_| true));
    assert_eq!(ids.len(), 2_500);
    assert_each_once(&ids);
    // Page 1 reads at offset 0; every later page continues from the cursor's key.
    assert_eq!(
        reads[0],
        RunsRead::Offset {
            status: None,
            offset: 0,
            limit: PAGE as usize + 1
        }
    );
    assert_eq!(reads.len(), 125);
    assert!(reads[1..].iter().all(
        |r| matches!(r, RunsRead::After { status: None, limit, .. } if *limit == PAGE as usize + 1)
    ));
}

#[test]
fn runs_cursor_job_scope_pages_only_that_job_happy() {
    let runs = seed_runs(2_500);
    let fake = FakeCoordinator::new(runs.clone(), Some("job-b"));
    let (ids, _) = page_through(&fake, None);
    assert_eq!(
        ids,
        expected_ids(&runs, |r| r.job_id.as_deref() == Some("job-b"))
    );
    assert_each_once(&ids);
}

#[test]
fn runs_cursor_failed_filter_merges_failed_and_timeout_without_scan_happy() {
    let runs = seed_runs(2_500);
    let fake = FakeCoordinator::new(runs.clone(), None);
    let filter = status_filter("failed");
    let (ids, reads) = page_through(&fake, Some(&filter));

    assert_eq!(
        ids,
        expected_ids(&runs, |r| matches!(
            r.status,
            CoreRunStatus::Failed | CoreRunStatus::Timeout
        ))
    );
    assert_eq!(ids.len(), 1_000);
    assert_each_once(&ids);
    // Two coordinator reads per page (failed, timeout), each sized to one page.
    assert_eq!(reads.len(), 2 * 50);
    for read in &reads {
        let (status, limit) = match read {
            RunsRead::Offset { status, limit, .. } | RunsRead::After { status, limit, .. } => {
                (*status, *limit)
            }
        };
        assert!(matches!(status, Some("failed" | "timeout")), "{read:?}");
        assert_eq!(limit, PAGE as usize + 1);
    }
}

#[test]
fn runs_cursor_single_status_filter_reads_one_page_per_request_happy() {
    let runs = seed_runs(2_500);
    let fake = FakeCoordinator::new(runs.clone(), None);
    let filter = status_filter(" Completed ");
    let (ids, reads) = page_through(&fake, Some(&filter));

    assert_eq!(
        ids,
        expected_ids(&runs, |r| r.status == CoreRunStatus::Success)
    );
    assert_eq!(reads.len(), 25);
    assert!(reads.iter().all(|r| !matches!(
        r,
        RunsRead::Offset { limit, .. } if *limit == RUNS_FILTER_SCAN_CAP
    )));
    assert!(reads.iter().all(|r| matches!(
        r,
        RunsRead::Offset {
            status: Some("success"),
            ..
        } | RunsRead::After {
            status: Some("success"),
            ..
        }
    )));
}

#[test]
fn runs_cursor_filter_change_falls_back_to_offset_sad() {
    let fake = FakeCoordinator::new(seed_runs(500), None);
    let (_, unfiltered_cursor) = expect_page(fake.page(&request(0, None, None)));
    let stale = unfiltered_cursor.expect("page 1 cursor");
    fake.take_reads();

    // Same cursor, new status filter: the fingerprint no longer matches, so page 2 reads by offset.
    let (rows, _) = expect_page(fake.page(&request(
        PAGE,
        Some(status_filter("completed")),
        Some(stale.clone()),
    )));
    assert_eq!(
        fake.take_reads(),
        vec![RunsRead::Offset {
            status: Some("success"),
            offset: PAGE as usize,
            limit: PAGE as usize + 1
        }]
    );
    assert_eq!(rows.len(), PAGE as usize + 1);

    // A two-status filter can't merge offsets, so a stale cursor past page 1 scans in memory.
    let rows = fake.page(&request(PAGE, Some(status_filter("failed")), Some(stale)));
    assert!(matches!(rows, RunsPageRows::Scanned(_)));
    assert_eq!(
        fake.take_reads(),
        vec![RunsRead::Offset {
            status: None,
            offset: 0,
            limit: RUNS_FILTER_SCAN_CAP
        }]
    );
}

#[test]
fn runs_cursor_garbage_foreign_and_bad_key_cursors_fall_back_to_offset_sad() {
    let fake = FakeCoordinator::new(seed_runs(200), Some("job-a"));
    let offset_read = RunsRead::Offset {
        status: None,
        offset: PAGE as usize,
        limit: PAGE as usize + 1,
    };

    let fingerprint_other_job =
        request(PAGE, None, None).query_fingerprint(&runs_cursor_scope(Some("job-b")));
    let fingerprint_this_job =
        request(PAGE, None, None).query_fingerprint(&runs_cursor_scope(Some("job-a")));
    let key = encode_run_key(&RunKey {
        scheduled_for: base_time(),
        run_id: "run-00001".into(),
    });
    let tokens = [
        "!!!not-a-cursor".to_string(),
        PageCursor::with_key(0, key, fingerprint_other_job).encode(),
        PageCursor::with_key(0, "not-a-key", fingerprint_this_job.clone()).encode(),
        PageCursor::with_key(
            0,
            "2026-01-01T00:00:00Z|../etc",
            fingerprint_this_job.clone(),
        )
        .encode(),
        PageCursor::new(40, fingerprint_this_job).encode(),
    ];
    for token in tokens {
        expect_page(fake.page(&request(PAGE, None, Some(token.clone()))));
        assert_eq!(
            fake.take_reads(),
            vec![offset_read.clone()],
            "token {token}"
        );
    }
}

#[test]
fn runs_quick_search_and_other_filters_stay_on_offset_scan_sad() {
    let fake = FakeCoordinator::new(seed_runs(50), None);
    let quick = PageRequest::with_query(0, PAGE, None, None, Some("run-0".into()));
    let not_equals = request(
        0,
        Some(FilterQuery {
            logic: FilterLogicWire::And,
            items: vec![FilterRuleParam {
                field: "status".into(),
                operator: "not_equals".into(),
                value: DataValue::Text("failed".into()),
            }],
        }),
        None,
    );
    let unknown_status = request(0, Some(status_filter("exploded")), None);
    for req in [quick, not_equals, unknown_status] {
        let rows = fake.page(&req);
        assert!(
            matches!(rows, RunsPageRows::Scanned(ref r) if r.len() == 50),
            "{req:?}"
        );
        assert_eq!(
            fake.take_reads(),
            vec![RunsRead::Offset {
                status: None,
                offset: 0,
                limit: RUNS_FILTER_SCAN_CAP
            }]
        );
    }
}

#[test]
fn run_key_round_trips_with_nanoseconds_happy() {
    let key = RunKey {
        scheduled_for: base_time() + Duration::nanoseconds(123_456_789),
        run_id: "run|with|pipes".into(),
    };
    let encoded = encode_run_key(&key);
    assert_eq!(encoded, "2026-01-01T00:00:00.123456789Z|run|with|pipes");
    assert_eq!(decode_run_key(&encoded), Some(key));
    assert_eq!(decode_run_key("no-separator"), None);
    assert_eq!(decode_run_key("yesterday|run-1"), None);
    assert_eq!(decode_run_key("2026-01-01T00:00:00Z|"), None);
}
