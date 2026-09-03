use orbital::primitives::DataTableColumnDef;

pub fn scripts_table_columns() -> Vec<DataTableColumnDef> {
    vec![
        DataTableColumnDef::new("name", "Script").with_sortable(false),
        DataTableColumnDef::new("signature", "Signature").with_sortable(false),
        DataTableColumnDef::new("description", "Description").with_sortable(false),
        DataTableColumnDef::new("params_summary", "Parameters").with_sortable(false),
    ]
}
