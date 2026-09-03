use std::sync::Arc;

use leptos::prelude::*;
use orbital::primitives::{ColumnType, DataTableColumnDef};
use orbital_data::DataRecord;

use crate::components::RunStatusBadge;
use crate::RunStatus;

pub fn runs_table_columns(show_job_column: bool) -> Vec<DataTableColumnDef> {
    let status_view = Arc::new(|record: DataRecord| {
        let label = record
            .get("status")
            .map(orbital_data::DataValue::display_string)
            .unwrap_or_default();
        let status = match label.as_str() {
            "running" => RunStatus::Running,
            "completed" => RunStatus::Completed,
            "failed" => RunStatus::Failed,
            "cancelled" => RunStatus::Cancelled,
            _ => RunStatus::Pending,
        };
        view! { <RunStatusBadge status=status /> }.into_any()
    });

    let mut cols = Vec::new();
    if show_job_column {
        cols.push(DataTableColumnDef::new("job_name", "Job").with_sortable(false));
    }
    cols.push(
        DataTableColumnDef::new("status", "Status")
            .with_col_type(ColumnType::SingleSelect)
            .with_sortable(false)
            .with_cell_view(status_view),
    );
    cols.push(DataTableColumnDef::new("started_at", "Started").with_sortable(false));
    cols.push(DataTableColumnDef::new("duration_ms", "Duration").with_sortable(false));
    cols
}
