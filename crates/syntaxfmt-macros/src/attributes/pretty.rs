use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, token::Comma, Ident, Result as SynResult};

use crate::syn_err;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Newlines(u8);

impl Newlines {
    pub const BEG: Self = Self(0b0001);
    pub const PRE: Self = Self(0b0010);
    pub const CON: Self = Self(0b0100);
    pub const SUF: Self = Self(0b1000);

    #[must_use]
    pub fn from_idents(idents: Punctuated<Ident, Comma>) -> SynResult<Self> {
        let mut _self = Self::default();
        for ident in idents {
            let mut nl = Self::default();
            if ident == "beg" {
                nl = Self::BEG;
            } else if ident == "pre" {
                nl = Self::PRE;
            } else if ident == "cont" {
                nl = Self::CON;
            } else if ident == "suf" {
                nl = Self::SUF;
            } else {
                return syn_err(ident, "syntaxfmt unexpected newline argument");
            }
            if _self.has(nl) {
                return syn_err(ident, "syntaxfmt found duplicate newline argument");
            }
            _self.combine(nl);
        }
        Ok(_self)
    }

    #[must_use]
    #[inline(always)]
    pub fn has(&self, nl: Self) -> bool {
        self.0 & nl.0 == nl.0
    }

    #[must_use]
    #[inline(always)]
    pub fn has_beg(&self) -> bool {
        self.0 & Self::BEG.0 != 0
    }

    #[must_use]
    #[inline(always)]
    pub fn has_pre(&self) -> bool {
        self.0 & Self::PRE.0 != 0
    }

    #[must_use]
    #[inline(always)]
    pub fn has_cont(&self) -> bool {
        self.0 & Self::CON.0 != 0
    }

    #[must_use]
    #[inline(always)]
    pub fn has_suf(&self) -> bool {
        self.0 & Self::SUF.0 != 0
    }

    #[inline(always)]
    pub fn combine(&mut self, nl: Self) {
        self.0 |= nl.0;
    }

    #[must_use]
    #[inline(always)]
    pub fn into_beg(self) -> Option<WriteNewline> {
        self.has_beg().then_some(WriteNewline)
    }

    #[must_use]
    #[inline(always)]
    pub fn into_pre(self) -> Option<WriteNewline> {
        self.has_pre().then_some(WriteNewline)
    }

    #[must_use]
    #[inline(always)]
    pub fn into_cont(self) -> Option<WriteNewline> {
        self.has_cont().then_some(WriteNewline)
    }

    #[must_use]
    #[inline(always)]
    pub fn into_suf(self) -> Option<WriteNewline> {
        self.has_suf().then_some(WriteNewline)
    }
}

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

impl<'a> From<&'a PushIndentRegion> for PopIndentRegion {
    fn from(_: &'a PushIndentRegion) -> Self {
        Self
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct WriteNewline;

impl ToTokens for WriteNewline {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.extend(quote! { f.write_newline()?; });
    }
}
