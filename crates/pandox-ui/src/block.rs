use dioxus::prelude::*;
use pandoc_types::definition::Block as PandocBlock;
use pandox_markdown::BlockComponent;

/// Signature for a block renderer in the manifest.
pub type BlockRenderer = fn(&BlockComponent) -> Option<Element>;

/// Built-in manifest of block renderers.
pub const BLOCK_MANIFEST: &[BlockRenderer] = &[render_css_class];

/// Renders raw HTML from BlockComponent
#[component]
pub fn Block(html: String) -> Element {
	rsx! {
		div { dangerous_inner_html: "{html}" }
	}
}

/// Renders BlockComponent with type awareness for customization.
#[component]
pub fn TypedBlock(component: BlockComponent) -> Element {
	render_from_manifest(&component, BLOCK_MANIFEST).unwrap_or_else(|| default_render(&component))
}

/// Render a block component using a custom manifest.
pub fn render_from_manifest(
	component: &BlockComponent,
	manifest: &[BlockRenderer],
) -> Option<Element> {
	manifest.iter().find_map(|renderer| renderer(component))
}

fn render_css_class(component: &BlockComponent) -> Option<Element> {
	block_class(component).map(|class| render_with_class(component, class))
}

fn default_render(component: &BlockComponent) -> Element {
	render_with_class(component, "block")
}

fn render_with_class(component: &BlockComponent, class: impl Into<String>) -> Element {
	let html = component.html.clone();
	let class = class.into();
	rsx! {
		div {
			class: "{class}",
			dangerous_inner_html: "{html}"
		}
	}
}

/// Returns the CSS class for the given block, if any.
pub fn block_class(component: &BlockComponent) -> Option<String> {
	match &component.block {
		PandocBlock::Header(level, _, _) => Some(format!("header header-{level}")),
		PandocBlock::CodeBlock(_, _) => Some("code-block".into()),
		PandocBlock::BlockQuote(_) => Some("blockquote".into()),
		PandocBlock::BulletList(_) => Some("bullet-list".into()),
		PandocBlock::OrderedList(_, _) => Some("ordered-list".into()),
		PandocBlock::Table(_) => Some("table".into()),
		PandocBlock::Figure(_, _, _) => Some("figure".into()),
		PandocBlock::Para(_) => Some("paragraph".into()),
		PandocBlock::Plain(_) => Some("plain".into()),
		PandocBlock::LineBlock(_) => Some("line-block".into()),
		PandocBlock::RawBlock(_, _) => Some("raw-block".into()),
		PandocBlock::DefinitionList(_) => Some("definition-list".into()),
		PandocBlock::HorizontalRule => Some("horizontal-rule".into()),
		PandocBlock::Div(_, _) => Some("div".into()),
		PandocBlock::Null => Some("null".into()),
	}
}
