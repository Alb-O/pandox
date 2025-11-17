//!! Macros for building markdown-based components.

use std::path::{Component, Path, PathBuf};

use pandox_markdown::{BlockComponent, MarkdownParser};
use proc_macro::TokenStream;
use proc_macro2::Span as Span2;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Ident, LitStr, Token};

const MODULES_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../modules");

struct MarkdownArgs {
	path: LitStr,
}

impl Parse for MarkdownArgs {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let path: LitStr = input.parse()?;
		let mut slug = None;

		while input.peek(Token![,]) {
			input.parse::<Token![,]>()?;
			if input.is_empty() {
				break;
			}

			let key: Ident = input.parse()?;
			input.parse::<Token![=]>()?;
			let value: LitStr = input.parse()?;
			match key.to_string().as_str() {
				"slug" => slug = Some(value),
				other => {
					return Err(syn::Error::new(
						key.span(),
						format!("unknown argument `{other}`"),
					));
				}
			}
		}

		Ok(Self {
			path,
		})
	}
}

/// Macro to include a markdown file as a Dioxus RSX component.
#[proc_macro]
pub fn mdfile(input: TokenStream) -> TokenStream {
	let args = parse_macro_input!(input as MarkdownArgs);

	match expand_markdown(&args) {
		Ok(stream) => stream,
		Err(err) => err,
	}
}

fn expand_markdown(args: &MarkdownArgs) -> Result<TokenStream, TokenStream> {
	let crate_root = std::env::var("CARGO_MANIFEST_DIR")
		.map(PathBuf::from)
		.map_err(|_| compile_error("CARGO_MANIFEST_DIR not set"))?;

	let path_str = args.path.value();

	let markdown_path = resolve_markdown_path(&path_str)?;

	let parser = MarkdownParser::new();
	let components = parser
		.extract_components(
			markdown_path.as_path(),
			Some(crate_root.as_path()),
		)
		.map_err(|err| compile_error(&err.to_string()))?;

	let nodes = components.into_iter().map(block_to_tokens);

	let expanded = quote! {
		rsx! {
			#(#nodes)*
		}
	};

	Ok(expanded.into())
}

fn block_to_tokens(block: BlockComponent) -> proc_macro2::TokenStream {
	html_to_rsx_tokens(&block.html)
}

fn html_to_rsx_tokens(html: &str) -> proc_macro2::TokenStream {
	if html.trim().is_empty() {
		return proc_macro2::TokenStream::new();
	}

	let rsx = html_to_rsx(html);
	let snippet = rsx.trim();
	if snippet.is_empty() {
		return proc_macro2::TokenStream::new();
	}

	let escaped = escape_rsx_strings(snippet);

	syn::parse_str(&escaped)
		.unwrap_or_else(|err| panic!("failed to parse RSX tokens: {err}\n{escaped}"))
}

fn html_to_rsx(html: &str) -> String {
	let dom =
		html_parser::Dom::parse(html).unwrap_or_else(|err| panic!("failed to parse HTML: {err}"));

	let callbody = dioxus_rsx_rosetta::rsx_from_html(&dom);

	dioxus_autofmt::write_block_out(&callbody).unwrap_or_else(|| panic!("failed to format RSX"))
}

fn escape_rsx_strings(source: &str) -> String {
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

fn resolve_markdown_path(value: &str) -> Result<PathBuf, TokenStream> {
	let path = Path::new(value);
	if path.as_os_str().is_empty() {
		return Err(compile_error("md path missing"));
	}

	for comp in path.components() {
		match comp {
			Component::Normal(_) => {}
			Component::CurDir => return Err(compile_error("skip `./` in md paths")),
			Component::ParentDir => return Err(compile_error("md paths stay under modules/")),
			Component::RootDir | Component::Prefix(_) => {
				return Err(compile_error("absolute md paths not allowed"))
			}
		}
	}

	if matches!(
		path.components().next(),
		Some(Component::Normal(seg)) if seg == "modules"
	) {
		return Err(compile_error("omit leading `modules/`"));
	}

	Ok(Path::new(MODULES_ROOT).join(path))
}

fn compile_error(msg: &str) -> TokenStream {
	TokenStream::from(syn::Error::new(Span2::call_site(), msg).to_compile_error())
}
