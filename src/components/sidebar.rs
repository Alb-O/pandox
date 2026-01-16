//! Sidebar component for documentation navigation.

use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

use crate::navigation::get_navigation;

/// Sidebar component displaying categorized navigation links.
#[component]
pub fn Sidebar() -> impl IntoView {
	let location = use_location();
	let categories = get_navigation();

	view! {
		<nav class="sidebar">
			<div class="sidebar-header">
				<A href="/" attr:class="sidebar-brand">
					"Docs"
				</A>
			</div>
			<ul class="sidebar-nav">
				{categories
					.into_iter()
					.map(|category| {
						view! {
							<li class="sidebar-category">
								<span class="category-title">{category.name}</span>
								<ul class="category-pages">
									{category
										.pages
										.into_iter()
										.map(|page| {
											let href = format!("/docs/{}", page.slug);
											let href_clone = href.clone();
											let is_active = move || location.pathname.get() == href_clone;
											view! {
												<li class:active=is_active>
													<A href=href>{page.title}</A>
												</li>
											}
										})
										.collect_view()}
								</ul>
							</li>
						}
					})
					.collect_view()}
			</ul>
		</nav>
	}
}
