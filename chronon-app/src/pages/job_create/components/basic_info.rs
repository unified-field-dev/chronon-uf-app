use leptos::prelude::*;
use orbital::components::Subtitle2;
use orbital::primitives::{
    Grid, GridConfig, GridItem, Input, InputAppearance, Label, MessageBar, MessageBarIntent,
    Select, Skeleton, SkeletonItem,
};
use turf::inline_style_sheet_values;

use crate::server::Script;

use super::FormSectionCard;

/// Skeleton placeholder for the script select while scripts load.
#[component]
fn ScriptSelectSkeleton() -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .SelectSkeleton { width: 100%; height: 32px; }
    };

    view! {
        <style>{style_sheet}</style>
        <Skeleton>
            <SkeletonItem class=class_names.select_skeleton />
        </Skeleton>
    }
}

/// Basic job metadata fields (name + script selection).
#[component]
pub fn BasicInfoSection(
    /// Two-way signal holding the job name.
    job_name: RwSignal<String>,
    /// Two-way signal holding the selected script.
    selected_script: RwSignal<String>,
    /// Resource that loads the scripts data.
    scripts_res: Resource<Result<Vec<Script>, ServerFnError>>,
) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .SectionTitle {
            margin-bottom: 16px;
        }

        .FormField {
            display: flex;
            flex-direction: column;
            gap: 6px;
        }

        .ScriptSignature {
            font-family: monospace;
            font-size: 12px;
            color: var(--colorNeutralForeground3);
            background: var(--colorNeutralBackground3);
            padding: 8px 12px;
            border-radius: 4px;
            margin-top: 8px;
        }
    };

    view! {
        <style>{style_sheet}</style>
        <FormSectionCard>
            <Subtitle2 block=true class=class_names.section_title>"Basic Information"</Subtitle2>
            <Grid config=GridConfig::with_gaps(1, 0, 16)>
                <GridItem>
                    <div class=class_names.form_field>
                        <Label required=true>"Job Name"</Label>
                        <div data-testid="job-name">
                            <Input appearance=InputAppearance::with_placeholder("e.g., daily-cleanup")
                                bind=job_name
                            />
                        </div>
                    </div>
                </GridItem>

                <GridItem>
                    <div class=class_names.form_field>
                        <Label required=true>"Script"</Label>
                        <Transition fallback=move || view! { <ScriptSelectSkeleton /> }>
                            {move || scripts_res.get().map(|r| match r {
                                Ok(scripts) => {
                                    let scripts_for_sig = scripts.clone();
                                    let selected_script_details = move || {
                                        let name = selected_script.get();
                                        scripts_for_sig.iter().find(|s| s.name == name).cloned()
                                    };
                                    view! {
                                        <>
                                            <Select bind=selected_script>
                                                <option value="">"Select a script..."</option>
                                                {scripts.iter().map(|script| {
                                                    let name = script.name.clone();
                                                    let display = format!(
                                                        "{} ({})",
                                                        script.name,
                                                        script.params.len()
                                                    );
                                                    view! {
                                                        <option value=name>{display}</option>
                                                    }
                                                }).collect_view()}
                                            </Select>

                                            {move || selected_script_details().map(|script| {
                                                view! {
                                                    <div class=class_names.script_signature>
                                                        {format!("{}(valence: Valence{})",
                                                            script.name,
                                                            if script.params.is_empty() {
                                                                String::new()
                                                            } else {
                                                                format!(", {}", script.params.iter()
                                                                    .map(|p| format!("{}: {}", p.name, p.param_type))
                                                                    .collect::<Vec<_>>()
                                                                    .join(", "))
                                                            }
                                                        )}
                                                    </div>
                                                }
                                            })}
                                        </>
                                    }.into_any()
                                }
                                Err(err) => view! {
                                    <MessageBar intent=MessageBarIntent::Error>
                                        "Failed to load scripts: " {err.to_string()}
                                    </MessageBar>
                                }.into_any(),
                            })}
                        </Transition>
                    </div>
                </GridItem>
            </Grid>
        </FormSectionCard>
    }
}
