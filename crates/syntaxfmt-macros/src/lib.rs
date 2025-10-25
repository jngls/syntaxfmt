use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, token::Union, DeriveInput, Error as SynError, Expr, Ident, Meta, MetaList, Path};

use crate::intermediate::{parse_type::ParseType, ty::SyntaxType};

#[cfg(feature = "trace")]
use trace::{init_depth_var, trace};

#[cfg(feature = "trace")]
init_depth_var!();

mod components;
mod intermediate;

#[derive(Debug, Clone)]
enum SyntaxError {
    ExpectedStringLit(Expr),
    ExpectedStringLits(Expr),
    ExpectedTraitPath(Path),
    ExpectedContent(Expr),
    ExpectedMetaList(Meta),
    ExpectedCondition(Path),
    UnexpectedAttributeArg(Path),
    UnexpectedAttributeType(Meta),
    UnexpectedContentExpr(Expr),
    UnexpectedConditionalExpr(Expr),
    FailedToParseMeta(MetaList, SynError),
    Union(Union),
}

#[proc_macro_derive(SyntaxFmt, attributes(syntax))]
pub fn derive_syntax_fmt(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ty = match SyntaxType::parse_type(&mut Vec::new(), &input) {
        Ok(ty) => ty,
        Err(e) => {
            return match e {
                SyntaxError::ExpectedStringLit(t) =>
                    SynError::new_spanned(t, "syntaxfmt argument expected string literal"),
                SyntaxError::ExpectedStringLits(t) =>
                    SynError::new_spanned(t, "syntaxfmt argument expected string literal or tuple of string literals (\"normal\", \"pretty\")"),
                SyntaxError::ExpectedTraitPath(t) =>
                    SynError::new_spanned(t, "syntaxfmt argument expected trait path"),
                SyntaxError::ExpectedContent(t) =>
                    SynError::new_spanned(t, "syntaxfmt `format` argument must contain `{*}` placeholder.
If you simply want to replace the content, please use the `content = \"xyz\"` argument instead."),
                SyntaxError::ExpectedMetaList(t) =>
                    SynError::new_spanned(t, "syntaxfmt argument expected meta list"),
                SyntaxError::ExpectedCondition(t) =>
                    SynError::new_spanned(t, "syntaxfmt argument expected `condition` argument"),
                SyntaxError::UnexpectedAttributeArg(t) =>
                    SynError::new_spanned(t, "syntaxfmt unexpected attribute argument"),
                SyntaxError::UnexpectedAttributeType(t) =>
                    SynError::new_spanned(t, "syntaxfmt unexpected attribute argument type"),
                SyntaxError::UnexpectedContentExpr(t) =>
                    SynError::new_spanned(t, "syntaxfmt unexpected content expression"),
                SyntaxError::UnexpectedConditionalExpr(t) => {
                    let dbg = format!("{t:?}");
                    SynError::new_spanned(t, format!("syntaxfmt unexpected conditional expression: {dbg}"))
                }
                SyntaxError::FailedToParseMeta(t, e) =>
                    SynError::new_spanned(t, format!("syntaxfmt failed to parse attribute meta list: {e}")),
                SyntaxError::Union(t) =>
                    SynError::new_spanned(t, "syntaxfmt cannot be derived for unions"),
            }.to_compile_error().into();
        }
    };

    ty.to_token_stream().into()
}
