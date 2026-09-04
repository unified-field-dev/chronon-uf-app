//! Scripts list page — registered Chronon scripts in a searchable `DataTable`.

mod scripts_data_table;

pub use scripts_data_table::ScriptsDataTable;

use leptos::prelude::*;
use orbital::components::{Caption1, Card, CardContent, ContentContainer, SpacingSize, Title3};
use orbital::primitives::Flex;

use crate::components::{chronon_card_content, chronon_table_page_layout};

/// Registered scripts list with search and parameter signatures.
#[component]
pub fn ChrononScriptsPage() -> impl IntoView {
    let (card_content_style, card_content_class) = chronon_card_content();
    let (page_style, page_classes) = chronon_table_page_layout();
    let fill_card_content = format!("{} {}", card_content_class, page_classes.card_content);

    view! {
        <style>{card_content_style}</style>
        <style>{page_style}</style>
        <div id="chronon-scripts-page">
        <ContentContainer class=page_classes.page data_testid="chronon-scripts-page">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap() class=page_classes.body>
                <Flex vertical=true gap=SpacingSize::Size80.flex_gap()>
                    <Title3>"Registered Scripts"</Title3>
                    <Caption1>
                        "Scripts registered via #[chronon_coordinator_macros::script] appear here before you bind them to scheduled jobs."
                    </Caption1>
                </Flex>
                <Card class=page_classes.card>
                    <CardContent class=fill_card_content>
                        <div id="chronon-scripts-search">
                            <ScriptsDataTable fill_height=true />
                        </div>
                    </CardContent>
                </Card>
            </Flex>
        </ContentContainer>
        </div>
    }
}
