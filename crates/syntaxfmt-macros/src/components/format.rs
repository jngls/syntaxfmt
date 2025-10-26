use std::{array::from_fn, mem::take};

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::Expr;

use crate::{components::{modal::{Strings, NUM_MODES}, parse_basic::ParseBasic}, SyntaxError};

pub trait IntoSplitFormat: Sized {
    type Side;

    fn into_split_format(self) -> (Self::Side, Self::Side, bool);
}

impl IntoSplitFormat for String {
    type Side = Self;
    fn into_split_format(self) -> (Self::Side, Self::Side, bool) {
        if let Some(pos) = self.find("{*}") {
            (self[..pos].into(), self[pos + 3..].into(), true)
        } else if let Some(pos) = self.find("{content}") {
            (self[..pos].into(), self[pos + 9..].into(), true)
        } else {
            Default::default()
        }
    }
}

impl IntoSplitFormat for Strings {
    type Side = Self;

    fn into_split_format(mut self) -> (Self::Side, Self::Side, bool) {
        let mut split: [(String, String, bool); NUM_MODES] = from_fn(|i| take(&mut self[i]).into_split_format());
        let prefix = from_fn(|i| take(&mut split[i].0));
        let suffix = from_fn(|i| take(&mut split[i].1));
        let valid = split.iter().all(|(_, _, b)| *b);
        (Self(prefix), Self(suffix), valid)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Format {
    pub prefix: Strings,
    pub suffix: Strings,
}

impl<'a> ParseBasic<'a> for Format {
    type Input = Expr;

    fn parse_basic(input: &'a Self::Input) -> Result<Self, SyntaxError> {
        let (prefix, suffix, valid) = Strings::parse_basic(input)?.into_split_format();
        let format = valid.then_some(Self { prefix, suffix });
        format.ok_or(SyntaxError::ExpectedContent(input.clone()))
    }
}

impl Format {
    pub fn split<'a>(&'a self) -> (WritePrefix<'a>, WriteSuffix<'a>) {
        (WritePrefix(&self.prefix), WriteSuffix(&self.suffix))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WritePrefix<'a>(pub &'a Strings);

impl<'a> ToTokens for WritePrefix<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let prefix = self.0;
        tokens.extend(quote! { f.write_strs(#prefix)?; });
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteSuffix<'a>(pub &'a Strings);

impl<'a> ToTokens for WriteSuffix<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let suffix = self.0;
        tokens.extend(quote! { f.write_strs(#suffix)?; });
    }
}
