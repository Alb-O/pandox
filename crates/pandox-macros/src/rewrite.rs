use dioxus_rsx::{
	Attribute, AttributeName, AttributeValue, BodyNode, HotLiteral, IfChain, PartialExpr,
};
use proc_macro2::Span as Span2;
use syn::{LitStr, parse_quote};

use crate::assets::AssetRewriteCtx;

pub fn rewrite_asset_srcs(nodes: &mut [BodyNode], ctx: &mut AssetRewriteCtx) {
	for node in nodes {
		match node {
			BodyNode::Element(el) => {
				rewrite_asset_attrs(&mut el.raw_attributes, ctx);
				rewrite_asset_attrs(&mut el.merged_attributes, ctx);
				rewrite_asset_srcs(&mut el.children, ctx);
			}
			BodyNode::Component(component) => {
				rewrite_asset_attrs(&mut component.fields, ctx);
				rewrite_asset_srcs(&mut component.children.roots, ctx);
			}
			BodyNode::ForLoop(for_loop) => rewrite_asset_srcs(&mut for_loop.body.roots, ctx),
			BodyNode::IfChain(chain) => rewrite_ifchain(chain, ctx),
			_ => {}
		}
	}
}

fn rewrite_ifchain(chain: &mut IfChain, ctx: &mut AssetRewriteCtx) {
	rewrite_asset_srcs(&mut chain.then_branch.roots, ctx);

	if let Some(else_branch) = &mut chain.else_branch {
		rewrite_asset_srcs(&mut else_branch.roots, ctx);
	}

	if let Some(else_if) = &mut chain.else_if_branch {
		rewrite_ifchain(else_if, ctx);
	}
}

fn rewrite_asset_attrs(attrs: &mut [Attribute], ctx: &mut AssetRewriteCtx) {
	for attr in attrs {
		rewrite_asset_attr(attr, ctx);
	}
}

fn rewrite_asset_attr(attr: &mut Attribute, ctx: &mut AssetRewriteCtx) {
	let AttributeName::BuiltIn(name) = &attr.name else {
		return;
	};

	if name.to_string() != "src" {
		return;
	}

	let AttributeValue::AttrLiteral(HotLiteral::Fmted(lit)) = &attr.value else {
		return;
	};

	let raw_value = lit.formatted_input.source.value();

	if let Some(expr) = ctx.asset_expr_for_value(&raw_value) {
		attr.value = AttributeValue::AttrExpr(expr);
	}
}

pub fn asset_expr_literal(path: &str) -> PartialExpr {
	let lit = LitStr::new(path, Span2::call_site());
	let expr = parse_quote! {{
		::dioxus::core::AttributeValue::Text(::std::string::String::from(asset!(#lit)))
	}};
	PartialExpr::from_expr(&expr)
}
