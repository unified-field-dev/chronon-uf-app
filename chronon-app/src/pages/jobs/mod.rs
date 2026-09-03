use leptos::prelude::*;
use leptos_router::components::A;
use orbital::components::{Card, CardContent, ContentContainer, SpacingSize, Title3};
use orbital::primitives::{Button, ButtonAppearance, Flex, FlexAlign, FlexJustify, Icon};

use crate::components::{chronon_card_content, chronon_table_page_layout, JobsDataTable};

/// Jobs list page
#[component]
pub fn ChrononJobsPage() -> impl IntoView {
    let (card_content_style, card_content_class) = chronon_card_content();
    let (page_style, page_classes) = chronon_table_page_layout();
    let fill_card_content = format!("{} {}", card_content_class, page_classes.card_content);

    view! {
        <style>{card_content_style}</style>
        <style>{page_style}</style>
        <div id="chronon-jobs-page">
        <ContentContainer class=page_classes.page data_testid="chronon-jobs-page">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap() class=page_classes.body>
                <Flex justify=FlexJustify::SpaceBetween align=FlexAlign::Center>
                    <Title3>"Jobs"</Title3>
                    <div id="chronon-jobs-create-button" data-testid="jobs-create-button">
                        <A href=crate::paths::JOBS_NEW>
                            <Button appearance=ButtonAppearance::Primary>
                                <Icon icon=icondata::AiPlusOutlined />
                                "Create Job"
                            </Button>
                        </A>
                    </div>
                </Flex>

                <Card class=page_classes.card>
                    <CardContent class=fill_card_content>
                        <div id="chronon-jobs-search">
                            <JobsDataTable />
                        </div>
                    </CardContent>
                </Card>
            </Flex>
        </ContentContainer>
        </div>
    }
}
