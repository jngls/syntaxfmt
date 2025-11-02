use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::Ident;

use crate::attributes::modal::Strings;

#[derive(Debug, Default, Clone)]
pub enum FieldKind {
    #[default]
    SelfValue,
    Field(Ident),
}

impl ToTokens for FieldKind {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            FieldKind::SelfValue => tokens.extend(quote! { self }),
            FieldKind::Field(ident) => ident.to_tokens(tokens),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PushContext {
    pub sep: Strings,
    pub indent: bool,
    pub nl_sep: bool,
}

impl ToTokens for PushContext {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let sep = &self.sep;
        let indent = self.indent;
        let nl_sep = self.nl_sep;
        tokens.extend(quote! { f.push_context(#sep, #indent, #nl_sep); });
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PopContext;

impl ToTokens for PopContext {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.extend(quote! { f.pop_context(); });
    }
}
