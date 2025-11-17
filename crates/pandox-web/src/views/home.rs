use dioxus::prelude::*;
use pandox_ui::Hero;

#[component]
pub fn Home() -> Element {
	rsx! {
		Hero {}

	}
}
