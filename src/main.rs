//! Client entrypoint for the CSR build.

// Bin target reuses lib deps, silence noisy lint.
#![allow(unused_crate_dependencies)]

use bezel::{init_logging, App};
use leptos::prelude::*;

fn main() {
	init_logging();

	// Remove loading placeholder before mounting
	if let Some(window) = web_sys::window() {
		if let Some(document) = window.document() {
			if let Some(loading) = document.get_element_by_id("loading") {
				loading.remove();
			}
		}
	}

	mount_to_body(|| {
		view! { <App /> }
	})
}
