use std::{array::from_fn, ops::{Index, IndexMut}};

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, token::Comma, Error as SynError, Expr, ExprArray, ExprLit, Lit, LitStr, Result as SynResult
};

pub const NUM_MODES: usize = 2;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Strings(pub [String; NUM_MODES]);

impl Index<usize> for Strings {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Strings {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

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
                return Err(SynError::new_spanned(litstrs, format!("syntaxfmt expected {NUM_MODES} string literals (one for each mode `normal`, `pretty`, etc.)")));
            }
        }
        Ok(strs)
    }
}

impl ToTokens for Strings {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let strs = &self.0;
        tokens.extend(quote! { [#(#strs),*] });
    }
}
