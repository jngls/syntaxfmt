use std::{array::from_fn, ops::{Index, IndexMut}};

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{Expr, ExprLit, ExprTuple, Lit};

use crate::{components::{parse_basic::ParseBasic}, SyntaxError};

#[cfg(feature = "trace")]
use crate::{trace, DEPTH};

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

impl<'a> ParseBasic<'a> for Strings {
    type Input = Expr;

    #[cfg_attr(feature = "trace", trace)]
    fn parse_basic(expr: &Self::Input) -> Result<Self, SyntaxError> {
        match expr {
            Expr::Lit(ExprLit {
                lit: Lit::Str(s), ..
            }) => {
                let s = s.value();
                Ok(Self(from_fn(|_| s.clone())))
            }
            Expr::Tuple(ExprTuple { elems, .. }) => {
                let mut strs = Strings::default();
                let mut i = 0;
                for (s, e) in strs.0.iter_mut().zip(elems) {
                    *s = String::parse_basic(e)?;
                    i += 1;
                }
                if i != NUM_MODES {
                    return Err(SyntaxError::ExpectedStringLits(expr.clone()));
                }
                Ok(strs)
            }
            _ => Err(SyntaxError::ExpectedStringLits(expr.clone())),
        }
    }
}

impl ToTokens for Strings {
    #[cfg_attr(feature = "trace", trace)]
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let strs = &self.0;
        tokens.extend(quote! { [#(#strs),*] });
    }
}
