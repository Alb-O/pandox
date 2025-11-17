#![warn(
	missing_docs,
	missing_debug_implementations,
	missing_copy_implementations,
	trivial_bounds,
	unused_extern_crates,
	unused_results
)]

//! This crate contains all shared UI for the workspace.

/// UI components
pub mod components;

pub use components::{
	Callout, CalloutProps, CalloutTone, ComponentStore, Hero, Navbar, StatelessComponent,
};
