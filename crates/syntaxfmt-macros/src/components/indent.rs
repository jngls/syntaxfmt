use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PushIndentRegion;

impl ToTokens for PushIndentRegion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.extend(quote! { f.push_indent(); });
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PopIndentRegion;

impl ToTokens for PopIndentRegion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.extend(quote! { f.pop_indent(); });
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct WriteNewline;

impl ToTokens for WriteNewline {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.extend(quote! { f.write_newline()?; });
    }
}
