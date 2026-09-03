use leptos::prelude::*;
use orbital::components::{Body1, Card, CardContent, SpacingSize, Subtitle2, Text, TextTag};
use orbital::primitives::{Flex, MessageBar, MessageBarBody, MessageBarIntent};

/// Log and error output section for a run.
///
/// Renders up to three sections: an error banner (from `error_json`),
/// stdout output, and stderr output. Shows "No output captured." when
/// all three are absent.
#[component]
pub fn RunLogsDisplay(
    /// Log output to display.
    #[prop(optional_no_strip)]
    logs: Option<String>,
    /// Captured standard error output.
    #[prop(optional_no_strip)]
    stderr: Option<String>,
    /// Optional error message.
    #[prop(optional_no_strip)]
    error_message: Option<String>,
) -> impl IntoView {
    let has_any = error_message.is_some() || logs.is_some() || stderr.is_some();
    let has_error = error_message.is_some();
    let has_logs = logs.is_some();
    let has_stderr = stderr.is_some();
    let error_msg = StoredValue::new(error_message.unwrap_or_default());
    let logs_text = StoredValue::new(logs.unwrap_or_default());
    let stderr_text = StoredValue::new(stderr.unwrap_or_default());

    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .LogsPre {
            font-family: var(--orb-type-family-mono);
            font-size: var(--orb-type-size-sm);
            line-height: var(--orb-type-line-md);
            padding: var(--orb-space-block-md) var(--orb-space-inline-md);
            margin: 0;
            background: var(--orb-color-surface-subtle);
            border-radius: var(--orb-radius-md);
            overflow-x: auto;
            white-space: pre-wrap;
            word-break: break-all;
        }
    };

    view! {
        <style>{style_sheet}</style>
        <Flex vertical=true gap=SpacingSize::Size160.flex_gap()>
            <Subtitle2>"Output"</Subtitle2>

            <Show when=move || has_error>
                <MessageBar intent=MessageBarIntent::Error>
                    <MessageBarBody>
                        <Body1>{error_msg.get_value()}</Body1>
                    </MessageBarBody>
                </MessageBar>
            </Show>

            <Show when=move || has_logs>
                <Card>
                    <CardContent>
                        <Text tag=TextTag::Pre class=class_names.logs_pre>{logs_text.get_value()}</Text>
                    </CardContent>
                </Card>
            </Show>

            <Show when=move || has_stderr>
                <Flex vertical=true gap=SpacingSize::Size80.flex_gap()>
                    <Subtitle2>"Stderr"</Subtitle2>
                    <Card>
                        <CardContent>
                            <Text tag=TextTag::Pre class=class_names.logs_pre>{stderr_text.get_value()}</Text>
                        </CardContent>
                    </Card>
                </Flex>
            </Show>

            <Show when=move || !has_any>
                <Card>
                    <CardContent>
                        <Body1>"No output captured."</Body1>
                    </CardContent>
                </Card>
            </Show>
        </Flex>
    }
}
