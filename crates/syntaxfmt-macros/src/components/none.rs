use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::Expr;

use crate::{components::{modal::Strings, parse_tokens::ParseTokens}, SyntaxError};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SetNone(pub Strings);

impl ParseTokens for SetNone {
    type Input = Expr;

    fn parse_tokens(expr: &Self::Input) -> Result<Self, SyntaxError> {
        Ok(Self(Strings::parse_tokens(expr)?))
    }
}

impl ToTokens for SetNone {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let strs = &self.0;
        tokens.extend(quote! { f.set_none(#strs); });
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ClearNone;

impl ToTokens for ClearNone {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.extend(quote! { f.clear_none(); });
    }
}
