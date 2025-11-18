use dioxus::prelude::*;

const BLOG_CSS: Asset = asset!("/assets/blog.css");

#[component]
pub fn Blog() -> Element {
	rsx! {
		document::Link { rel: "stylesheet", href: BLOG_CSS}

		div {
			id: "blog",

			h1 { "Blog" }
			p { "Blog content goes here." }
		}
	}
}
