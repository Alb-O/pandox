use dioxus::prelude::*;
use pandox_macros::mdfile;

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
	mdfile!("test/index.md")
}
