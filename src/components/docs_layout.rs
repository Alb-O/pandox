//! Layout wrapper for documentation pages with sidebar.

use leptos::prelude::*;

use crate::components::sidebar::Sidebar;

/// Layout component that wraps documentation content with a sidebar.
#[component]
pub fn DocsLayout(children: Children) -> impl IntoView {
	view! {
		<div class="docs-layout">
			<Sidebar />
			<main class="docs-content">{children()}</main>
		</div>
	}
}
