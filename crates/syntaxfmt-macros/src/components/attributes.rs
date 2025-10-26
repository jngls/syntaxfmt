use std::fmt::Debug;

use proc_macro2::TokenStream as TokenStream2;

use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, token::Comma, Attribute, Expr, ExprPath, Meta, MetaNameValue, Path};

use crate::{
    components::{
        content::Content, delims::{PopDelims, PushDelims}, eval::Eval, format::Format, indent::{PopIndentRegion, PushIndentRegion, WriteNewline}, parse_basic::ParseBasic
    }, SyntaxError
};

#[derive(Debug, Clone)]
enum Newline {
    Begin,
    Prefix,
    Content,
    Suffix,
}

impl<'a> ParseBasic<'a> for Newline {
    type Input = Expr;

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
    Skip(Path),
    Newline,
}

impl<'a> ParseBasic<'a> for ParsedMetaPath {
    type Input = Path;

    fn parse_basic(input: &'a Self::Input) -> Result<Self, SyntaxError> {
        if input.is_ident("ind") {
            Ok(Self::Indent)
        } else if input.is_ident("nl") {
            Ok(Self::Newline)
        } else if input.is_ident("skip") {
            Ok(Self::Skip(input.clone()))
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
    StateBound(Path, ExprPath),
    State(Path, ExprPath),
    Newlines(Newlines),
}

impl<'a> ParseBasic<'a> for ParsedMetaNameValue {
    type Input = MetaNameValue;

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

fn parse_metas<'a>(metas: &mut Vec<ParsedMeta>, meta: &'a Meta) -> Result<(), SyntaxError> {
    match meta.require_list() {
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
    }
}

#[derive(Debug, Clone)]
struct ParsedAttributes(pub Vec<ParsedMeta>);

impl<'a> ParseBasic<'a> for ParsedAttributes {
    type Input = [Attribute];

    fn parse_basic(input: &'a Self::Input) -> Result<Self, SyntaxError> {
        let mut metas = Vec::new();
        for attr in input {
            if attr.path().is_ident("syntax") {
                parse_metas(&mut metas, &attr.meta)?;
            }
        }
        Ok(Self(metas))
    }
}

#[derive(Debug, Clone)]
struct ParsedAttributesElse(pub Vec<ParsedMeta>);

impl<'a> ParseBasic<'a> for ParsedAttributesElse {
    type Input = [Attribute];

    fn parse_basic(input: &'a Self::Input) -> Result<Self, SyntaxError> {
        let mut metas = Vec::new();
        for attr in input {
            if attr.path().is_ident("syntax_else") {
                parse_metas(&mut metas, &attr.meta)?;
            }
        }
        Ok(Self(metas))
    }
}

#[derive(Debug, Default, Clone)]
pub struct Attributes {
    pub skip: Option<Path>,
    pub format: Option<Format>,
    pub delims: Option<PushDelims>,
    pub eval: Option<Eval>,
    pub content: Option<Content>,
    pub state_bound: Option<(Path, ExprPath)>,
    pub state: Option<(Path, ExprPath)>,
    pub indent: bool,
    pub nl_begin: bool,
    pub nl_prefix: bool,
    pub nl_content: bool,
    pub nl_suffix: bool,
}

impl Attributes {
    fn from_parsed_metas(value: Vec<ParsedMeta>) -> Self {
        let mut attrs = Self::default();
        for parsed_meta in value {
            match parsed_meta {
                ParsedMeta::Path(ParsedMetaPath::Indent) => attrs.indent = true,
                ParsedMeta::Path(ParsedMetaPath::Newline) => attrs.nl_suffix = true,
                ParsedMeta::Path(ParsedMetaPath::Skip(path)) => attrs.skip = Some(path),
                ParsedMeta::NameValue(ParsedMetaNameValue::Format(f)) => attrs.format = Some(f),
                ParsedMeta::NameValue(ParsedMetaNameValue::Delims(d)) => attrs.delims = Some(d),
                ParsedMeta::NameValue(ParsedMetaNameValue::Eval(c)) => attrs.eval = Some(c),
                ParsedMeta::NameValue(ParsedMetaNameValue::Content(c)) => attrs.content = Some(c),
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

    #[allow(private_interfaces)]
    pub fn parse_for_type<'a>(input: &'a [Attribute]) -> Result<Self, SyntaxError> {
        Ok(ParsedAttributes::parse_basic(input)?.into())
    }

    #[allow(private_interfaces)]
    pub fn parse_for_type_else<'a>(input: &'a [Attribute]) -> Result<Option<Self>, SyntaxError> {
        match ParsedAttributesElse::parse_basic(input)?.into() {
            Some(attrs_else) => {
                let mut attrs_else: Attributes = attrs_else;
                // TODO detect missing eval in normal attributes and emit ExpectedCondition
                if let Some(eval) = attrs_else.eval.take() {
                    Err(SyntaxError::UnexpectedConditionalExpr(eval.0))?
                }
                if let Some(path) = attrs_else.skip.take() {
                    Err(SyntaxError::UnexpectedAttributeArg(path))?
                }
                Ok(Some(attrs_else))
            }
            None => Ok(None),
        }
    }

    #[allow(private_interfaces)]
    pub fn parse_for_field<'a>(input: &'a [Attribute]) -> Result<Self, SyntaxError> {
        let mut attrs: Attributes = ParsedAttributes::parse_basic(input)?.into();
        if let Some((path, _)) = attrs.state_bound.take() {
            Err(SyntaxError::UnexpectedAttributeArg(path))?
        }
        if let Some((path, _)) = attrs.state.take() {
            Err(SyntaxError::UnexpectedAttributeArg(path))?
        }
        Ok(attrs)
    }

    #[allow(private_interfaces)]
    pub fn parse_for_field_else<'a>(input: &'a [Attribute]) -> Result<Option<Self>, SyntaxError> {
        match ParsedAttributesElse::parse_basic(input)?.into() {
            Some(attrs_else) => {
                let mut attrs_else: Attributes = attrs_else;
                // TODO detect missing eval in normal attributes and emit ExpectedCondition
                if let Some(eval) = attrs_else.eval.take() {
                    Err(SyntaxError::UnexpectedConditionalExpr(eval.0))?
                }
                if let Some(path) = attrs_else.skip.take() {
                    Err(SyntaxError::UnexpectedAttributeArg(path))?
                }
                if let Some((path, _)) = attrs_else.state_bound.take() {
                    Err(SyntaxError::UnexpectedAttributeArg(path))?
                }
                if let Some((path, _)) = attrs_else.state.take() {
                    Err(SyntaxError::UnexpectedAttributeArg(path))?
                }
                Ok(Some(attrs_else))
            }
            None => Ok(None),
        }
    }

    fn to_tokens(&self, field: &impl ToTokens, default_content: &Content) -> TokenStream2 {
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

        let content = self.content.as_ref().unwrap_or(&default_content).to_tokens(field);

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
        let pre = quote! { #nl_begin #prefix #push_indent #nl_prefix #push_delims };
        let post = quote! { #pop_delims #pop_indent #nl_content #suffix #nl_suffix };

        quote! { #pre #content #post }
    }

    pub fn to_conditional_tokens(attrs: &Self, attrs_else: &Option<Self>, field: &impl ToTokens, default_content: &Content) -> TokenStream2 {
        let content = attrs.to_tokens(field, default_content);
        let content_else = attrs_else.as_ref().map(|a| a.to_tokens(field, default_content));

        let eval = attrs.eval.as_ref().map(|c| c.to_tokens(field));

        match (eval, content_else) {
            (Some(eval), Some(content_else)) => quote! {
                if #eval {
                    #content
                } else {
                    #content_else
                }
            },
            (Some(eval), None) => quote! {
                if #eval {
                    #content
                }
            },
            _ => content,
        }
    }
}

impl From<ParsedAttributes> for Attributes {
    fn from(value: ParsedAttributes) -> Self {
        Self::from_parsed_metas(value.0)
    }
}

impl From<ParsedAttributesElse> for Option<Attributes> {
    fn from(value: ParsedAttributesElse) -> Self {
        (!value.0.is_empty()).then(|| Attributes::from_parsed_metas(value.0))
    }
}
