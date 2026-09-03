use leptos::ev;
use leptos::prelude::*;
use orbital::components::SpacingSize;
use orbital::primitives::{
    Button, ButtonAppearance, DiscussionAdapter, Flex, FlexJustify, SchedulerDataSource,
};
use turf::inline_style_sheet_values;

/// Footer action row for the job creation form.
#[component]
pub fn ActionsSection(
    /// Callback invoked when the action is cancelled.
    on_cancel: Callback<ev::MouseEvent>,
    /// Callback invoked when the item should be created.
    on_create: Callback<ev::MouseEvent>,
    /// Two-way signal controlling whether create loading is enabled.
    create_loading: RwSignal<bool>,
) -> impl IntoView {
    // Top rule separates actions from the form; Flex has no divider prop.
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Actions {
            padding-top: var(--spacingVerticalL, 24px);
            border-top: 1px solid var(--colorNeutralStroke1);
        }
    };

    view! {
        <style>{style_sheet}</style>
        <Flex
            justify=FlexJustify::FlexEnd
            gap=SpacingSize::Size120.flex_gap()
            class=class_names.actions
        >
            <div id="chronon-job-create-cancel">
            <Button
                appearance=ButtonAppearance::Secondary
                disabled=Signal::derive(move || create_loading.get())
                on_click=Callback::new(move |ev| on_cancel.run(ev))
            >
                "Cancel"
            </Button>
            </div>
            <div id="chronon-job-create-submit" data-testid="create-job">
                <Button
                    appearance=ButtonAppearance::Primary
                    disabled=Signal::derive(move || create_loading.get())
                    on_click=Callback::new(move |ev| on_create.run(ev))
                >
                    {move || if create_loading.get() { "Creating…" } else { "Create Job" }}
                </Button>
            </div>
        </Flex>
    }
}
