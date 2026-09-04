use leptos::prelude::*;
use orbital::components::Subtitle2;
use orbital::primitives::{InfoLabel, InfoLabelInfo};

/// Section title with optional domain help popover.
#[component]
pub fn ChrononHelpSectionHeader(
    /// Title text.
    title: &'static str,
    /// Supplementary info/help content.
    #[prop(optional)]
    info: Option<AnyView>,
) -> impl IntoView {
    view! {
        {if let Some(info_view) = info {
            view! {
                <InfoLabel>
                    <Subtitle2 block=true>{title}</Subtitle2>
                    <InfoLabelInfo slot>
                        {info_view}
                    </InfoLabelInfo>
                </InfoLabel>
            }.into_any()
        } else {
            view! { <Subtitle2 block=true>{title}</Subtitle2> }.into_any()
        }}
    }
}
