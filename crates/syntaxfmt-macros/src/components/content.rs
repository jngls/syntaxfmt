use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{Expr, Ident};

use crate::{components::{delims::{PopDelims, PushDelims}, format::{PopFormat, PushFormat}, indent::{PopIndentRegion, PushIndentRegion, WriteIndent}, modal::Bools, none::{ClearNone, SetNone}, parse_tokens::ParseTokens}, intermediate::attributes::SyntaxAttributes, SyntaxError};

pub trait EmitContent {
    fn emit_content(&self) -> ContentMode;
}

impl EmitContent for Bools {
    fn emit_content(&self) -> ContentMode {
        let count = self.0.iter().filter(|b| **b).count();
        if count == self.0.len() {
            return ContentMode::Always;
        }
        match count {
            0 => ContentMode::Never,
            _ => ContentMode::Runtime,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ContentMode {
    #[default]
    Never,
    Runtime,
    Always,
}

#[derive(Debug, Clone)]
pub struct WriteContent(pub Expr);

impl WriteContent {
    pub fn tokens_or(_self: &Option<Self>, ident: &Ident, or: impl ToTokens) -> TokenStream2 {
        match _self {
            Some(Self(Expr::Call(inner))) => quote! { (#inner)?; },
            Some(Self(Expr::Macro(inner))) => quote! { (#inner)?; },
            Some(Self(Expr::Closure(inner))) => quote! { let i = #ident; (#inner)(i, f)?; },
            Some(Self(Expr::Path(inner))) => quote! { let i = #ident; (#inner)(i, f)?; },
            Some(Self(Expr::Lit(inner))) => quote! { (#inner).syntax_fmt(f)?; },
            Some(Self(Expr::Tuple(inner))) => quote! { (#inner).syntax_fmt(f)?; },
            None => or.to_token_stream(),
            _ => TokenStream2::new(),
        }
    }

    pub fn gen_content(attrs: &impl SyntaxAttributes, pre: &impl ToTokens, content: &TokenStream2, post: &impl ToTokens) -> TokenStream2 {
        let content_mode = attrs
            .format()
            .map(|f| f.emit_content())
            .unwrap_or(ContentMode::Always);
        
        match content_mode {
            ContentMode::Never => quote! { #pre #post },
            ContentMode::Runtime => quote! {
                #pre
                if f.has_content() {
                    #content
                }
                #post
            },
            ContentMode::Always => quote! { #pre #content #post },
        }
    }
}

impl ParseTokens for WriteContent {
    type Input = Expr;

    fn parse_tokens(expr: &Self::Input) -> Result<Self, SyntaxError> {
        Ok(match expr {
            e @ Expr::Call(..) => Self(e.clone()),
            e @ Expr::Macro(..) => Self(e.clone()),
            e @ Expr::Closure(..) => Self(e.clone()),
            e @ Expr::Path(..) => Self(e.clone()),
            e @ Expr::Lit(..) => Self(e.clone()),
            e @ Expr::Tuple(..) => Self(e.clone()),
            e => return Err(SyntaxError::UnexpectedContentExpr(e.clone())),
        })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SyntaxTypePreContent<'a> {
    pub push_format: Option<&'a PushFormat>,
    pub push_delim: Option<&'a PushDelims>,
    pub push_indent: Option<&'a PushIndentRegion>,
    pub write_indent: Option<&'a WriteIndent>,
}

impl<'a> ToTokens for SyntaxTypePreContent<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let write_indent = self.write_indent;
        let push_format = self.push_format;
        let push_delim = self.push_delim;
        let push_indent = self.push_indent;
        tokens.extend(quote! {
            #write_indent
            #push_format
            #push_delim
            #push_indent
        });
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SyntaxTypePostContent {
    pub pop_format: Option<PopFormat>,
    pub pop_delim: Option<PopDelims>,
    pub pop_indent: Option<PopIndentRegion>,
}

impl ToTokens for SyntaxTypePostContent {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let pop_format = self.pop_format.as_ref();
        let pop_delim = self.pop_delim.as_ref();
        let pop_indent = self.pop_indent.as_ref();
        tokens.extend(quote! {
            #pop_indent
            #pop_delim
            #pop_format
        });
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SyntaxFieldPreContent<'a> {
    pub push_format: Option<&'a PushFormat>,
    pub push_delim: Option<&'a PushDelims>,
    pub push_indent: Option<&'a PushIndentRegion>,
    pub write_indent: Option<&'a WriteIndent>,
    pub set_none: Option<&'a SetNone>,
}

impl<'a> ToTokens for SyntaxFieldPreContent<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let default_format = PushFormat::default();
        let write_indent = self.write_indent;
        let push_format = self.push_format.unwrap_or(&default_format);
        let push_delim = self.push_delim;
        let push_indent = self.push_indent;
        let set_none = self.set_none;
        tokens.extend(quote! {
            #write_indent
            #push_format
            #push_delim
            #push_indent
            #set_none
        });
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SyntaxFieldPostContent {
    pub pop_format: Option<PopFormat>,
    pub pop_delim: Option<PopDelims>,
    pub pop_indent: Option<PopIndentRegion>,
    pub clear_none: Option<ClearNone>,
}

impl ToTokens for SyntaxFieldPostContent {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let pop_format = self.pop_format.as_ref();
        let pop_delim = self.pop_delim.as_ref();
        let pop_indent = self.pop_indent.as_ref();
        let clear_none = self.clear_none.as_ref();
        tokens.extend(quote! {
            #clear_none
            #pop_indent
            #pop_delim
            #pop_format
        });
    }
}
