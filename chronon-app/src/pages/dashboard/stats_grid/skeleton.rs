use leptos::prelude::*;
use orbital::components::{Caption1, Skeleton, SkeletonItem, SkeletonItemSize, SpacingSize};
use orbital::primitives::{
    Card, DiscussionAdapter, Flex, FlexAlign, FlexWrap, Icon, SchedulerDataSource,
};

use super::kpis::KPI_DEFS;

/// Stat card shell with a skeleton placeholder for the value only.
#[component]
pub fn ChrononStatCardSkeleton(
    /// Label text.
    label: &'static str,
    /// Icon to display.
    icon: Option<icondata_core::Icon>,
) -> impl IntoView {
    let value_skeleton = Signal::from(SkeletonItemSize::S32);
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Card { min-width: 140px; flex: 1; }
        .Label { color: var(--orb-color-text-tertiary); }
        .ValueSkeleton { width: 4rem; }
    };

    view! {
        <style>{style_sheet}</style>
        <Card class=class_names.card>
            <Flex vertical=true gap=SpacingSize::Size80.flex_gap() padding=SpacingSize::Size160.inset()>
                <Flex align=FlexAlign::Center gap=SpacingSize::Size80.flex_gap()>
                    {icon.map(|icon| view! { <Icon icon=icon /> })}
                    <Caption1 class=class_names.label>{label}</Caption1>
                </Flex>
                <Skeleton>
                    <SkeletonItem class=class_names.value_skeleton size=value_skeleton />
                </Skeleton>
            </Flex>
        </Card>
    }
}

/// Skeleton row matching the seven dashboard KPI cards.
#[component]
pub fn ChrononStatsSkeleton() -> impl IntoView {
    view! {
        <Flex gap=SpacingSize::Size160.flex_gap() wrap=FlexWrap::Wrap>
            {KPI_DEFS.iter().map(|def| {
                view! {
                    <div data-testid=def.test_id>
                        <ChrononStatCardSkeleton label=def.label icon=def.icon />
                    </div>
                }
            }).collect_view()}
        </Flex>
    }
}
