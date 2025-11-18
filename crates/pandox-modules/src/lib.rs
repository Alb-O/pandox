//!! Pandox Modules Library

use dioxus::prelude::*;
use pandox_macros::mdfile;

/// A test component to verify markdown module functionality
#[component]
pub fn Test() -> Element {
	rsx! {
		div {
			class: "test-content",
			TestContent {}
		}
	}
}

/// Test component content
#[component]
pub fn TestContent() -> Element {
	mdfile!("test/index.md")
}
