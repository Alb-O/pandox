use dioxus::prelude::*;

include!(concat!(env!("OUT_DIR"), "/test_content.rs"));

#[component]
pub fn Test() -> Element {
	rsx! {
		div {
			class: "test-content",
			TestContent {}
		}
	}
}
