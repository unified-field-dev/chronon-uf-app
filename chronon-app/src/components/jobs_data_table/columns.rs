use std::sync::Arc;

use leptos::prelude::*;
use orbital::components::{Body1Strong, Text, TextTag};
use orbital::primitives::{ColumnType, DataTableColumnDef};
use orbital_data::DataRecord;

use crate::components::{ChrononHelpColumnHeader, JobStatusBadge};
use crate::JobStatus;

pub fn jobs_table_columns() -> Vec<DataTableColumnDef> {
    let status_view = Arc::new(|record: DataRecord| {
        let label = record
            .get("status")
            .map(orbital_data::DataValue::display_string)
            .unwrap_or_default();
        let status = match label.as_str() {
            "paused" => JobStatus::Paused,
            "disabled" => JobStatus::Disabled,
            _ => JobStatus::Active,
        };
        view! { <JobStatusBadge status=status /> }.into_any()
    });

    let name_view = Arc::new(|record: DataRecord| {
        let name = record
            .get("name")
            .map(orbital_data::DataValue::display_string)
            .unwrap_or_default();
        view! {
            <Body1Strong>{name}</Body1Strong>
        }
        .into_any()
    });

    let cron_view = Arc::new(|record: DataRecord| {
        let cron = record
            .get("cron")
            .map(orbital_data::DataValue::display_string)
            .unwrap_or_default();
        view! {
            <Text tag=TextTag::Code>{cron}</Text>
        }
        .into_any()
    });

    let schedule_header = Arc::new(|| {
        view! {
            <ChrononHelpColumnHeader
                label="Schedule"
                info=view! {
                    <orbital::components::Caption1>
                        "Cron jobs show a five-field cron expression. Manual and run-once jobs display a schedule label instead."
                    </orbital::components::Caption1>
                }.into_any()
            />
        }
        .into_any()
    });

    vec![
        DataTableColumnDef::new("name", "Name")
            .with_sortable(false)
            .with_cell_view(name_view),
        DataTableColumnDef::new("script_name", "Script").with_sortable(false),
        DataTableColumnDef::new("cron", "Schedule")
            .with_sortable(false)
            .with_header_view(schedule_header)
            .with_cell_view(cron_view),
        DataTableColumnDef::new("status", "Status")
            .with_col_type(ColumnType::SingleSelect)
            .with_sortable(false)
            .with_cell_view(status_view),
        DataTableColumnDef::new("last_run", "Last Run").with_sortable(false),
        DataTableColumnDef::new("next_run", "Next Run").with_sortable(false),
    ]
}
