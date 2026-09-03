//! Help spotlight tour inventory for Chronon ops routes.
//!
//! Each step is a `#[help_spotlight_step]` component registered into `uf-help`
//! inventory at link time. Call [`ensure_help_steps_linked`] from the host or
//! [`crate::ChrononRoutes`] so those submissions survive linking.
//!
//! | Route pattern | Module |
//! |---------------|--------|
//! | `/chronon` | `dashboard` |
//! | `/chronon/jobs` | `jobs` |
//! | `/chronon/jobs/new` | `job_create` |
//! | `/chronon/jobs/:job_id` | `job_detail` |
//! | `/chronon/runs` | `runs` |
//! | `/chronon/runs/:run_id` | `run_detail` |
//! | `/chronon/scripts` | `scripts` |
//!
//! Exact inventory keys win over `:param` siblings (for example `/chronon/jobs/new`
//! does not merge with `/chronon/jobs/:job_id`).
//!
//! ```rust,ignore
//! use chronon_app::ensure_help_steps_linked;
//!
//! ensure_help_steps_linked();
//! ```

mod dashboard;
mod job_create;
mod job_detail;
mod jobs;
mod run_detail;
mod runs;
mod scripts;

use leptos::prelude::*;
use orbital::components::{Body1, Caption1, SpacingSize};
use orbital::primitives::Flex;

/// Shared step body: lead paragraph, optional detail, optional legend lines.
pub(crate) fn help_stack(
    testid: &'static str,
    lead: &'static str,
    detail: Option<&'static str>,
    legend: &'static [&'static str],
) -> impl IntoView {
    view! {
        <div data-testid=testid>
            <Flex vertical=true gap=SpacingSize::Size80.flex_gap()>
                <Body1>{lead}</Body1>
                {detail.map(|d| view! { <Caption1>{d}</Caption1> })}
                {legend
                    .iter()
                    .copied()
                    .map(|line| view! { <Caption1>{line}</Caption1> })
                    .collect_view()}
            </Flex>
        </div>
    }
}

/// Force-link Chronon Help spotlight inventory into the host binary.
///
/// Empty body; `#[help_spotlight_step]` submissions in child modules are retained
/// when this crate is linked and this function is called from routes or the host.
pub const fn ensure_help_steps_linked() {}
