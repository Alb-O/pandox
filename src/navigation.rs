//! Navigation data structure for sidebar grouping.

use std::collections::BTreeMap;

use crate::content::{Page, PAGES};

/// A category containing ordered pages.
pub struct Category {
	pub name: &'static str,
	pub pages: Vec<&'static Page>,
}

/// Returns pages grouped by category, sorted by order within each category.
pub fn get_navigation() -> Vec<Category> {
	let mut categories: BTreeMap<&'static str, Vec<&'static Page>> = BTreeMap::new();

	for page in PAGES.iter() {
		categories.entry(page.category).or_default().push(page);
	}

	categories
		.into_iter()
		.map(|(name, mut pages)| {
			pages.sort_by_key(|p| p.order);
			Category { name, pages }
		})
		.collect()
}
