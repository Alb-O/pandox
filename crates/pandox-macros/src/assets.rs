use std::path::{Path, PathBuf};

use dioxus_rsx::PartialExpr;

pub struct AssetRewriteCtx {
	modules_root: PathBuf,
	markdown_dir: PathBuf,
}

impl AssetRewriteCtx {
	pub fn new(markdown_path: &Path) -> Self {
		let markdown_dir = markdown_path
			.parent()
			.map(Path::to_path_buf)
			.unwrap_or_else(|| markdown_path.to_path_buf());

		let modules_root = PathBuf::from(&crate::modules_root());

		Self {
			modules_root,
			markdown_dir,
		}
	}

	pub fn asset_expr_for_value(&self, raw: &str) -> Option<PartialExpr> {
		let trimmed = raw.trim();
		if trimmed.is_empty() || is_external(trimmed) {
			return None;
		}

		// Already points into crate assets folder
		if trimmed.starts_with("/assets/") {
			return Some(crate::rewrite::asset_expr_literal(trimmed));
		}

		if trimmed.starts_with(&format!("/{}/", crate::MODULES_DIR)) {
			return Some(crate::rewrite::asset_expr_literal(trimmed));
		}

		let normalized = self.relative_markdown_asset(trimmed)?;

		Some(crate::rewrite::asset_expr_literal(&normalized))
	}

	fn relative_markdown_asset(&self, raw: &str) -> Option<String> {
		let joined = self.markdown_dir.join(raw);
		self.path_to_asset_value(&joined)
	}

	fn path_to_asset_value(&self, absolute: &Path) -> Option<String> {
		let relative = absolute.strip_prefix(&self.modules_root).ok()?;
		let value = relative.to_string_lossy().replace('\\', "/");

		if value.is_empty() {
			return None;
		}

		Some(format!("/{value}"))
	}
}

fn is_external(value: &str) -> bool {
	let lower = value.to_ascii_lowercase();
	lower.starts_with("http://")
		|| lower.starts_with("https://")
		|| lower.starts_with("data:")
		|| value.starts_with("//")
}
