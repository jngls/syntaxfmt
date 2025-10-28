use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{
    Error as SynError, Expr, LitStr, Result as SynResult, punctuated::Punctuated, token::Comma,
};

use crate::attributes::modal::Strings;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prefix(pub Strings);

impl Prefix {
    #[must_use]
    #[inline(always)]
    pub fn from_litstrs(litstrs: Punctuated<LitStr, Comma>) -> SynResult<Option<Self>> {
        Ok(Some(Self(Strings::from_litstrs(litstrs)?)))
    }
}

impl ToTokens for Prefix {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let prefix = &self.0;
        tokens.extend(quote! { f.write_strs(#prefix)?; });
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Suffix(pub Strings);

impl Suffix {
    #[must_use]
    #[inline(always)]
    pub fn from_litstrs(litstrs: Punctuated<LitStr, Comma>) -> SynResult<Option<Self>> {
        Ok(Some(Self(Strings::from_litstrs(litstrs)?)))
    }
}

impl ToTokens for Suffix {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let suffix = &self.0;
        tokens.extend(quote! { f.write_strs(#suffix)?; });
    }
}
