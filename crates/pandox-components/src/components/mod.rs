use std::collections::BTreeMap;

use dioxus::prelude::*;

/// Callout component
pub mod callout;
/// Hero component
pub mod hero;
/// Navbar component
pub mod navbar;

pub use callout::{Callout, CalloutProps, CalloutTone};
pub use hero::Hero;
pub use navbar::Navbar;

/// Type alias for storing stateless components in [`ComponentStore`].
pub type StatelessComponent = fn() -> Element;

/// Simple registry for reusable stateless components.
#[derive(Debug)]
pub struct ComponentStore {
	map: BTreeMap<&'static str, StatelessComponent>,
}

impl ComponentStore {
	/// Construct an empty component store.
	pub fn new() -> Self {
		Self {
			map: BTreeMap::new(),
		}
	}

	/// Construct a store with defaults registered.
	pub fn with_defaults() -> Self {
		Self::new().register("Hero", Hero as StatelessComponent)
	}

	/// Register a component under a given name.
	#[allow(unused_results)]
	pub fn register(mut self, name: &'static str, component: StatelessComponent) -> Self {
		self.map.insert(name, component);
		self
	}

	/// Retrieve a component by name.
	pub fn get(&self, name: &str) -> Option<StatelessComponent> {
		self.map.get(name).copied()
	}

	/// Iterate through stored component entries.
	pub fn entries(&self) -> impl Iterator<Item = (&'static str, &StatelessComponent)> {
		self.map.iter().map(|(name, component)| (*name, component))
	}
}

impl Default for ComponentStore {
	fn default() -> Self {
		Self::new()
	}
}
