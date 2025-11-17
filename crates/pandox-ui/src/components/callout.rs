use dioxus::prelude::*;

/// Severity levels that can be rendered by the [`Callout`] component.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalloutTone {
	/// Informational note that carries no urgency.
	Note,
	/// Highlighted callout that invites attention.
	Info,
	/// Warning that needs action from the user.
	Warning,
	/// Dangerous or destructive callout.
	Danger,
}

impl CalloutTone {
	fn class(self) -> &'static str {
		match self {
			Self::Note => "note",
			Self::Info => "info",
			Self::Warning => "warning",
			Self::Danger => "danger",
		}
	}
}

/// Props for the [`Callout`] component.
#[derive(Props, PartialEq, Clone, Debug)]
pub struct CalloutProps {
	/// Severity tone that controls styling.
	pub tone: CalloutTone,
	/// Label shown in the callout header.
	#[props(into)]
	pub label: String,
	/// Child content rendered inside the callout body.
	#[props(optional)]
	pub children: Element,
}

/// Callout component for displaying messages with different styles and severity.
#[component]
pub fn Callout(
	CalloutProps {
		tone,
		label,
		children,
	}: CalloutProps,
) -> Element {
	let class = tone.class();
	rsx! {
		div {
			class: "callout callout-{class}",
			div { class: "callout-label", "{label}" }
			div { class: "callout-body", {children} }
		}
	}
}
