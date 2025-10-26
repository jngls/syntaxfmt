use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::Expr;

use crate::components::{modal::Strings, parse_basic::ParseBasic};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prefix(pub Strings);

impl<'a> ParseBasic<'a> for Prefix {
    type Input = Expr;

    fn parse_basic(input: &'a Self::Input) -> Result<Self, crate::SyntaxError> {
        Ok(Self(Strings::parse_basic(input)?))
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

impl<'a> ParseBasic<'a> for Suffix {
    type Input = Expr;

    fn parse_basic(input: &'a Self::Input) -> Result<Self, crate::SyntaxError> {
        Ok(Self(Strings::parse_basic(input)?))
    }
}

impl ToTokens for Suffix {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let suffix = &self.0;
        tokens.extend(quote! { f.write_strs(#suffix)?; });
    }
}
