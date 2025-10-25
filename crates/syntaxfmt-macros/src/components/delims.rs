use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::Expr;

use crate::{components::{modal::Strings, parse_basic::ParseBasic}, SyntaxError};

#[cfg(feature = "trace")]
use crate::{trace, DEPTH};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PushDelims(pub Strings);

impl<'a> ParseBasic<'a> for PushDelims {
    type Input = Expr;

    #[cfg_attr(feature = "trace", trace)]
    fn parse_basic(input: &Self::Input) -> Result<Self, SyntaxError> {
        Ok(Self(Strings::parse_basic(input)?))
    }
}

impl ToTokens for PushDelims {
    #[cfg_attr(feature = "trace", trace)]
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let strs = &self.0;
        tokens.extend(quote! { f.push_delim(#strs); });
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PopDelims;

impl ToTokens for PopDelims {
    #[cfg_attr(feature = "trace", trace)]
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.extend(quote! { f.pop_delim(); });
    }
}
