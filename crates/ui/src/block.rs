use dioxus::prelude::*;
use pandoc_types::definition::Block as PandocBlock;
use parser::BlockComponent;

/// Renders raw HTML from BlockComponent
#[component]
pub fn Block(html: String) -> Element {
	rsx! {
		div { dangerous_inner_html: "{html}" }
	}
}

/// Renders BlockComponent with type awareness for customization
#[component]
pub fn TypedBlock(component: BlockComponent) -> Element {
	match &component.block {
		PandocBlock::Header(level, _, _) => rsx! {
			div {
				class: "header header-{level}",
				dangerous_inner_html: "{component.html}"
			}
		},
		PandocBlock::CodeBlock(_, _) => rsx! {
			div {
				class: "code-block",
				dangerous_inner_html: "{component.html}"
			}
		},
		PandocBlock::BlockQuote(_) => rsx! {
			div {
				class: "blockquote",
				dangerous_inner_html: "{component.html}"
			}
		},
		PandocBlock::BulletList(_) => rsx! {
			div {
				class: "bullet-list",
				dangerous_inner_html: "{component.html}"
			}
		},
		PandocBlock::OrderedList(_, _) => rsx! {
			div {
				class: "ordered-list",
				dangerous_inner_html: "{component.html}"
			}
		},
		PandocBlock::Table(_) => rsx! {
			div {
				class: "table",
				dangerous_inner_html: "{component.html}"
			}
		},
		PandocBlock::Figure(_, _, _) => rsx! {
			div {
				class: "figure",
				dangerous_inner_html: "{component.html}"
			}
		},
		PandocBlock::Para(_) => rsx! {
			div {
				class: "paragraph",
				dangerous_inner_html: "{component.html}"
			}
		},
		PandocBlock::Plain(_) => rsx! {
			div {
				class: "plain",
				dangerous_inner_html: "{component.html}"
			}
		},
		PandocBlock::LineBlock(_) => rsx! {
			div {
				class: "line-block",
				dangerous_inner_html: "{component.html}"
			}
		},
		PandocBlock::RawBlock(_, _) => rsx! {
			div {
				class: "raw-block",
				dangerous_inner_html: "{component.html}"
			}
		},
		PandocBlock::DefinitionList(_) => rsx! {
			div {
				class: "definition-list",
				dangerous_inner_html: "{component.html}"
			}
		},
		PandocBlock::HorizontalRule => rsx! {
			div {
				class: "horizontal-rule",
				dangerous_inner_html: "{component.html}"
			}
		},
		PandocBlock::Div(_, _) => rsx! {
			div {
				class: "div",
				dangerous_inner_html: "{component.html}"
			}
		},
		PandocBlock::Null => rsx! { div { class: "null" } },
	}
}

