use turf::inline_style_sheet_values;

/// Card body padding for Chronon table/card sections.
///
/// Orbital `CardContent` defaults to `0 16px 16px` (no top inset). `DataTable` toolbars and
/// infinite-scroll regions need uniform inset inside the card surface.
pub fn chronon_card_content() -> (&'static str, String) {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .ChrononCardContent {
            --orbital-card-content-padding: var(--orb-space-block-md, var(--spacingVerticalM, 12px))
                var(--orb-space-inline-md, var(--spacingHorizontalM, 16px))
                var(--orb-space-block-md, var(--spacingVerticalM, 16px));
        }
    };
    (style_sheet, class_names.chronon_card_content.to_string())
}

/// Layout classes for list pages whose `DataTable` should fill the viewport.
pub struct ChrononTablePageClasses {
    pub page: String,
    pub body: String,
    pub card: String,
    pub card_content: String,
}

/// Flex column layout so `DataTables` can fill remaining viewport below the app chrome.
pub fn chronon_table_page_layout() -> (&'static str, ChrononTablePageClasses) {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .FillPage {
            display: flex;
            flex-direction: column;
            min-height: calc(100vh - 11rem);
            min-height: calc(100dvh - 11rem);
        }
        .FillBody {
            flex: 1;
            min-height: 0;
            display: flex;
            flex-direction: column;
        }
        .FillCard {
            flex: 1;
            min-height: 0;
            display: flex;
            flex-direction: column;
        }
        .FillCardContent {
            flex: 1;
            min-height: 0;
            display: flex;
            flex-direction: column;
        }
    };
    (
        style_sheet,
        ChrononTablePageClasses {
            page: class_names.fill_page.to_string(),
            body: class_names.fill_body.to_string(),
            card: class_names.fill_card.to_string(),
            card_content: class_names.fill_card_content.to_string(),
        },
    )
}
