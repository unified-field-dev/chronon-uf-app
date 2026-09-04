use leptos::prelude::*;
use orbital::components::{Card, CardContent};
use turf::inline_style_sheet_values;

use crate::components::chronon_card_content;

/// Card surface for job-create form sections with Orbital body padding.
#[component]
pub fn FormSectionCard(
    /// Additional CSS class(es) to apply.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Child content rendered inside the component.
    children: Children,
) -> impl IntoView {
    let (card_content_style, card_content_class) = chronon_card_content();
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .FormSectionCard {
            width: 100%;
        }
    };

    let card_class = {
        let extra = class.get().unwrap_or_default();
        if extra.is_empty() {
            class_names.form_section_card.to_string()
        } else {
            format!("{} {}", class_names.form_section_card, extra)
        }
    };

    view! {
        <style>{card_content_style}</style>
        <style>{style_sheet}</style>
        <Card class=card_class>
            <CardContent class=card_content_class>
                {children()}
            </CardContent>
        </Card>
    }
}
