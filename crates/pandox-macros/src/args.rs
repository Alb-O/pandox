use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitStr, Token};

pub struct MarkdownArgs {
    pub path: LitStr,
    pub slug: Option<String>,
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
                "slug" => slug = Some(value.value()),
                other => {
                    return Err(syn::Error::new(
                        key.span(),
                        format!("unknown argument `{other}`"),
                    ));
                }
            }
        }

        Ok(Self { path, slug })
    }
}
