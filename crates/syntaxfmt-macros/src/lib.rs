use std::fmt::Display;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{DeriveInput, Error as SynError, Result as SynResult, parse_macro_input};

use crate::intermediate::ty::SyntaxType;

mod attributes;
mod intermediate;

fn syn_err<T: ToTokens, U: Display, R>(tokens: T, message: U) -> SynResult<R> {
    Err(SynError::new_spanned(tokens, message))
}

#[proc_macro_derive(SyntaxFmt, attributes(syntax, syntax_else))]
pub fn derive_syntax_fmt(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ty = match SyntaxType::from_derive_input(&input) {
        Ok(ty) => ty,
        Err(e) => return e.to_compile_error().into(),
    };

    ty.to_token_stream().into()
}
