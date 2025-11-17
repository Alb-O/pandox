use dioxus::prelude::*;
use pandox_macros::markdown_component;

#[component]
pub fn Test() -> Element {
	rsx! {
		div {
			class: "test-content",
			TestContent {}
		}
	}
}

#[component]
pub fn TestContent() -> Element {
	markdown_component!("../../public/test/index.md", slug = "test")
}
