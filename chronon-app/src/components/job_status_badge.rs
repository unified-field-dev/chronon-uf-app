use crate::JobStatus;
use leptos::prelude::*;
use orbital::primitives::{Badge, BadgeAppearance, BadgeColor};

/// Badge component for displaying job status
#[component]
pub fn JobStatusBadge(
    /// Current status value.
    #[prop(into)]
    status: JobStatus,
) -> impl IntoView {
    let (label, appearance, color) = match status {
        JobStatus::Active => ("Active", BadgeAppearance::Filled, BadgeColor::Success),
        JobStatus::Paused => ("Paused", BadgeAppearance::Tint, BadgeColor::Warning),
        JobStatus::Disabled => ("Disabled", BadgeAppearance::Outline, BadgeColor::Subtle),
    };

    view! {
        <Badge appearance=appearance color=color>{label}</Badge>
    }
}
