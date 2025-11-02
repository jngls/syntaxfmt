use std::ops::{Deref, DerefMut};

use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{LitStr, Result as SynResult, punctuated::Punctuated, token::Comma};

use crate::syn_err;

pub const NUM_MODES: usize = 2;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Strings(pub [String; NUM_MODES]);

impl Strings {
    #[must_use]
    pub fn from_litstrs(litstrs: Punctuated<LitStr, Comma>) -> SynResult<Self> {
        let mut strs = Strings::default();

        if litstrs.len() == 1 {
            strs.0.iter_mut().for_each(|s| *s = litstrs[0].value());
        } else {
            let mut i = 0;
            for (s, e) in strs.0.iter_mut().zip(&litstrs) {
                *s = e.value();
                i += 1;
            }
            if i != NUM_MODES {
                return syn_err(
                    litstrs,
                    format!(
                        "syntaxfmt expected {NUM_MODES} string literals (one for each mode `normal`, `pretty`, etc.)"
                    ),
                );
            }
        }
        Ok(strs)
    }
}

impl Deref for Strings {
    type Target = [String; NUM_MODES];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Strings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ToTokens for Strings {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let strs = &self.0;
        tokens.extend(quote! { [#(#strs),*] });
    }
}
