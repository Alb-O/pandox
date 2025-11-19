//!! The main entry point for the Pandox web application.

use dioxus::prelude::*;
use pandox_components::Navbar;
use pandox_modules::Test;
use views::{Blog, Home};
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
	#[layout(WebNavbar)]
	#[route("/")]
	Home {},
	#[route("/blog")]
	Blog {},
	#[route("/test")]
	Test {}
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
// const TAILWIND: Option<Asset> = option_asset!("/assets/tailwind.compiled.css");

/// Launch the Pandox site app
pub fn main() {
	dioxus::launch(App);
}

#[component]
fn App() -> Element {
	// Build cool things ✌️

	rsx! {
		// Global app resources
		document::Link { rel: "icon", href: FAVICON }
		document::Link { rel: "stylesheet", href: MAIN_CSS }
		// if let Some(tailwind) = TAILWIND {
		// 	document::Link { rel: "stylesheet", href: tailwind }
		// }

		Router::<Route> {}
	}
}

/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
fn WebNavbar() -> Element {
	rsx! {
		Navbar {
			Link {
				to: Route::Home {},
				"Home"
			}
			Link {
				to: Route::Blog {},
				"Blog"
			}
			Link {
				to: Route::Test {},
				"Test Module"
			}
		}

		Outlet::<Route> {}
	}
}
