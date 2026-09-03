use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;
use orbital::components::{Caption1, SpacingSize, Title3};
use orbital::primitives::{Button, ButtonAppearance, ButtonSize, Flex, FlexAlign};

use crate::components::RunStatusBadge;
use crate::server::Run;

use super::{RunInfoGrid, RunLogsDisplay};

/// Loaded run detail body — values derived from `run_res` so refetch updates in place.
#[allow(clippy::too_many_lines)]
#[component]
pub fn RunDetailContent(
    /// Resource that loads the run data.
    run_res: Resource<Result<Option<Run>, ServerFnError>>,
) -> impl IntoView {
    let navigate = use_navigate();
    let nav_store = StoredValue::new(navigate);

    let run_id_display = Memo::new(move |_| {
        run_res
            .get()
            .and_then(|r| r.ok().flatten())
            .map(|r| r.id)
            .unwrap_or_default()
    });
    let job_id = Memo::new(move |_| {
        run_res
            .get()
            .and_then(|r| r.ok().flatten())
            .map(|r| r.job_id)
            .unwrap_or_default()
    });
    let job_name = Memo::new(move |_| {
        run_res
            .get()
            .and_then(|r| r.ok().flatten())
            .map(|r| r.job_name)
            .unwrap_or_default()
    });
    let status = Memo::new(move |_| {
        run_res
            .get()
            .and_then(|r| r.ok().flatten())
            .map(|r| r.status)
    });
    let started = Memo::new(move |_| {
        run_res
            .get()
            .and_then(|r| r.ok().flatten())
            .map(|r| r.started_at)
            .unwrap_or_default()
    });
    let finished = Memo::new(move |_| {
        run_res
            .get()
            .and_then(|r| r.ok().flatten())
            .and_then(|r| r.finished_at)
            .unwrap_or_else(|| "\u{2014}".to_string())
    });
    let duration = Memo::new(move |_| {
        run_res
            .get()
            .and_then(|r| r.ok().flatten())
            .and_then(|r| r.duration_ms.map(|d| format!("{d}ms")))
            .unwrap_or_else(|| "\u{2014}".to_string())
    });
    let logs = Memo::new(move |_| {
        run_res
            .get()
            .and_then(|r| r.ok().flatten())
            .and_then(|r| r.logs)
    });
    let stderr = Memo::new(move |_| {
        run_res
            .get()
            .and_then(|r| r.ok().flatten())
            .and_then(|r| r.stderr)
    });
    let error_message = Memo::new(move |_| {
        run_res
            .get()
            .and_then(|r| r.ok().flatten())
            .and_then(|r| r.error_message)
    });
    let parent_run_id = Memo::new(move |_| {
        run_res
            .get()
            .and_then(|r| r.ok().flatten())
            .and_then(|r| r.parent_run_id)
    });

    view! {
        <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
            <div id="chronon-run-detail-header">
            <Flex vertical=true gap=SpacingSize::Size80.flex_gap()>
                <Flex align=FlexAlign::Center gap=SpacingSize::Size120.flex_gap()>
                    <Title3>"Run " {move || run_id_display.get()}</Title3>
                    {move || status.get().map(|s| view! { <RunStatusBadge status=s /> })}
                </Flex>
                <Flex align=FlexAlign::Center gap=SpacingSize::Size40.flex_gap()>
                    <Caption1>"Job:"</Caption1>
                    <div id="chronon-run-detail-job-link" data-testid="run-job-link">
                        <Button
                            appearance=ButtonAppearance::Transparent
                            size=ButtonSize::Small
                            on_click=Callback::new({
                                let nav = nav_store.get_value();
                                move |_| {
                                    let id = job_id.get();
                                    if !id.is_empty() {
                                        nav(&chronon_backend::chronon_job_path(&id), NavigateOptions::default());
                                    }
                                }
                            })
                        >
                            {move || job_name.get()}
                        </Button>
                    </div>
                </Flex>
            </Flex>
            </div>

            <div id="chronon-run-detail-timing">
            {move || view! {
                <RunInfoGrid
                    started=started.get()
                    finished=finished.get()
                    duration=duration.get()
                    parent_run_id=parent_run_id.get()
                />
            }}
            </div>

            <div id="chronon-run-detail-output">
            {move || view! {
                <RunLogsDisplay
                    logs=logs.get()
                    stderr=stderr.get()
                    error_message=error_message.get()
                />
            }}
            </div>
        </Flex>
    }
}
