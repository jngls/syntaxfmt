use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{
    DeriveInput, Error as SynError, Expr, Path, parse_macro_input,
    token::Union,
};

use crate::intermediate::{parse_type::ParseType, ty::SyntaxType};

mod intermediate;
mod components;

enum SyntaxError {
    ExpectedStringLit(Expr),
    ExpectedStringLits(Expr),
    ExpectedTraitPath(Expr),
    UnexpectedAttributeArg(Path),
    UnexpectedContentExpr(Expr),
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
                SyntaxError::UnexpectedAttributeArg(t) =>
                    SynError::new_spanned(t, "syntaxfmt unexpected attribute argument"),
                SyntaxError::UnexpectedContentExpr(t) =>
                    SynError::new_spanned(t, "syntaxfmt unexpected content expression"),
                SyntaxError::Union(t) =>
                    SynError::new_spanned(t, "syntaxfmt cannot be derived for unions"),
            }.to_compile_error().into();
        }
    };

    ty.to_token_stream().into()
}
