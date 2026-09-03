use leptos::prelude::*;
use orbital::components::{Caption1, Card, CardContent, ContentContainer, SpacingSize, Title3};
use orbital::primitives::Flex;

use crate::components::{
    chronon_card_content, chronon_table_page_layout, RunsDataTable, RunsTableChrome, RunsTableScope,
};
use crate::live::use_chronon_poll_tick;

/// Runs list page — paginated execution history with search and filters.
#[component]
pub fn ChrononRunsPage() -> impl IntoView {
    let poll_tick = use_chronon_poll_tick();
    let refresh_signal = Signal::derive(move || poll_tick.get());

    let (card_content_style, card_content_class) = chronon_card_content();
    let (page_style, page_classes) = chronon_table_page_layout();
    let fill_card_content = format!("{} {}", card_content_class, page_classes.card_content);

    view! {
        <style>{card_content_style}</style>
        <style>{page_style}</style>
        <div id="chronon-runs-page">
        <ContentContainer class=page_classes.page data_testid="chronon-runs-page">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap() class=page_classes.body>
                <Flex vertical=true gap=SpacingSize::Size80.flex_gap()>
                    <Title3>"Runs"</Title3>
                    <Caption1>
                        "Execution history for all scheduled and manual runs."
                    </Caption1>
                </Flex>
                <Card class=page_classes.card>
                    <CardContent class=fill_card_content>
                        <div id="chronon-runs-search">
                            <RunsDataTable
                                scope=RunsTableScope::All
                                chrome=RunsTableChrome {
                                    show_job_column: true,
                                    show_card_header: false,
                                    fill_height: true,
                                    infinite_scroll: false,
                                }
                                refresh_signal=refresh_signal
                            />
                        </div>
                    </CardContent>
                </Card>
            </Flex>
        </ContentContainer>
        </div>
    }
}
