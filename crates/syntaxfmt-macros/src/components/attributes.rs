use std::fmt::Debug;

use proc_macro2::TokenStream as TokenStream2;

use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, token::Comma, Attribute, Expr, ExprPath, Meta, MetaNameValue, Path};

use crate::{
    SyntaxError,
    components::{
        eval::Eval,
        content::Content,
        delims::{PopDelims, PushDelims},
        format::Format,
        indent::{PopIndentRegion, PushIndentRegion, WriteIndent},
        parse_basic::ParseBasic,
    },
};

#[cfg(feature = "trace")]
use crate::{DEPTH, trace};

#[derive(Debug, Clone)]
enum ParsedMetaPath {
    IndentRegion,
    Indent,
    Skip,
}

impl<'a> ParseBasic<'a> for ParsedMetaPath {
    type Input = Path;

    #[cfg_attr(feature = "trace", trace)]
    fn parse_basic(input: &'a Self::Input) -> Result<Self, SyntaxError> {
        if input.is_ident("indent_region") {
            Ok(Self::IndentRegion)
        } else if input.is_ident("indent") {
            Ok(Self::Indent)
        } else if input.is_ident("skip") {
            Ok(Self::Skip)
        } else {
            Err(SyntaxError::UnexpectedAttributeArg(input.clone()))
        }
    }
}

#[derive(Debug, Clone)]
enum ParsedMetaNameValue {
    Format(Format),
    Delims(PushDelims),
    Eval(Eval),
    Content(Content),
    ElseContent(Path, Content),
    StateBound(Path, ExprPath),
}

impl<'a> ParseBasic<'a> for ParsedMetaNameValue {
    type Input = MetaNameValue;

    #[cfg_attr(feature = "trace", trace)]
    fn parse_basic(input: &'a Self::Input) -> Result<Self, SyntaxError> {
        let path = &input.path;
        let value = &input.value;

        if path.is_ident("format") {
            Ok(Self::Format(Format::parse_basic(value)?))
        } else if path.is_ident("delim") {
            Ok(Self::Delims(PushDelims::parse_basic(value)?))
        } else if path.is_ident("eval") {
            Ok(Self::Eval(Eval::parse_basic(value)?))
        } else if path.is_ident("content") {
            Ok(Self::Content(Content::parse_basic(value)?))
        } else if path.is_ident("else_content") {
            Ok(Self::ElseContent(path.clone(), Content::parse_basic(value)?))
        } else if path.is_ident("state_bound") {
            match value {
                Expr::Path(inner) => Ok(Self::StateBound(path.clone(), inner.clone())),
                _ => Err(SyntaxError::ExpectedTraitPath(path.clone()))?,
            }
        } else {
            Err(SyntaxError::UnexpectedAttributeArg(path.clone()))
        }
    }
}

#[derive(Debug, Clone)]
enum ParsedMeta {
    Path(ParsedMetaPath),
    NameValue(ParsedMetaNameValue),
}

impl<'a> ParseBasic<'a> for ParsedMeta {
    type Input = Meta;

    fn parse_basic(input: &'a Self::Input) -> Result<Self, SyntaxError> {
        match input {
            Meta::Path(inner) => Ok(Self::Path(ParsedMetaPath::parse_basic(inner)?)),
            Meta::NameValue(inner) => Ok(Self::NameValue(ParsedMetaNameValue::parse_basic(inner)?)),
            input => Err(SyntaxError::UnexpectedAttributeType(input.clone())),
        }
    }
}

#[derive(Debug, Clone)]
struct ParsedAttributes(pub Vec<ParsedMeta>);

impl<'a> ParseBasic<'a> for ParsedAttributes {
    type Input = [Attribute];

    fn parse_basic(input: &'a Self::Input) -> Result<Self, SyntaxError> {
        let mut metas = Vec::new();

        let parse_meta = |metas: &mut Vec<ParsedMeta>, meta: &'a Meta| match meta.require_list() {
            Ok(meta_list) => {
                match meta_list.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated) {
                    Ok(punct_meta_list) => {
                        for meta in punct_meta_list {
                            metas.push(ParsedMeta::parse_basic(&meta)?);
                        }
                        Ok(())
                    }
                    Err(e) => Err(SyntaxError::FailedToParseMeta(meta_list.clone(), e)),
                }
            }
            Err(_) => Err(SyntaxError::ExpectedMetaList(meta.clone())),
        };

        for attr in input {
            if attr.path().is_ident("syntax") {
                parse_meta(&mut metas, &attr.meta)?;
            }
        }

        Ok(Self(metas))
    }
}

#[derive(Debug, Default, Clone)]
pub struct Attributes {
    pub format: Option<Format>,
    pub delims: Option<PushDelims>,
    pub cond: Option<Eval>,
    pub content: Option<Content>,
    pub else_content: Option<(Path, Content)>,
    pub state_bound: Option<(Path, ExprPath)>,
    pub indent_region: bool,
    pub indent: bool,
    pub skip: bool,
}

impl Attributes {
    pub fn parse_for_type<'a>(input: &'a [Attribute]) -> Result<Self, SyntaxError> {
        let mut attrs: Attributes = ParsedAttributes::parse_basic(input)?.into();
        if attrs.cond.is_none() {
            if let Some((path, _)) = attrs.else_content.take() {
                Err(SyntaxError::ExpectedCondition(path))?
            }
        }
        Ok(attrs)
    }

    pub fn parse_for_field<'a>(input: &'a [Attribute]) -> Result<Self, SyntaxError> {
        let mut attrs: Attributes = ParsedAttributes::parse_basic(input)?.into();
        if attrs.cond.is_none() {
            if let Some((path, _)) = attrs.else_content.take() {
                Err(SyntaxError::ExpectedCondition(path))?
            }
        }
        if let Some((path, _)) = attrs.state_bound.take() {
            Err(SyntaxError::UnexpectedAttributeArg(path))?
        }
        Ok(attrs)
    }

    pub fn to_tokens(&self, insert: impl ToTokens, default_content: Content) -> TokenStream2 {
        let (prefix, suffix) = match &self.format {
            Some(f) => {
                let (prefix, suffix) = f.split();
                (Some(prefix), Some(suffix))
            }
            None => Default::default(),
        };

        let (push_delims, pop_delims) = match &self.delims {
            Some(d) => (Some(d), Some(PopDelims)),
            None => Default::default(),
        };

        let cond = &self.cond;

        let content = self.content.as_ref().unwrap_or(&default_content);

        let (push_indent, pop_indent) = if self.indent_region {
            (Some(PushIndentRegion), Some(PopIndentRegion))
        } else {
            Default::default()
        };

        let indent = self.indent.then_some(WriteIndent);

        let pre = quote! { #indent #prefix #push_delims #push_indent };

        let post = quote! { #pop_indent #pop_delims #suffix };

        match cond {
            Some(cond) => quote! {
                #insert
                if #cond {
                    #pre #content #post
                }
            },
            _ => quote! { #insert #pre #content #post },
        }
    }
}

impl From<ParsedAttributes> for Attributes {
    fn from(value: ParsedAttributes) -> Self {
        let mut attrs = Self::default();
        for parsed_meta in value.0 {
            match parsed_meta {
                ParsedMeta::Path(ParsedMetaPath::IndentRegion) => attrs.indent_region = true,
                ParsedMeta::Path(ParsedMetaPath::Indent) => attrs.indent = true,
                ParsedMeta::Path(ParsedMetaPath::Skip) => attrs.skip = true,
                ParsedMeta::NameValue(ParsedMetaNameValue::Format(f)) => attrs.format = Some(f),
                ParsedMeta::NameValue(ParsedMetaNameValue::Delims(d)) => attrs.delims = Some(d),
                ParsedMeta::NameValue(ParsedMetaNameValue::Eval(c)) => attrs.cond = Some(c),
                ParsedMeta::NameValue(ParsedMetaNameValue::Content(c)) => attrs.content = Some(c),
                ParsedMeta::NameValue(ParsedMetaNameValue::ElseContent(p, c)) => attrs.else_content = Some((p, c)),
                ParsedMeta::NameValue(ParsedMetaNameValue::StateBound(p, s)) => {
                    attrs.state_bound = Some((p, s))
                }
            }
        }
        attrs
    }
}
