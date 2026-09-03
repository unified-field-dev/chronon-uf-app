mod charts;
mod recent_runs_table;
mod run_trend_card;
mod stats_grid;

pub use recent_runs_table::RecentRunsTable;
pub use run_trend_card::RunTrendCard;
pub use stats_grid::ChrononStatsGrid;

use leptos::prelude::*;
use orbital::components::{ContentContainer, SpacingSize, Title3};
use orbital::primitives::Flex;

use crate::live::use_chronon_poll_tick;
use crate::server::{get_dashboard_stats, get_recent_runs};

/// Dashboard page -- shows stats, run trend chart, and recent runs.
#[component]
pub fn ChrononDashboardPage() -> impl IntoView {
    let poll_tick = use_chronon_poll_tick();
    let stats_res = Resource::new(|| (), |()| async move { get_dashboard_stats().await });
    let runs_res = Resource::new(|| (), |()| async move { get_recent_runs(10).await });

    Effect::new(move |_| {
        if poll_tick.get() > 0 {
            stats_res.refetch();
            runs_res.refetch();
        }
    });

    view! {
        <ContentContainer data_testid="chronon-dashboard">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
                <Title3>"Chronon Dashboard"</Title3>
                <ChrononStatsGrid stats_res=stats_res />
                <RunTrendCard />
                <RecentRunsTable runs_res=runs_res />
            </Flex>
        </ContentContainer>
    }
}
