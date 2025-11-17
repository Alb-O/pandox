//! This crate contains all shared UI for the workspace.

mod hero;
pub use hero::Hero;

mod navbar;
pub use navbar::Navbar;

mod block;
pub use block::{
	BLOCK_MANIFEST, Block, BlockRenderer, TypedBlock, block_class, render_from_manifest,
};
