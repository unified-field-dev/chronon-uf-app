use std::collections::HashMap;

use leptos::prelude::*;
use orbital::components::{Caption1, Caption2};
use orbital::primitives::{Grid, GridConfig, GridItem, Input, Label};
use turf::inline_style_sheet_values;

use crate::components::ChrononHelpSectionHeader;
use crate::server::Script;

use super::FormSectionCard;

/// Parameter editor for the currently selected script.
#[component]
pub fn ParametersSection(
    /// Two-way signal holding the selected script.
    selected_script: RwSignal<String>,
    /// List of scripts.
    scripts: Vec<Script>,
    /// Two-way signal holding the param signals.
    param_signals: RwSignal<HashMap<String, RwSignal<String>>>,
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

        .Muted {
            color: var(--colorNeutralForeground3);
        }
    };

    let selected_script_details = move || {
        let name = selected_script.get();
        scripts.iter().find(|s| s.name == name).cloned()
    };

    view! {
        <style>{style_sheet}</style>
        {move || {
            selected_script_details().map(|script| {
                if script.params.is_empty() {
                    view! {
                        <FormSectionCard>
                            <ChrononHelpSectionHeader
                                title="Parameters"
                                info=view! {
                                    <Caption1>
                                        "This script accepts no parameters. Values are passed to the script as JSON when the job runs."
                                    </Caption1>
                                }.into_any()
                            />
                            <Caption2 italic=true class=class_names.muted>
                                "This script has no parameters."
                            </Caption2>
                        </FormSectionCard>
                    }.into_any()
                } else {
                    view! {
                        <FormSectionCard>
                            <ChrononHelpSectionHeader
                                title="Parameters"
                                info=view! {
                                    <Caption1>
                                        "Values are coerced to the script parameter types and stored with the job. Required parameters must be filled before creating the job."
                                    </Caption1>
                                }.into_any()
                            />
                            <Grid config=GridConfig::with_gaps(1, 0, 16)>
                                {script.params.iter().filter_map(|param| {
                                    let param_name = param.name.clone();
                                    let param_type = param.param_type.clone();
                                    let is_required = param.required;
                                    let test_id = format!("param-{param_name}");
                                    let signal = param_signals.with(|m| m.get(&param_name).copied())?;

                                    Some(view! {
                                        <GridItem>
                                            <div class=class_names.form_field>
                                                <Label required=is_required>
                                                    {param_name.clone()} " (" {param_type} ")"
                                                </Label>
                                                <div data-testid=test_id>
                                                    <Input bind=signal />
                                                </div>
                                            </div>
                                        </GridItem>
                                    })
                                }).collect_view()}
                            </Grid>
                        </FormSectionCard>
                    }.into_any()
                }
            })
        }}
    }
}
