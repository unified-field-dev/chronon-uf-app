use leptos::prelude::*;
use orbital::components::{Caption1, FormHint};
use orbital::primitives::{
    Grid, GridConfig, GridItem, InfoLabel, InfoLabelInfo, Input, InputAppearance, Label,
    OptionBind, Radio, RadioGroup, RadioGroupBind,
};
use turf::inline_style_sheet_values;

use crate::components::ChrononHelpSectionHeader;
use crate::pages::job_create::ScheduleType;

use super::FormSectionCard;

/// Schedule-type selector and schedule-specific inputs.
#[allow(clippy::too_many_lines)]
#[component]
pub fn ScheduleSection(
    /// Two-way signal holding the schedule type.
    schedule_type: RwSignal<ScheduleType>,
    /// Two-way signal holding the cron expression.
    cron_expression: RwSignal<String>,
    /// Two-way signal holding the timezone identifier.
    timezone: RwSignal<String>,
    /// Two-way signal holding the run once datetime.
    run_once_datetime: RwSignal<String>,
) -> impl IntoView {
    let schedule_option = RwSignal::new(Some(
        match schedule_type.get_untracked() {
            ScheduleType::Cron => "cron",
            ScheduleType::RunOnce => "run_once",
            ScheduleType::Manual => "manual",
        }
        .to_string(),
    ));

    Effect::new(move || {
        let val = schedule_option.get().unwrap_or_else(|| "cron".to_string());
        let st = match val.as_str() {
            "run_once" => ScheduleType::RunOnce,
            "manual" => ScheduleType::Manual,
            _ => ScheduleType::Cron,
        };
        schedule_type.set(st);
    });

    let (style_sheet, class_names) = inline_style_sheet_values! {
        .SectionTitle {
            margin-bottom: 16px;
        }

        .FormField {
            display: flex;
            flex-direction: column;
            gap: 6px;
        }
    };

    let cron_description = move || {
        let expr = cron_expression.get();
        match expr.as_str() {
            "* * * * *" => "Every minute".to_string(),
            "0 * * * *" => "Every hour, at minute 0".to_string(),
            "0 0 * * *" => "Every day at midnight".to_string(),
            "0 3 * * *" => "Every day at 03:00 AM".to_string(),
            "0 9 * * 1" => "Every Monday at 09:00 AM".to_string(),
            "*/5 * * * *" => "Every 5 minutes".to_string(),
            "*/15 * * * *" => "Every 15 minutes".to_string(),
            "0 */2 * * *" => "Every 2 hours".to_string(),
            _ => format!("Custom: {expr}"),
        }
    };

    view! {
            <style>{style_sheet}</style>
            <FormSectionCard>
                <ChrononHelpSectionHeader
                    title="Schedule"
                    info=view! {
                        <Caption1>
                            "Choose how often the job runs. Cron uses a five-field expression (minute hour day month weekday). Timezone applies to cron schedules only."
                        </Caption1>
                    }.into_any()
                />
                <Grid config=GridConfig::with_gaps(1, 0, 16)>
                    <GridItem>
                        <div class=class_names.form_field>
                            <Label required=true>"Type"</Label>
                            <RadioGroup bind=RadioGroupBind {
                                value: OptionBind::Signal(schedule_option),
                                name: "schedule_type".into(),
                                ..Default::default()
                            }>
                                <Radio value="cron" label="Cron" />
                                <Radio value="run_once" label="Run Once" />
                                <Radio value="manual" label="Manual" />
                            </RadioGroup>
                        </div>
                    </GridItem>

                    {move || match schedule_type.get() {
                        ScheduleType::Cron => view! {
                            <GridItem>
                                <div class=class_names.form_field>
                                    <InfoLabel>
                                        <Label required=true>"Cron Expression"</Label>
                                        <InfoLabelInfo slot>
                                            <Caption1>
                                                "Five fields: minute, hour, day of month, month, day of week. Example: 0 9 * * 1 runs every Monday at 09:00."
                                            </Caption1>
                                        </InfoLabelInfo>
                                    </InfoLabel>
                                    <div data-testid="cron-expr">
                                        <Input appearance=InputAppearance::with_placeholder("0 * * * *")
                                            bind=cron_expression
    />
                                    </div>
                                    <FormHint>
                                        {cron_description}
                                    </FormHint>
                                </div>
                            </GridItem>

                            <GridItem>
                                <div class=class_names.form_field>
                                    <InfoLabel>
                                        <Label>"Timezone"</Label>
                                        <InfoLabelInfo slot>
                                            <Caption1>
                                                "IANA timezone for cron evaluation (for example UTC or America/Los_Angeles). Affects when the next run is calculated."
                                            </Caption1>
                                        </InfoLabelInfo>
                                    </InfoLabel>
                                    <Input appearance=InputAppearance::with_placeholder("UTC")
                                        bind=timezone
    />
                                </div>
                            </GridItem>
                        }.into_any(),
                        ScheduleType::RunOnce => view! {
                            <GridItem>
                                <div class=class_names.form_field>
                                    <Label required=true>"Run At"</Label>
                                    <div data-testid="run-once-datetime">
                                        <Input appearance=InputAppearance::with_placeholder("2026-01-25T03:00:00")
                                            bind=run_once_datetime
    />
                                    </div>
                                    <FormHint>
                                        "Enter datetime in ISO 8601 format"
                                    </FormHint>
                                </div>
                            </GridItem>
                        }.into_any(),
                        ScheduleType::Manual => view! {
                            <GridItem>
                                <FormHint>
                                    "This job will only run when triggered manually via 'Run Now'."
                                </FormHint>
                            </GridItem>
                        }.into_any(),
                    }}
                </Grid>
            </FormSectionCard>
        }
}
