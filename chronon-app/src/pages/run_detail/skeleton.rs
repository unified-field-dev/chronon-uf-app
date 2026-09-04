use leptos::prelude::*;
use orbital::components::{
    Caption1, Caption2, Card, CardContent, Skeleton, SkeletonItem, SkeletonItemSize, SpacingSize,
    Subtitle2,
};
use orbital::primitives::{Flex, FlexAlign, Grid, GridConfig, GridItem};

/// Structured loading shell mirroring the run detail page layout.
#[component]
pub fn RunDetailPageSkeleton() -> impl IntoView {
    let value_skeleton = Signal::from(SkeletonItemSize::S16);
    let title_skeleton = Signal::from(SkeletonItemSize::S32);
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Label { color: var(--colorNeutralForeground3); }
        .TitleSkeleton { width: 20rem; }
        .BadgeSkeleton { width: 5rem; }
        .JobSkeleton { width: 8rem; }
        .LogsSkeleton { width: 100%; height: 120px; }
        .Table { width: 100%; }
    };

    view! {
        <style>{style_sheet}</style>
        <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
            <Flex vertical=true gap=SpacingSize::Size80.flex_gap()>
                <Flex align=FlexAlign::Center gap=SpacingSize::Size120.flex_gap()>
                    <Caption1 class=class_names.label>"Run"</Caption1>
                    <Skeleton>
                        <SkeletonItem class=class_names.title_skeleton size=title_skeleton />
                    </Skeleton>
                    <Skeleton>
                        <SkeletonItem class=class_names.badge_skeleton size=value_skeleton />
                    </Skeleton>
                </Flex>
                <Flex align=FlexAlign::Center gap=SpacingSize::Size40.flex_gap()>
                    <Caption1>"Job:"</Caption1>
                    <Skeleton>
                        <SkeletonItem class=class_names.job_skeleton size=value_skeleton />
                    </Skeleton>
                </Flex>
            </Flex>

            <Card>
                <CardContent>
                    <Grid config=GridConfig::with_gaps(2, 16, 8)>
                        {["Started", "Finished", "Duration"].into_iter().map(|label| view! {
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
                <Subtitle2>"Output"</Subtitle2>
                <Card>
                    <CardContent>
                        <Skeleton>
                            <SkeletonItem class=class_names.logs_skeleton />
                        </Skeleton>
                    </CardContent>
                </Card>
            </Flex>
        </Flex>
    }
}
