use crate::components::{RunsDataTable, RunsTableChrome, RunsTableScope};
use crate::server::JOB_RUNS_PAGE_SIZE;
use leptos::prelude::*;

/// Recent runs section using the shared runs DataTable.
#[component]
pub fn JobRecentRuns(
    /// The job ID to fetch runs for
    job_id: String,
    /// Bumped when live run events arrive for this job (refreshes table without remount).
    refresh_signal: Signal<u32>,
) -> impl IntoView {
    view! {
        <RunsDataTable
            scope=RunsTableScope::ForJob(job_id)
            chrome=RunsTableChrome {
                show_job_column: false,
                show_card_header: true,
                fill_height: false,
                infinite_scroll: true,
            }
            max_height=400.0
            page_size=JOB_RUNS_PAGE_SIZE
            refresh_signal=refresh_signal
        />
    }
}
