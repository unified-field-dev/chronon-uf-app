mod kpis;
mod skeleton;

use leptos::prelude::*;
use orbital::components::{SpacingSize, StatCard};
use orbital::primitives::{
    DiscussionAdapter, Flex, FlexWrap, MessageBar, MessageBarIntent, SchedulerDataSource,
};

use kpis::{dashboard_stat_memos, KPI_DEFS};
use skeleton::ChrononStatsSkeleton;

/// Stats cards row showing job and run counts.
#[component]
pub fn ChrononStatsGrid(
    /// Resource that loads the stats data.
    stats_res: Resource<Result<crate::server::DashboardStats, ServerFnError>>,
) -> impl IntoView {
    let memos = StoredValue::new(dashboard_stat_memos(stats_res));

    view! {
        <Transition fallback=move || view! { <ChrononStatsSkeleton /> }>
            {move || stats_res.get().map(|r| match r {
                Ok(_) => view! {
                    <div id="chronon-dashboard-stats">
                    <Flex gap=SpacingSize::Size160.flex_gap() wrap=FlexWrap::Wrap>
                        {KPI_DEFS.iter().enumerate().map(|(i, def)| {
                            let value = Signal::derive(move || {
                                memos.with_value(|m| m[i].get())
                            });
                            let label = def.label;
                            let variant = def.variant;
                            let test_id = def.test_id;
                            view! {
                                <div data-testid=test_id>
                                    {def.icon.map_or_else(|| view! {
                                        <StatCard
                                            label=label
                                            value=value
                                            variant=variant
                                        />
                                    }.into_any(), |icon| view! {
                                        <StatCard
                                            label=label
                                            value=value
                                            icon=icon
                                            variant=variant
                                        />
                                    }.into_any())}
                                </div>
                            }
                        }).collect_view()}
                    </Flex>
                    </div>
                }.into_any(),
                Err(e) => view! {
                    <MessageBar intent=MessageBarIntent::Error>
                        "Failed to load stats: " {e.to_string()}
                    </MessageBar>
                }.into_any(),
            })}
        </Transition>
    }
}
