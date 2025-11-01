use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{LitStr, Result as SynResult, punctuated::Punctuated, token::Comma};

use crate::attributes::modal::Strings;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PushSeps(pub Strings);

impl PushSeps {
    pub fn from_litstrs(litstrs: Punctuated<LitStr, Comma>) -> SynResult<Option<Self>> {
        Ok(Some(Self(Strings::from_litstrs(litstrs)?)))
    }
}

impl ToTokens for PushSeps {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let strs = &self.0;
        tokens.extend(quote! { f.push_sep(#strs); });
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PopSeps;

impl ToTokens for PopSeps {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.extend(quote! { f.pop_sep(); });
    }
}

impl<'a> From<&'a PushSeps> for PopSeps {
    fn from(_: &'a PushSeps) -> Self {
        Self
    }
}
