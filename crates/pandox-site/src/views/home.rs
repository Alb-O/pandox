use dioxus::prelude::*;
use pandox_components::Hero;

#[component]
pub fn Home() -> Element {
	rsx! {
		Hero {}

	}
}
