use leptos::prelude::*;
use orbital::components::{Body1, Caption2, Card, CardContent};
use orbital::primitives::{Grid, GridConfig, GridItem};

/// Metadata grid showing run timing details (started, finished, duration, parent run).
#[component]
pub fn RunInfoGrid(
    /// Start timestamp.
    started: String,
    /// Finish timestamp.
    finished: String,
    /// Duration to display.
    duration: String,
    /// Optional parent run ID.
    #[prop(optional_no_strip)]
    parent_run_id: Option<String>,
) -> impl IntoView {
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Label { color: var(--colorNeutralForeground3); }
    };

    view! {
        <style>{style_sheet}</style>
        <Card>
            <CardContent>
                <Grid config=GridConfig::with_gaps(2, 16, 8)>
                    <GridItem><Caption2 class=class_names.label>"Started"</Caption2></GridItem>
                    <GridItem><Body1>{started}</Body1></GridItem>

                    <GridItem><Caption2 class=class_names.label>"Finished"</Caption2></GridItem>
                    <GridItem><Body1>{finished}</Body1></GridItem>

                    <GridItem><Caption2 class=class_names.label>"Duration"</Caption2></GridItem>
                    <GridItem><Body1>{duration}</Body1></GridItem>

                    {parent_run_id.map(|pid| view! {
                        <GridItem><Caption2 class=class_names.label>"Parent Run"</Caption2></GridItem>
                        <GridItem><Body1>{pid}</Body1></GridItem>
                    })}
                </Grid>
            </CardContent>
        </Card>
    }
}
