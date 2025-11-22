use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::content::PAGES;

#[component]
pub fn MarkdownPage() -> impl IntoView {
	let params = use_params_map();
	let html = move || {
		params
			.with(|p| p.get("slug").map(|s| s.to_string()))
			.and_then(|slug| PAGES.iter().find(|page| page.slug == slug))
			.map(|page| page.html)
			.unwrap_or("<p>Not found</p>")
	};

	view! { <article class="markdown-body" inner_html=move || html().to_string() /> }
}
