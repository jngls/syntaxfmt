use std::ops::{BitOr, BitOrAssign};

use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{Ident, Result as SynResult, punctuated::Punctuated, token::Comma};

use crate::syn_err;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Newlines(u8);

impl Newlines {
    pub const BEG: Self = Self(1 << 0);
    pub const PRE: Self = Self(1 << 1);
    pub const CON: Self = Self(1 << 2);
    pub const SUF: Self = Self(1 << 3);
    pub const SEP: Self = Self(1 << 4);

    #[must_use]
    pub fn from_idents(idents: Punctuated<Ident, Comma>) -> SynResult<Self> {
        let mut _self = Self::default();
        for ident in idents {
            let nl;
            if ident == "beg" {
                nl = Self::BEG;
            } else if ident == "pre" {
                nl = Self::PRE;
            } else if ident == "cont" {
                nl = Self::CON;
            } else if ident == "suf" {
                nl = Self::SUF;
            } else if ident == "sep" {
                nl = Self::SEP;
            } else if ident == "inner" {
                nl = Self::PRE | Self::SEP | Self::CON;
            } else if ident == "outer" {
                nl = Self::BEG | Self::SUF;
            } else {
                return syn_err(ident, "syntaxfmt unexpected newline argument");
            }
            _self |= nl;
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
    pub fn into_beg(self) -> Option<WriteNewline> {
        self.has(Self::BEG).then_some(WriteNewline)
    }

    #[must_use]
    #[inline(always)]
    pub fn into_pre(self) -> Option<WriteNewline> {
        self.has(Self::PRE).then_some(WriteNewline)
    }

    #[must_use]
    #[inline(always)]
    pub fn into_cont(self) -> Option<WriteNewline> {
        self.has(Self::CON).then_some(WriteNewline)
    }

    #[must_use]
    #[inline(always)]
    pub fn into_suf(self) -> Option<WriteNewline> {
        self.has(Self::SUF).then_some(WriteNewline)
    }
}

impl BitOr for Newlines {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Newlines {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct WriteNewline;

impl ToTokens for WriteNewline {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.extend(quote! { f.write_newline()?; });
    }
}
