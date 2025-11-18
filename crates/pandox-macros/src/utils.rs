use std::path::{Component, Path, PathBuf};

use proc_macro::TokenStream;
use proc_macro2::Span as Span2;

pub fn escape_rsx_strings(source: &str) -> String {
	let mut escaped = String::with_capacity(source.len());
	let mut in_string = false;
	let mut is_escaped = false;
	let mut saw_unicode_prefix = false;
	let mut unicode_escape = false;

	for ch in source.chars() {
		if !is_escaped && ch == '"' {
			in_string = !in_string;
			escaped.push(ch);
			continue;
		}

		if in_string && saw_unicode_prefix && ch == '{' {
			unicode_escape = true;
			saw_unicode_prefix = false;
			escaped.push(ch);
			continue;
		}

		if unicode_escape {
			if ch == '}' {
				unicode_escape = false;
			}
			escaped.push(ch);
			continue;
		}

		if in_string && !is_escaped {
			match ch {
				'{' => {
					escaped.push('{');
					escaped.push('{');
					continue;
				}
				'}' => {
					escaped.push('}');
					escaped.push('}');
					continue;
				}
				_ => {}
			}
		}

		if in_string && ch == '\\' && !is_escaped {
			is_escaped = true;
			saw_unicode_prefix = false;
		} else {
			saw_unicode_prefix = is_escaped && matches!(ch, 'u' | 'U');
			is_escaped = false;
		}

		escaped.push(ch);
	}

	escaped
}

pub fn resolve_markdown_path(value: &str) -> Result<PathBuf, TokenStream> {
	let path = Path::new(value);
	if path.as_os_str().is_empty() {
		return Err(compile_error("md path missing"));
	}

	for comp in path.components() {
		match comp {
			Component::Normal(_) => {}
			Component::CurDir => return Err(compile_error("skip `./` in md paths")),
			Component::ParentDir => {
				return Err(compile_error(&format!(
					"md paths stay under {}/",
					crate::MODULES_DIR
				)));
			}
			Component::RootDir | Component::Prefix(_) => {
				return Err(compile_error("absolute md paths not allowed"));
			}
		}
	}

	if matches!(
		path.components().next(),
		Some(Component::Normal(seg)) if seg == crate::MODULES_DIR
	) {
		return Err(compile_error(&format!(
			"omit leading `{}/`",
			crate::MODULES_DIR
		)));
	}

	Ok(Path::new(&crate::modules_root()).join(path))
}

pub fn compile_error(msg: &str) -> TokenStream {
	TokenStream::from(syn::Error::new(Span2::call_site(), msg).to_compile_error())
}
