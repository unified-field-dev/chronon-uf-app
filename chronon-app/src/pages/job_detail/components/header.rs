use leptos::ev;
use leptos::prelude::*;
use orbital::components::{
    Body1, Caption1, Caption2, Card, CardHeader, SkeletonItem, SpacingSize, Title3,
};
use orbital::primitives::{
    Button, ButtonAppearance, Flex, FlexAlign, FlexJustify, Icon, InfoLabel, InfoLabelInfo,
    MessageBar, MessageBarBody, MessageBarIntent, Select, SelectAppearance, Switch,
};
use turf::inline_style_sheet_values;

use crate::components::JobStatusBadge;
use crate::server::{JobRevision, JobStatus};

/// Consolidated input for `JobDetailHeader`, reducing 20+ individual props
/// into a single struct the orchestrator builds and passes.
#[derive(Clone, Copy)]
pub struct JobDetailHeaderInput {
    /// Display name of the job (driven by revision snapshot or form)
    pub display_job_name: Memo<String>,
    /// Job status for the badge
    pub job_status: JobStatus,
    /// Resource providing revision list for the selector
    pub revisions_res: Resource<Result<Vec<JobRevision>, ServerFnError>>,
    /// The latest revision number (computed from revisions list)
    pub latest_revision_num: Memo<u32>,
    /// Currently selected revision (two-way bound via Select)
    pub revision_str: RwSignal<String>,
    /// Whether the user is viewing an older (non-current) revision
    pub is_viewing_old_revision: Memo<bool>,
    /// Whether the page is in edit mode
    pub is_editing: RwSignal<bool>,
    /// Enabled toggle signal
    pub enabled_signal: RwSignal<bool>,
    /// Run-now button loading state
    pub run_now_loading: RwSignal<bool>,
    /// Run-now error message, if any
    pub run_now_error: RwSignal<Option<String>>,
    /// Whether the Run Now button should be disabled
    pub run_now_disabled: Memo<bool>,
    /// Callback when user clicks Run Now
    pub on_run_now: Callback<ev::MouseEvent>,
    /// Save button loading state
    pub save_loading: RwSignal<bool>,
    /// Save error message, if any
    pub save_error: RwSignal<Option<String>>,
    /// Callback when user clicks Save
    pub on_save: Callback<ev::MouseEvent>,
    /// Callback when user clicks Cancel
    pub on_cancel: Callback<ev::MouseEvent>,
    /// Callback when user clicks Edit
    pub on_edit: Callback<ev::MouseEvent>,
}

/// Header section for the job detail page.
#[allow(clippy::too_many_lines)]
// `props` is a Leptos component prop; the `#[component]` macro requires it be
// taken by value (it's captured into `'static` reactive closures), so it can't
// be passed by reference despite its size.
#[allow(clippy::large_types_passed_by_value)]
#[component]
pub fn JobDetailHeader(
    /// Bundled props for the component.
    props: JobDetailHeaderInput,
) -> impl IntoView {
    let JobDetailHeaderInput {
        display_job_name,
        job_status,
        revisions_res,
        latest_revision_num,
        revision_str,
        is_viewing_old_revision,
        is_editing,
        enabled_signal,
        run_now_loading,
        run_now_error,
        run_now_disabled,
        on_run_now,
        save_loading,
        save_error,
        on_save,
        on_cancel,
        on_edit,
    } = props;

    let (style_sheet, class_names) = inline_style_sheet_values! {
        .JobDetailCard {
            width: 100%;
            max-width: 100%;
            margin: 0 0 24px 0;
            box-sizing: border-box;
        }

        .Content {
            padding: 0 16px 16px 16px;
        }

        .RevisionLabel {
            color: var(--orb-color-text-tertiary);
        }

        .RevisionSelect {
            min-width: 120px;
        }

        .RevisionSelectSkeleton {
            min-width: 120px;
            height: 32px;
        }

        .EnabledToggleDisabled {
            opacity: 0.5;
            pointer-events: none;
        }

        .EnabledLabel {
            color: var(--colorNeutralForeground2);
        }
    };

    view! {
        <style>{style_sheet}</style>

        <Card class=class_names.job_detail_card>
            <CardHeader>
                <div id="chronon-job-detail-header">
                <Flex align=FlexAlign::Center gap=SpacingSize::Size120.flex_gap()>
                    <Title3>{move || display_job_name.get()}</Title3>
                    <JobStatusBadge status=job_status />
                </Flex>
                </div>
            </CardHeader>

            <Flex vertical=true gap=SpacingSize::Size160.flex_gap() class=class_names.content>
                <Suspense fallback=move || view! {
                    <SkeletonItem class=class_names.revision_select_skeleton />
                }>
                    {move || {
                        match revisions_res.get() {
                            Some(Ok(revisions)) => {
                                let revision_options: Vec<u32> = revisions.iter()
                                    .map(|r| r.revision_number)
                                    .collect();

                                view! {
                                    <Flex align=FlexAlign::Center gap=SpacingSize::Size80.flex_gap()>
                                        <InfoLabel>
                                            <Caption2 class=class_names.revision_label>"Revision"</Caption2>
                                            <InfoLabelInfo slot>
                                                <Caption1>
                                                    "Each save creates a new revision. Older revisions are read-only snapshots of prior configuration."
                                                </Caption1>
                                            </InfoLabelInfo>
                                        </InfoLabel>
                                        <div id="chronon-job-detail-revision" data-testid="revision-select">
                                            <Select
                                                bind=revision_str
                                                class=class_names.revision_select
                                                appearance=SelectAppearance {
                                                    disabled: Signal::derive(move || is_editing.get()),
                                                    ..Default::default()
                                                }
                                            >
                                                {move || {
                                                    let latest = latest_revision_num.get();
                                                    revision_options.iter().map(|&rev| {
                                                        let is_current = rev == latest;
                                        let label = if is_current {
                                            format!("{rev} (current)")
                                        } else {
                                                            rev.to_string()
                                                        };
                                                        view! {
                                                            <option value=rev.to_string()>{label}</option>
                                                        }
                                                    }).collect_view()
                                                }}
                                            </Select>
                                        </div>
                                    </Flex>
                                }.into_any()
                            }
                            _ => view! {
                                <SkeletonItem class=class_names.revision_select_skeleton />
                            }.into_any(),
                        }
                    }}
                </Suspense>

                <Show when=move || is_viewing_old_revision.get() fallback=|| ()>
                    <MessageBar intent=MessageBarIntent::Warning>
                        <MessageBarBody>
                            <Flex align=FlexAlign::Center gap=SpacingSize::Size80.flex_gap()>
                                <Icon icon=icondata::AiWarningOutlined />
                                "You are viewing an older revision. This version cannot be edited."
                            </Flex>
                        </MessageBarBody>
                    </MessageBar>
                </Show>

                <Flex justify=FlexJustify::SpaceBetween align=FlexAlign::Center>
                    <div class=move || {
                        if is_viewing_old_revision.get() {
                            class_names.enabled_toggle_disabled.to_string()
                        } else {
                            String::new()
                        }
                    }>
                        <Flex align=FlexAlign::Center gap=SpacingSize::Size80.flex_gap()>
                            <div id="chronon-job-detail-enabled">
                            <InfoLabel>
                                <Body1 class=class_names.enabled_label>"Enabled"</Body1>
                                <InfoLabelInfo slot>
                                    <Caption1>
                                        "When enabled, the scheduler runs this job on its schedule. Paused or disabled jobs do not run automatically."
                                    </Caption1>
                                </InfoLabelInfo>
                            </InfoLabel>
                            <Switch bind=enabled_signal />
                            </div>
                        </Flex>
                    </div>

                    <Flex align=FlexAlign::Center gap=SpacingSize::Size120.flex_gap()>
                        <div id="chronon-job-detail-run-now" data-testid="run-now-button">
                            <Button
                                appearance=ButtonAppearance::Secondary
                                on_click=Callback::new(move |e| on_run_now.run(e))
                                disabled=run_now_disabled
                            >
                                <Icon icon=icondata::AiPlayCircleOutlined />
                                {move || if run_now_loading.get() { "Running..." } else { "Run Now" }}
                            </Button>
                        </div>

                        <Show when=move || run_now_error.get().is_some() fallback=|| ()>
                            <MessageBar intent=MessageBarIntent::Error>
                                {move || format!("Failed to run job: {}", run_now_error.get().unwrap_or_default())}
                            </MessageBar>
                        </Show>

                        <Show when=move || is_editing.get() fallback=|| ()>
                            <Flex vertical=true gap=SpacingSize::Size120.flex_gap()>
                                <Show when=move || save_error.get().is_some() fallback=|| ()>
                                    <MessageBar intent=MessageBarIntent::Error>
                                        "Failed to save: " {move || save_error.get().unwrap_or_default()}
                                    </MessageBar>
                                </Show>
                                <Flex gap=SpacingSize::Size120.flex_gap()>
                                    <div id="chronon-job-detail-save" data-testid="save-job-button">
                                        <Button
                                            appearance=ButtonAppearance::Primary
                                            disabled=save_loading
                                            on_click=Callback::new(move |e| on_save.run(e))
                                        >
                                            {move || if save_loading.get() { "Saving..." } else { "Save" }}
                                        </Button>
                                    </div>
                                    <div id="chronon-job-detail-cancel" data-testid="cancel-edit-button">
                                        <Button
                                            appearance=ButtonAppearance::Secondary
                                            disabled=save_loading
                                            on_click=Callback::new(move |e| on_cancel.run(e))
                                        >
                                            "Cancel"
                                        </Button>
                                    </div>
                                </Flex>
                            </Flex>
                        </Show>

                        <Show when=move || !is_editing.get() fallback=|| ()>
                            <div id="chronon-job-detail-edit" data-testid="edit-job-button">
                                <Button
                                    appearance=ButtonAppearance::Subtle
                                    disabled=is_viewing_old_revision
                                    on_click=Callback::new(move |e| on_edit.run(e))
                                >
                                    <Icon icon=icondata::AiEditOutlined />
                                    "Edit"
                                </Button>
                            </div>
                        </Show>
                    </Flex>
                </Flex>
            </Flex>
        </Card>
    }
}
