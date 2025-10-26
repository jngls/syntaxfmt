use std::fmt::Debug;

use proc_macro2::TokenStream as TokenStream2;

use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, token::Comma, Attribute, Expr, ExprPath, Meta, MetaNameValue, Path};

use crate::{
    components::{
        content::Content, delims::{PopDelims, PushDelims}, eval::Eval, format::Format, indent::{PopIndentRegion, PushIndentRegion, WriteNewline}, parse_basic::ParseBasic
    }, SyntaxError
};

#[cfg(feature = "trace")]
use crate::{DEPTH, trace};

#[derive(Debug, Clone)]
enum Newline {
    Begin,
    Prefix,
    Content,
    Suffix,
}

impl<'a> ParseBasic<'a> for Newline {
    type Input = Expr;

    #[cfg_attr(feature = "trace", trace)]
    fn parse_basic(input: &'a Self::Input) -> Result<Self, SyntaxError> {
        match input {
            Expr::Path(ExprPath { path, .. }) => {
                if path.is_ident("beg") {
                    Ok(Self::Begin)
                } else if path.is_ident("pre") {
                    Ok(Self::Prefix)
                } else if path.is_ident("con") {
                    Ok(Self::Content)
                } else if path.is_ident("suf") {
                    Ok(Self::Suffix)
                } else {
                    Err(SyntaxError::UnsupportedNewlinePath(path.clone()))
                }
            }
            _ => Err(SyntaxError::UnsupportedNewlineExpr(input.clone())),
        }
    }
}

#[derive(Debug, Clone)]
struct Newlines(Vec<Newline>);

impl<'a> ParseBasic<'a> for Newlines {
    type Input = Expr;

    #[cfg_attr(feature = "trace", trace)]
    fn parse_basic(input: &'a Self::Input) -> Result<Self, SyntaxError> {
        match input {
            Expr::Path(..) => Ok(Self(vec![Newline::parse_basic(input)?])),
            Expr::Array(inner) => {
                let mut newlines = Vec::new();
                for elem in &inner.elems {
                    newlines.push(Newline::parse_basic(elem)?);
                }
                Ok(Self(newlines))
            },
            _ => Err(SyntaxError::UnsupportedNewlineExpr(input.clone())),
        }
    }
}

#[derive(Debug, Clone)]
enum ParsedMetaPath {
    Indent,
    Skip,
    Newline,
}

impl<'a> ParseBasic<'a> for ParsedMetaPath {
    type Input = Path;

    #[cfg_attr(feature = "trace", trace)]
    fn parse_basic(input: &'a Self::Input) -> Result<Self, SyntaxError> {
        if input.is_ident("ind") {
            Ok(Self::Indent)
        } else if input.is_ident("nl") {
            Ok(Self::Newline)
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
    State(Path, ExprPath),
    Newlines(Newlines),
}

impl<'a> ParseBasic<'a> for ParsedMetaNameValue {
    type Input = MetaNameValue;

    #[cfg_attr(feature = "trace", trace)]
    fn parse_basic(input: &'a Self::Input) -> Result<Self, SyntaxError> {
        let path = &input.path;
        let value = &input.value;

        if path.is_ident("fmt") {
            Ok(Self::Format(Format::parse_basic(value)?))
        } else if path.is_ident("delim") {
            Ok(Self::Delims(PushDelims::parse_basic(value)?))
        } else if path.is_ident("eval") {
            Ok(Self::Eval(Eval::parse_basic(value)?))
        } else if path.is_ident("cont") {
            Ok(Self::Content(Content::parse_basic(value)?))
        } else if path.is_ident("else_cont") {
            Ok(Self::ElseContent(path.clone(), Content::parse_basic(value)?))
        } else if path.is_ident("nl") {
            Ok(Self::Newlines(Newlines::parse_basic(value)?))
        } else if path.is_ident("bound") {
            match value {
                Expr::Path(inner) => Ok(Self::StateBound(path.clone(), inner.clone())),
                _ => Err(SyntaxError::ExpectedTraitPath(path.clone()))?,
            }
        } else if path.is_ident("state") {
            match value {
                Expr::Path(inner) => Ok(Self::State(path.clone(), inner.clone())),
                _ => Err(SyntaxError::ExpectedStatePath(path.clone()))?,
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
    pub eval: Option<Eval>,
    pub content: Option<Content>,
    pub else_content: Option<(Path, Content)>,
    pub state_bound: Option<(Path, ExprPath)>,
    pub state: Option<(Path, ExprPath)>,
    pub indent: bool,
    pub nl_begin: bool,
    pub nl_prefix: bool,
    pub nl_content: bool,
    pub nl_suffix: bool,
    pub skip: bool,
}

impl Attributes {
    pub fn parse_for_type<'a>(input: &'a [Attribute]) -> Result<Self, SyntaxError> {
        let mut attrs: Attributes = ParsedAttributes::parse_basic(input)?.into();
        if attrs.eval.is_none() {
            if let Some((path, _)) = attrs.else_content.take() {
                Err(SyntaxError::ExpectedCondition(path))?
            }
        }
        Ok(attrs)
    }

    pub fn parse_for_field<'a>(input: &'a [Attribute]) -> Result<Self, SyntaxError> {
        let mut attrs: Attributes = ParsedAttributes::parse_basic(input)?.into();
        if attrs.eval.is_none() {
            if let Some((path, _)) = attrs.else_content.take() {
                Err(SyntaxError::ExpectedCondition(path))?
            }
        }
        if let Some((path, _)) = attrs.state_bound.take() {
            Err(SyntaxError::UnexpectedAttributeArg(path))?
        }
        if let Some((path, _)) = attrs.state.take() {
            Err(SyntaxError::UnexpectedAttributeArg(path))?
        }
        Ok(attrs)
    }

    pub fn to_tokens(&self, field: &impl ToTokens, default_content: Content) -> TokenStream2 {
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

        let eval = self.eval.as_ref().map(|c| c.to_tokens(field));

        let content = self.content.as_ref().unwrap_or(&default_content).to_tokens(field);
        let else_content = self.else_content.as_ref().map(|(_, c)| c.to_tokens(field));

        let (push_indent, pop_indent) = if self.indent {
            (Some(PushIndentRegion), Some(PopIndentRegion))
        } else {
            Default::default()
        };

        let nl_begin = self.nl_begin.then_some(WriteNewline);
        let nl_prefix = self.nl_prefix.then_some(WriteNewline);
        let nl_content = self.nl_content.then_some(WriteNewline);
        let nl_suffix = self.nl_suffix.then_some(WriteNewline);

        // Push and pop indent has to be in non-symmetric location
        // This is because indenting is non-symmetric
        let pre = quote! { #push_indent #nl_begin #prefix #nl_prefix #push_delims };
        let post = quote! { #pop_indent #nl_content #pop_delims #suffix #nl_suffix };

        match (eval, else_content) {
            (Some(eval), Some(else_content)) => quote! {
                if #eval {
                    #pre #content #post
                } else {
                    // TODO need else_format
                    #else_content
                }
            },
            (Some(eval), None) => quote! {
                if #eval {
                    #pre #content #post
                }
            },
            _ => quote! { #pre #content #post },
        }
    }
}

impl From<ParsedAttributes> for Attributes {
    fn from(value: ParsedAttributes) -> Self {
        let mut attrs = Self::default();
        for parsed_meta in value.0 {
            match parsed_meta {
                ParsedMeta::Path(ParsedMetaPath::Indent) => attrs.indent = true,
                ParsedMeta::Path(ParsedMetaPath::Newline) => attrs.nl_suffix = true,
                ParsedMeta::Path(ParsedMetaPath::Skip) => attrs.skip = true,
                ParsedMeta::NameValue(ParsedMetaNameValue::Format(f)) => attrs.format = Some(f),
                ParsedMeta::NameValue(ParsedMetaNameValue::Delims(d)) => attrs.delims = Some(d),
                ParsedMeta::NameValue(ParsedMetaNameValue::Eval(c)) => attrs.eval = Some(c),
                ParsedMeta::NameValue(ParsedMetaNameValue::Content(c)) => attrs.content = Some(c),
                ParsedMeta::NameValue(ParsedMetaNameValue::ElseContent(p, c)) => attrs.else_content = Some((p, c)),
                ParsedMeta::NameValue(ParsedMetaNameValue::Newlines(nl)) => {
                    for nl in &nl.0 {
                        match nl {
                            Newline::Begin => attrs.nl_begin = true,
                            Newline::Prefix => attrs.nl_prefix = true,
                            Newline::Content => attrs.nl_content = true,
                            Newline::Suffix => attrs.nl_suffix = true,
                        }
                    }
                }
                ParsedMeta::NameValue(ParsedMetaNameValue::StateBound(p, s)) => {
                    attrs.state_bound = Some((p, s))
                }
                ParsedMeta::NameValue(ParsedMetaNameValue::State(p, s)) => {
                    attrs.state = Some((p, s))
                }
            }
        }
        attrs
    }
}
