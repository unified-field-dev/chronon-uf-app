use leptos::prelude::*;
use orbital::components::StatCardVariant;

use crate::server::DashboardStats;

/// Static definition for one dashboard KPI card.
pub struct DashboardKpiDef {
    pub label: &'static str,
    pub icon: Option<icondata_core::Icon>,
    pub variant: StatCardVariant,
    pub test_id: &'static str,
    pub select: fn(&DashboardStats) -> u64,
}

pub const KPI_DEFS: &[DashboardKpiDef] = &[
    DashboardKpiDef {
        label: "Total Jobs",
        icon: Some(icondata::AiScheduleOutlined),
        variant: StatCardVariant::Default,
        test_id: "chronon-stat-total-jobs",
        select: |s| u64::from(s.total_jobs),
    },
    DashboardKpiDef {
        label: "Active",
        icon: Some(icondata::AiCheckCircleOutlined),
        variant: StatCardVariant::Success,
        test_id: "chronon-stat-active",
        select: |s| u64::from(s.active_jobs),
    },
    DashboardKpiDef {
        label: "Paused",
        icon: Some(icondata::AiPauseCircleOutlined),
        variant: StatCardVariant::Warning,
        test_id: "chronon-stat-paused",
        select: |s| u64::from(s.paused_jobs),
    },
    DashboardKpiDef {
        label: "Runs Today",
        icon: Some(icondata::AiHistoryOutlined),
        variant: StatCardVariant::Default,
        test_id: "chronon-stat-runs-today",
        select: |s| u64::from(s.total_runs_today),
    },
    DashboardKpiDef {
        label: "Successful",
        icon: None,
        variant: StatCardVariant::Success,
        test_id: "chronon-stat-successful",
        select: |s| u64::from(s.successful_runs_today),
    },
    DashboardKpiDef {
        label: "Failed",
        icon: None,
        variant: StatCardVariant::Danger,
        test_id: "chronon-stat-failed",
        select: |s| u64::from(s.failed_runs_today),
    },
    DashboardKpiDef {
        label: "Running Now",
        icon: Some(icondata::AiLoadingOutlined),
        variant: StatCardVariant::Default,
        test_id: "chronon-stat-running-now",
        select: |s| u64::from(s.running_now),
    },
];

/// Memoized string values for each KPI, updating in place on resource refetch.
pub fn dashboard_stat_memos(
    stats_res: Resource<Result<DashboardStats, ServerFnError>>,
) -> Vec<Memo<String>> {
    KPI_DEFS
        .iter()
        .map(|def| {
            let select = def.select;
            Memo::new(move |_| {
                stats_res
                    .get()
                    .and_then(Result::ok)
                    .map(|s| select(&s).to_string())
                    .unwrap_or_default()
            })
        })
        .collect()
}
