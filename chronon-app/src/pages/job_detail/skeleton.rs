use leptos::prelude::*;
use orbital::components::{
    Caption1, Caption2, Card, CardContent, CardHeader, CardSectionBorder, Skeleton, SkeletonItem,
    SkeletonItemSize, SpacingSize, Subtitle2,
};
use orbital::primitives::{
    Flex, FlexAlign, Grid, GridConfig, GridItem, Table, TableBody, TableCell, TableCellLayout,
    TableHeader, TableHeaderCell, TableRow,
};
use turf::inline_style_sheet_values;

use crate::components::chronon_card_content;

const SKELETON_ROW_COUNT: usize = 4;

/// Structured loading shell mirroring the job detail page layout.
#[component]
#[allow(clippy::too_many_lines)]
pub fn JobDetailPageSkeleton() -> impl IntoView {
    let value_skeleton = Signal::from(SkeletonItemSize::S16);
    let (card_content_style, card_content_class) = chronon_card_content();
    let card_content_class_2 = card_content_class.clone();
    let card_content_class_3 = card_content_class.clone();
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .JobDetailCard {
            width: 100%;
            max-width: 100%;
            margin: 0 0 24px 0;
            box-sizing: border-box;
        }
        .Label { color: var(--colorNeutralForeground3); }
        .Table { width: 100%; }
    };

    let config_labels = [
        "Script",
        "Schedule",
        "Timezone",
        "Parameters",
        "Last Run",
        "Next Run",
    ];

    view! {
        <style>{card_content_style}</style>
        <style>{style_sheet}</style>
        <div data-testid="chronon-job-detail-skeleton">
        <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
            <Card class=class_names.job_detail_card>
                <CardHeader>
                    <Flex align=FlexAlign::Center gap=SpacingSize::Size120.flex_gap()>
                        <Skeleton>
                            <SkeletonItem width="12rem".to_string() height="32px".to_string() />
                        </Skeleton>
                    </Flex>
                </CardHeader>
                <CardContent class=card_content_class_3.clone()>
                    <Flex align=FlexAlign::Center gap=SpacingSize::Size80.flex_gap()>
                        <Caption2 class=class_names.label>"Revision"</Caption2>
                        <Skeleton>
                            <SkeletonItem width="120px".to_string() height="32px".to_string() />
                        </Skeleton>
                    </Flex>
                    <Flex gap=SpacingSize::Size120.flex_gap()>
                        {(0..2).map(|_| view! {
                            <Skeleton>
                                <SkeletonItem width="5rem".to_string() height="16px".to_string() />
                            </Skeleton>
                        }).collect_view()}
                    </Flex>
                </CardContent>
            </Card>

            <Card class=class_names.job_detail_card>
                <CardHeader>
                    <Subtitle2>"Configuration"</Subtitle2>
                </CardHeader>
                <CardSectionBorder />
                <CardContent class=card_content_class_2>
                    <Grid config=GridConfig::with_gaps(2, 16, 8)>
                        {config_labels.into_iter().map(|label| view! {
                            <>
                                <GridItem><Caption2 class=class_names.label>{label}</Caption2></GridItem>
                                <GridItem>
                                    <Skeleton>
                                        <SkeletonItem size=value_skeleton />
                                    </Skeleton>
                                </GridItem>
                            </>
                        }).collect_view()}
                    </Grid>
                </CardContent>
            </Card>

            <Flex vertical=true gap=SpacingSize::Size160.flex_gap()>
                <Subtitle2>"Recent Runs"</Subtitle2>
                <Card>
                    <CardContent class=card_content_class>
                        <Table class=class_names.table>
                            <TableHeader>
                                <TableRow>
                                    <TableHeaderCell><Caption1>"Status"</Caption1></TableHeaderCell>
                                    <TableHeaderCell><Caption1>"Started"</Caption1></TableHeaderCell>
                                    <TableHeaderCell><Caption1>"Duration"</Caption1></TableHeaderCell>
                                </TableRow>
                            </TableHeader>
                            <TableBody>
                                <Skeleton>
                                    {(0..SKELETON_ROW_COUNT).map(|_| view! {
                                        <TableRow>
                                            <TableCell>
                                                <TableCellLayout>
                                                    <SkeletonItem size=value_skeleton />
                                                </TableCellLayout>
                                            </TableCell>
                                            <TableCell>
                                                <TableCellLayout>
                                                    <SkeletonItem size=value_skeleton />
                                                </TableCellLayout>
                                            </TableCell>
                                            <TableCell>
                                                <TableCellLayout>
                                                    <SkeletonItem size=value_skeleton />
                                                </TableCellLayout>
                                            </TableCell>
                                        </TableRow>
                                    }).collect_view()}
                                </Skeleton>
                            </TableBody>
                        </Table>
                    </CardContent>
                </Card>
            </Flex>
        </Flex>
        </div>
    }
}
