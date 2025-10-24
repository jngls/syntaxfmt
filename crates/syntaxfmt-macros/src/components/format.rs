use std::{array::from_fn, mem::take};

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};

use crate::components::{content::EmitContent, modal::{Bools, Strings, NUM_MODES}};

pub trait IntoSplitFormat: Sized {
    type Fix;
    type WantContent;

    fn into_split_format(self) -> (Self::Fix, Self::Fix, Self::WantContent);
}

impl IntoSplitFormat for String {
    type Fix = Self;
    type WantContent = bool;

    fn into_split_format(self) -> (Self::Fix, Self::Fix, Self::WantContent) {
        if let Some(pos) = self.find("{content}") {
            (self[..pos].into(), self[pos + 9..].into(), true)
        } else {
            (self.clone(), self, false)
        }
    }
}

impl IntoSplitFormat for Strings {
    type Fix = Self;
    type WantContent = Bools;

    fn into_split_format(mut self) -> (Self::Fix, Self::Fix, Self::WantContent) {
        let mut split: [(String, String, bool); NUM_MODES] = from_fn(|i| take(&mut self[i]).into_split_format());
        let prefix = from_fn(|i| take(&mut split[i].0));
        let suffix = from_fn(|i| take(&mut split[i].1));
        let has_content = from_fn(|i| split[i].2);
        (Self(prefix), Self(suffix), Bools(has_content))
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PushFormat {
    pub prefix: Strings,
    pub suffix: Strings,
    pub want_content: Bools,
}

impl EmitContent for PushFormat {
    fn emit_content(&self) -> super::content::ContentMode {
        self.want_content.emit_content()
    }
}

impl ToTokens for PushFormat {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let prefix = &self.prefix;
        let suffix = &self.suffix;
        let want_content = &self.want_content;
        tokens.extend(quote! { f.push_fmt_info(#prefix, #suffix, #want_content); });
    }
}

impl From<Strings> for PushFormat {
    fn from(value: Strings) -> Self {
        let (prefix, suffix, want_content) = value.into_split_format();
        Self { prefix, suffix, want_content }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PopFormat;

impl ToTokens for PopFormat {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.extend(quote! { f.pop_fmt_info(); });
    }
}
