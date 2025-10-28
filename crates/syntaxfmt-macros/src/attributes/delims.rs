use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, token::Comma, Error as SynError, Expr, LitStr, Result as SynResult
};

use crate::{attributes::{modal::Strings}};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PushDelims(pub Strings);

impl PushDelims {
    pub fn from_litstrs(litstrs: Punctuated<LitStr, Comma>) -> SynResult<Option<Self>> {
        Ok(Some(Self(Strings::from_litstrs(litstrs)?)))
    }
}

impl ToTokens for PushDelims {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let strs = &self.0;
        tokens.extend(quote! { f.push_delim(#strs); });
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PopDelims;

impl ToTokens for PopDelims {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.extend(quote! { f.pop_delim(); });
    }
}

impl<'a> From<&'a PushDelims> for PopDelims {
    fn from(_: &'a PushDelims) -> Self {
        Self
    }
}