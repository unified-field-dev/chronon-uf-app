use leptos::prelude::*;
use orbital::components::{FormHint, SpacingSize};
use orbital::primitives::{
    Button, ButtonAppearance, Dialog, DialogActions, DialogBody, DialogContent, DialogSurface,
    DialogTitle, Field, Flex, MessageBar, MessageBarIntent, Textarea, TextareaAppearance,
    TextareaResize,
};
use orbital_motion::{MotionCurve, OrbitalPresence, PresenceMotion};

#[derive(Clone, Copy)]
pub struct RunNowDialogInput {
    pub open: RwSignal<bool>,
    pub params_str: RwSignal<String>,
    pub loading: RwSignal<bool>,
    pub error: RwSignal<Option<String>>,
    pub on_submit: Callback<()>,
    pub on_cancel: Callback<()>,
}

#[component]
pub fn RunNowDialog(
    /// Bundled props for the component.
    props: RunNowDialogInput,
) -> impl IntoView {
    let RunNowDialogInput {
        open,
        params_str,
        loading,
        error,
        on_submit,
        on_cancel,
    } = props;

    let params_textarea_appearance = TextareaAppearance {
        placeholder: MaybeProp::from("{}"),
        resize: Signal::from(TextareaResize::Vertical),
        ..Default::default()
    };

    let open_signal = open.read_only();
    let panel_motion =
        Signal::from(PresenceMotion::fade_scale().with_curve(MotionCurve::DecelerateMid));

    view! {
        <Dialog open=open>
            <OrbitalPresence appear=true show=open_signal motion=panel_motion>
                <DialogSurface>
                <DialogBody>
                    <DialogTitle>"Run Job Now"</DialogTitle>
                    <DialogContent>
                        <div data-testid="run-now-dialog">
                            <Flex vertical=true gap=SpacingSize::Size160.flex_gap()>
                                <Field label="Parameters (JSON)">
                                    <Textarea bind=params_str appearance=params_textarea_appearance />
                                </Field>
                                <FormHint>
                                    "Defaults are prefilled. Update values to override for this run only."
                                </FormHint>
                                <Show when=move || error.get().is_some() fallback=|| ()>
                                    <MessageBar intent=MessageBarIntent::Error>
                                        {move || error.get().unwrap_or_default()}
                                    </MessageBar>
                                </Show>
                            </Flex>
                        </div>
                    </DialogContent>
                    <DialogActions>
                        <Button
                            appearance=ButtonAppearance::Secondary
                            disabled=Signal::derive(move || loading.get())
                            on_click=Callback::new(move |_| on_cancel.run(()))
                        >
                            "Cancel"
                        </Button>
                        <Button
                            appearance=ButtonAppearance::Primary
                            disabled=Signal::derive(move || loading.get())
                            on_click=Callback::new(move |_| on_submit.run(()))
                        >
                            {move || if loading.get() { "Running..." } else { "Run Now" }}
                        </Button>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
            </OrbitalPresence>
        </Dialog>
    }
}
