//! Reads one runs `DataTable` page from the coordinator, by keyset cursor where it can.
//!
//! [`read_runs_page`] picks the read for a [`PageRequest`]:
//!
//! - **Cursor path.** No quick search, and the filter is empty or a single `status` equals/is
//!   rule. The status and job scope go to the coordinator. A request whose cursor decodes reads
//!   with `list_runs_after` from the cursor's [`RunKey`]; `offset` is ignored. Without a usable
//!   cursor it reads `list_runs` at `offset`. Both return `next_cursor` from the last row when
//!   there are more rows. Page 1 always reads `list_runs`, which is cheap on every backend;
//!   later pages rely on the backend's `list_runs_after`, and a backend that keeps the
//!   trait's default body scans up to 10,000 runs per page there.
//! - **Offset path.** Quick search or any other filter reads the newest
//!   [`RUNS_FILTER_SCAN_CAP`] runs; the caller filters them in memory and slices by `offset`.
//!   A cursor-path request with no usable cursor and `offset > 0` whose status covers two
//!   coordinator statuses (UI `pending` and `failed`) also takes this path, because offsets
//!   can't be merged across two status reads.
//!
//! The cursor key is `<scheduled_for RFC 3339 with nanoseconds>|<run_id>`. The cursor
//! fingerprint covers the request's sort, filter, quick search, page size and the job scope,
//! so a cursor from another query, another job or a garbled token decodes to nothing and the
//! page is read by `offset`.

use std::future::Future;

use chrono::{DateTime, SecondsFormat, Utc};
use chronon_core::{Run as CoreRun, RunKey};
use orbital_paging::{FilterLogicWire, PageCursor, PageRequest};

use crate::validate::validate_run_id;

/// Most runs the offset path reads for quick search and filters it can't send to the coordinator.
pub const RUNS_FILTER_SCAN_CAP: usize = 5000;

/// One coordinator read [`read_runs_page`] asks the caller to run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunsRead {
    /// `list_runs(job_id, status, offset, limit)`.
    Offset {
        /// Coordinator status string (`"failed"`, `"timeout"`, …), or `None` for every status.
        status: Option<&'static str>,
        /// Rows to skip.
        offset: usize,
        /// Rows to return.
        limit: usize,
    },
    /// `list_runs_after(job_id, status, Some(&after), limit)`.
    After {
        /// Coordinator status string, or `None` for every status.
        status: Option<&'static str>,
        /// Key of the last run on the previous page.
        after: RunKey,
        /// Rows to return.
        limit: usize,
    },
}

/// Rows read for one runs page.
#[derive(Debug, Clone)]
pub enum RunsPageRows {
    /// Up to `limit + 1` runs, newest first, ready for `Page::from_oversized`.
    Page {
        /// The page's runs plus one lookahead row when more follow.
        rows: Vec<CoreRun>,
        /// Cursor for the next page, set when the lookahead row exists.
        next_cursor: Option<String>,
    },
    /// The newest [`RUNS_FILTER_SCAN_CAP`] runs, to filter in memory and slice by `offset`.
    Scanned(Vec<CoreRun>),
}

/// Cursor fingerprint scope for the runs table, all runs or one job's runs.
#[must_use]
pub fn runs_cursor_scope(job_id: Option<&str>) -> String {
    job_id.map_or_else(
        || "chronon.runs".to_string(),
        |id| format!("chronon.runs.job:{id}"),
    )
}

/// Coordinator statuses for a request the cursor path can serve.
///
/// `Some(vec![])` means no status filter; `None` means the request needs the offset path.
#[must_use]
pub fn runs_pushdown_statuses(request: &PageRequest) -> Option<Vec<&'static str>> {
    if request
        .quick_search
        .as_ref()
        .is_some_and(|quick| !quick.trim().is_empty())
    {
        return None;
    }
    let Some(filter) = request.filter.as_ref() else {
        return Some(Vec::new());
    };
    match filter.items.as_slice() {
        [] if filter.logic == FilterLogicWire::And => Some(Vec::new()),
        [rule] if rule.field == "status" && matches!(rule.operator.as_str(), "equals" | "is") => {
            let value = rule.value.display_string().trim().to_lowercase();
            let statuses: &[&'static str] = match value.as_str() {
                "pending" => &["queued", "claimed"],
                "running" => &["running"],
                "completed" => &["success"],
                "failed" => &["failed", "timeout"],
                "cancelled" => &["canceled"],
                _ => return None,
            };
            Some(statuses.to_vec())
        }
        _ => None,
    }
}

/// Encodes a run key as `<scheduled_for>|<run_id>`.
#[must_use]
pub fn encode_run_key(key: &RunKey) -> String {
    format!(
        "{}|{}",
        key.scheduled_for
            .to_rfc3339_opts(SecondsFormat::Nanos, true),
        key.run_id
    )
}

/// Parses a key written by [`encode_run_key`]; `None` for anything else.
#[must_use]
pub fn decode_run_key(key: &str) -> Option<RunKey> {
    let (scheduled_for, run_id) = key.split_once('|')?;
    let scheduled_for = DateTime::parse_from_rfc3339(scheduled_for)
        .ok()?
        .with_timezone(&Utc);
    validate_run_id(run_id).ok()?;
    Some(RunKey {
        scheduled_for,
        run_id: run_id.to_string(),
    })
}

/// Reads one runs page with the caller's coordinator reads.
///
/// `read` runs each [`RunsRead`] against the coordinator for the page's job scope (`job_id`
/// must be the same job the caller filters by). The module docs list which requests take the
/// cursor path.
pub async fn read_runs_page<F, Fut, E>(
    request: &PageRequest,
    job_id: Option<&str>,
    mut read: F,
) -> Result<RunsPageRows, E>
where
    F: FnMut(RunsRead) -> Fut,
    Fut: Future<Output = Result<Vec<CoreRun>, E>>,
{
    let Some(statuses) = runs_pushdown_statuses(request) else {
        return scan(&mut read).await;
    };
    let scope = runs_cursor_scope(job_id);
    let after = PageCursor::from_request(request, &scope)
        .and_then(|cursor| cursor.key)
        .and_then(|key| decode_run_key(&key));
    let offset = request.offset as usize;
    if after.is_none() && offset > 0 && statuses.len() > 1 {
        return scan(&mut read).await;
    }

    let fetch = request.limit as usize + 1;
    let status_reads: Vec<Option<&'static str>> = if statuses.is_empty() {
        vec![None]
    } else {
        statuses.into_iter().map(Some).collect()
    };
    let mut rows: Vec<CoreRun> = Vec::new();
    for status in status_reads {
        let one = after.as_ref().map_or(
            RunsRead::Offset {
                status,
                offset,
                limit: fetch,
            },
            |after| RunsRead::After {
                status,
                after: after.clone(),
                limit: fetch,
            },
        );
        rows.extend(read(one).await?);
    }
    rows.sort_by(|a, b| {
        b.scheduled_for
            .cmp(&a.scheduled_for)
            .then_with(|| b.run_id.cmp(&a.run_id))
    });
    rows.truncate(fetch);

    // The last row shown is `rows[limit - 1]`; the row after it only proves more exist.
    let next_cursor = if rows.len() == fetch && request.limit > 0 {
        rows.get(request.limit as usize - 1).map(|last| {
            let fingerprint = request.query_fingerprint(&scope);
            PageCursor::with_key(0, encode_run_key(&RunKey::from(last)), fingerprint).encode()
        })
    } else {
        None
    };
    Ok(RunsPageRows::Page { rows, next_cursor })
}

async fn scan<F, Fut, E>(read: &mut F) -> Result<RunsPageRows, E>
where
    F: FnMut(RunsRead) -> Fut,
    Fut: Future<Output = Result<Vec<CoreRun>, E>>,
{
    let rows = read(RunsRead::Offset {
        status: None,
        offset: 0,
        limit: RUNS_FILTER_SCAN_CAP,
    })
    .await?;
    Ok(RunsPageRows::Scanned(rows))
}
