//! Script registry server functions.

use leptos::prelude::*;
use orbital_paging::{Page, PageRequest};

#[cfg(feature = "ssr")]
use super::helpers::parse_script_params;
#[cfg(feature = "ssr")]
use super::query::apply_scripts_page_query;
#[cfg(feature = "ssr")]
use super::ssr_utils::require_session;
use super::Script;

/// Get all registered scripts
#[uf_product_macros::server]
pub async fn get_scripts() -> Result<Vec<Script>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    let registry = super::ssr_utils::script_registry()?;

    let scripts: Vec<Script> = registry
        .list()
        .into_iter()
        .map(|d| Script {
            name: d.name.to_string(),
            signature: d.signature_hash.to_string(),
            params: parse_script_params(d.signature_json),
            description: None,
        })
        .collect();

    Ok(scripts)
}

/// Paginated registered scripts with quick-search and structured filters.
#[uf_product_macros::server]
pub async fn get_scripts_page(
    /// `DataTable` paging/filter/search/sort request from the client.
    request: PageRequest,
) -> Result<Page<Script>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    let registry = super::ssr_utils::script_registry()?;

    let mut all_scripts: Vec<Script> = registry
        .list()
        .into_iter()
        .map(|d| Script {
            name: d.name.to_string(),
            signature: d.signature_hash.to_string(),
            params: parse_script_params(d.signature_json),
            description: None,
        })
        .collect();
    chronon_backend::sort_scripts_by_name(&mut all_scripts);
    apply_scripts_page_query(&mut all_scripts, &request);

    let total_count = if request.is_first_page() {
        Some(all_scripts.len() as u64)
    } else {
        None
    };

    let page_slice: Vec<Script> = all_scripts
        .into_iter()
        .skip(request.offset as usize)
        .take((request.limit + 1) as usize)
        .collect();

    Ok(Page::from_oversized(page_slice, request.limit, total_count))
}
