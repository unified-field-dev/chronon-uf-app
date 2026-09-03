use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;
use orbital::components::{
    Body1, Caption1, Card, CardContent, EmptyState, Skeleton, SkeletonItem, SkeletonItemSize,
    SpacingSize, Subtitle2,
};
use orbital::primitives::{
    Flex, MessageBar, MessageBarIntent, Table, TableBody, TableCell, TableCellLayout, TableHeader,
    TableHeaderCell, TableRow,
};

use crate::components::chronon_card_content;
use crate::components::RunStatusBadge;
use crate::server::RecentRun;
use crate::utils::{format_duration, format_started_at};

const SKELETON_ROW_COUNT: usize = 5;

/// Recent runs table with status badges and click-to-navigate.
#[component]
pub fn RecentRunsTable(
    /// Resource that loads the runs data.
    runs_res: Resource<Result<Vec<RecentRun>, ServerFnError>>,
) -> impl IntoView {
    let navigate = use_navigate();
    let nav_store = StoredValue::new(navigate);
    let (card_content_style, card_content_class) = chronon_card_content();
    let card_content_class = StoredValue::new(card_content_class);

    let runs = Memo::new(move |_| runs_res.get().and_then(Result::ok).unwrap_or_default());

    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Table { width: 100%; }
        .Row { cursor: pointer; }
        .Row:hover { background: var(--colorNeutralBackground1Hover); }
        .JobLink { color: var(--colorBrandForeground1); }
        .Duration { font-variant-numeric: tabular-nums; }
    };

    view! {
        <style>{card_content_style}</style>
        <style>{style_sheet}</style>
        <div id="chronon-dashboard-recent-runs">
        <Flex vertical=true gap=SpacingSize::Size160.flex_gap()>
            <Subtitle2>"Recent Runs"</Subtitle2>

            <Transition fallback=move || view! { <RunsTableSkeleton /> }>
                {move || runs_res.get().map(|r| match r {
                    Ok(ref list) if list.is_empty() => {
                        let content_class = card_content_class.get_value();
                        view! {
                            <Card>
                                <CardContent class=content_class>
                                    <EmptyState
                                        message="No runs yet."
                                        description="Runs appear here after jobs execute. Create a job or trigger Run Now."
                                    />
                                </CardContent>
                            </Card>
                        }.into_any()
                    }
                    Ok(_) => {
                        let content_class = card_content_class.get_value();
                        view! {
                            <Card>
                                <CardContent class=content_class>
                                    <Table class=class_names.table>
                                        <TableHeader>
                                            <TableRow>
                                                <TableHeaderCell><Caption1>"Job"</Caption1></TableHeaderCell>
                                                <TableHeaderCell><Caption1>"Status"</Caption1></TableHeaderCell>
                                                <TableHeaderCell><Caption1>"Started"</Caption1></TableHeaderCell>
                                                <TableHeaderCell><Caption1>"Duration"</Caption1></TableHeaderCell>
                                            </TableRow>
                                        </TableHeader>
                                        <TableBody>
                                            <For
                                                each=move || runs.get()
                                                key=|run| run.id.clone()
                                                let:run
                                            >
                                                {
                                                    let run_id = run.id.clone();
                                                    let nav = nav_store.get_value();
                                                    view! {
                                                        <TableRow
                                                            class=class_names.row
                                                            on:click=move |_| {
                                                                let run_id = run_id.clone();
                                                                nav(&chronon_backend::chronon_run_path(&run_id), NavigateOptions::default());
                                                            }
                                                        >
                                                            <TableCell>
                                                                <Body1 class=class_names.job_link>{run.job_name.clone()}</Body1>
                                                            </TableCell>
                                                            <TableCell>
                                                                <RunStatusBadge status=run.status />
                                                            </TableCell>
                                                            <TableCell>{format_started_at(&run.started_at)}</TableCell>
                                                            <TableCell class=class_names.duration>{format_duration(run.duration_ms)}</TableCell>
                                                        </TableRow>
                                                    }
                                                }
                                            </For>
                                        </TableBody>
                                    </Table>
                                </CardContent>
                            </Card>
                        }.into_any()
                    }
                    Err(err) => view! {
                        <MessageBar intent=MessageBarIntent::Error>
                            "Failed to load runs: " {err.to_string()}
                        </MessageBar>
                    }.into_any(),
                })}
            </Transition>
        </Flex>
        </div>
    }
}

/// Skeleton table with real column headers and placeholder rows.
#[component]
fn RunsTableSkeleton() -> impl IntoView {
    let skeleton_size = Signal::from(SkeletonItemSize::S16);
    let (card_content_style, card_content_class) = chronon_card_content();
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Table { width: 100%; }
    };

    view! {
        <style>{card_content_style}</style>
        <style>{style_sheet}</style>
        <Card>
            <CardContent class=card_content_class>
                <Table class=class_names.table>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell><Caption1>"Job"</Caption1></TableHeaderCell>
                            <TableHeaderCell><Caption1>"Status"</Caption1></TableHeaderCell>
                            <TableHeaderCell><Caption1>"Started"</Caption1></TableHeaderCell>
                            <TableHeaderCell><Caption1>"Duration"</Caption1></TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <Skeleton>
                            {(0..SKELETON_ROW_COUNT).map(|_| view! {
                                <TableRow>
                                    <TableCell>
                                        <TableCellLayout>
                                            <SkeletonItem size=skeleton_size />
                                        </TableCellLayout>
                                    </TableCell>
                                    <TableCell>
                                        <TableCellLayout>
                                            <SkeletonItem size=skeleton_size />
                                        </TableCellLayout>
                                    </TableCell>
                                    <TableCell>
                                        <TableCellLayout>
                                            <SkeletonItem size=skeleton_size />
                                        </TableCellLayout>
                                    </TableCell>
                                    <TableCell>
                                        <TableCellLayout>
                                            <SkeletonItem size=skeleton_size />
                                        </TableCellLayout>
                                    </TableCell>
                                </TableRow>
                            }).collect_view()}
                        </Skeleton>
                    </TableBody>
                </Table>
            </CardContent>
        </Card>
    }
}
