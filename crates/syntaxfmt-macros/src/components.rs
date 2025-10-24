
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{Expr, ExprLit, ExprTuple, Lit};

use crate::SyntaxError;

pub trait StringParse: Sized {
    fn parse(expr: &Expr) -> Result<Self, SyntaxError>;
    fn into_split_format(self) -> (Self, Self, bool);
}

impl StringParse for String {
    fn parse(expr: &Expr) -> Result<Self, SyntaxError> {
        if let Expr::Lit(ExprLit {
            lit: Lit::Str(s), ..
        }) = expr
        {
            Ok(s.value())
        } else {
            Err(SyntaxError::ExpectedStringLit(expr.clone()))
        }
    }

    fn into_split_format(self) -> (Self, Self, bool) {
        if let Some(pos) = self.find("{content}") {
            (self[..pos].into(), self[pos + 9..].into(), true)
        } else {
            (self.clone(), self, false)
        }
    }
}

#[derive(Default)]
pub struct DualString {
    pub normal: String,
    pub pretty: String,
}

impl DualString {
    pub fn parse(expr: &Expr) -> Result<Self, SyntaxError> {
        match expr {
            Expr::Lit(ExprLit {
                lit: Lit::Str(s), ..
            }) => {
                let normal = s.value();
                let pretty = normal.clone();
                Ok(Self { normal, pretty })
            }
            Expr::Tuple(ExprTuple { elems, .. }) => {
                let mut elem = elems.iter();
                let normal = elem
                    .next()
                    .ok_or(SyntaxError::ExpectedStringLitOrDual(expr.clone()))
                    .and_then(|e| String::parse(e))?;
                let pretty = elem
                    .next()
                    .ok_or(SyntaxError::ExpectedStringLitOrDual(expr.clone()))
                    .and_then(|e| String::parse(e))?;
                Ok(Self { normal, pretty })
            }
            _ => Err(SyntaxError::ExpectedStringLitOrDual(expr.clone())),
        }
    }

    pub fn into_format(self) -> Format {
        let (n_prefix, n_suffix, n_content) = self.normal.into_split_format();
        let (p_prefix, p_suffix, p_content) = self.pretty.into_split_format();
        Format {
            prefix: DualString {
                normal: n_prefix.into(),
                pretty: p_prefix.into(),
            },
            suffix: DualString {
                normal: n_suffix.into(),
                pretty: p_suffix.into(),
            },
            has_content: DualBool { normal: n_content, pretty: p_content },
        }
    }
}

impl ToTokens for DualString {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let normal = &self.normal;
        let pretty = &self.pretty;
        tokens.extend(quote! {
            f.write_dual_str(syntaxfmt::DualStr::new(#normal, #pretty))?;
        });
    }
}

#[derive(Default, Clone, Copy)]
pub struct DualBool {
    pub normal: bool,
    pub pretty: bool,
}

pub trait SplitFormat {
    type Side;

    fn split_format(&self) -> (Self::Side, Self::Side, DualBool);
}

pub struct Format {
    pub prefix: DualString,
    pub suffix: DualString,
    pub has_content: DualBool,
}

impl<'a> SplitFormat for Option<&'a Format> {
    type Side = Option<&'a DualString>;

    fn split_format(&self) -> (Self::Side, Self::Side, DualBool) {
        match self {
            Some(Format { prefix, suffix, has_content }) => (Some(prefix), Some(suffix), *has_content),
            None => Default::default(),
        }
    }
}

pub struct PushDelims(pub DualString);

impl PushDelims {
    pub fn parse(expr: &Expr) -> Result<Self, SyntaxError> {
        Ok(Self(DualString::parse(expr)?))
    }
}

impl ToTokens for PushDelims {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let normal = &self.0.normal;
        let pretty = &self.0.pretty;
        tokens.extend(quote! {
            f.push_delim(syntaxfmt::DualStr::new(#normal, #pretty));
        });
    }
}

pub struct PopDelims;

impl ToTokens for PopDelims {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.extend(quote! {
            f.pop_delim();
        });
    }
}

pub struct PushIndentRegion;

impl ToTokens for PushIndentRegion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.extend(quote! {
            f.push_indent();
        });
    }
}

pub struct PopIndentRegion;

impl ToTokens for PopIndentRegion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.extend(quote! {
            f.pop_indent();
        });
    }
}

pub struct WriteIndent;

impl ToTokens for WriteIndent {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.extend(quote! {
            f.write_indent()?;
        });
    }
}

pub struct SetNone(pub DualString);

impl SetNone {
    pub fn parse(expr: &Expr) -> Result<Self, SyntaxError> {
        Ok(Self(DualString::parse(expr)?))
    }
}

impl ToTokens for SetNone {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let normal = &self.0.normal;
        let pretty = &self.0.pretty;
        tokens.extend(quote! {
            f.set_none(syntaxfmt::DualStr::new(#normal, #pretty));
        });
    }
}

pub struct ClearNone;

impl ToTokens for ClearNone {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.extend(quote! {
            f.clear_none();
        });
    }
}

pub struct WriteContent(pub Expr);

impl WriteContent {
    pub fn parse(expr: &Expr) -> Result<Self, SyntaxError> {
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

impl ToTokens for WriteContent {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.extend(match &self.0 {
            Expr::Call(inner) => quote!{ (#inner)?; },
            Expr::Macro(inner) => quote!{ (#inner)?; },
            Expr::Closure(inner) => quote!{ (#inner)(f)?; },
            Expr::Path(inner) => quote!{ (#inner)(f)?; },
            Expr::Lit(inner) => quote!{ (#inner).syntax_fmt(f)?; },
            Expr::Tuple(inner) => quote!{ (#inner).syntax_fmt(f)?; },
            _ => TokenStream2::new(),
        });
    }
}

pub struct SyntaxTypePreContent<'a> {
    pub write_prefix: Option<&'a DualString>,
    pub push_delim: Option<&'a PushDelims>,
    pub push_indent: Option<&'a PushIndentRegion>,
    pub write_indent: Option<&'a WriteIndent>,
}

impl<'a> ToTokens for SyntaxTypePreContent<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let write_prefix = self.write_prefix;
        let write_indent = self.write_indent;
        let push_delim = self.push_delim;
        let push_indent = self.push_indent;
        tokens.extend(quote! {
            #write_indent
            #write_prefix
            #push_delim
            #push_indent
        });
    }
}

pub struct SyntaxTypePostContent<'a> {
    pub write_suffix: Option<&'a DualString>,
    pub pop_delim: Option<PopDelims>,
    pub pop_indent: Option<PopIndentRegion>,
}

impl<'a> ToTokens for SyntaxTypePostContent<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let write_suffix = self.write_suffix;
        let pop_delim = self.pop_delim.as_ref();
        let pop_indent = self.pop_indent.as_ref();
        tokens.extend(quote! {
            #pop_indent
            #pop_delim
            #write_suffix
        });
    }
}

pub struct SyntaxFieldPreContent<'a> {
    pub write_prefix: Option<&'a DualString>,
    pub push_delim: Option<&'a PushDelims>,
    pub push_indent: Option<&'a PushIndentRegion>,
    pub write_indent: Option<&'a WriteIndent>,
    pub set_none: Option<&'a SetNone>,
}

impl<'a> ToTokens for SyntaxFieldPreContent<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let write_prefix = self.write_prefix;
        let write_indent = self.write_indent;
        let push_delim = self.push_delim;
        let push_indent = self.push_indent;
        let set_none = self.set_none;
        tokens.extend(quote! {
            #write_indent
            #write_prefix
            #push_delim
            #push_indent
            #set_none
        });
    }
}

pub struct SyntaxFieldPostContent<'a> {
    pub write_suffix: Option<&'a DualString>,
    pub pop_delim: Option<PopDelims>,
    pub pop_indent: Option<PopIndentRegion>,
    pub clear_none: Option<ClearNone>,
}

impl<'a> ToTokens for SyntaxFieldPostContent<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let write_suffix = self.write_suffix;
        let pop_delim = self.pop_delim.as_ref();
        let pop_indent = self.pop_indent.as_ref();
        let clear_none = self.clear_none.as_ref();
        tokens.extend(quote! {
            #clear_none
            #pop_indent
            #pop_delim
            #write_suffix
        });
    }
}
