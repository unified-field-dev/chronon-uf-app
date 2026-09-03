use leptos::prelude::*;
use orbital::primitives::{InfoLabel, InfoLabelInfo};

/// Table column header with an optional info popover.
#[component]
pub fn ChrononHelpColumnHeader(
    /// Label text.
    label: &'static str,
    /// Supplementary info/help content.
    #[prop(optional)]
    info: Option<AnyView>,
) -> impl IntoView {
    view! {
        {if let Some(info_view) = info {
            view! {
                <InfoLabel>
                    {label}
                    <InfoLabelInfo slot>
                        {info_view}
                    </InfoLabelInfo>
                </InfoLabel>
            }.into_any()
        } else {
            view! { {label} }.into_any()
        }}
    }
}
