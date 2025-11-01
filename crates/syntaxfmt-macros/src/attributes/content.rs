use std::fmt::Debug;

use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{Expr, ExprClosure, Ident, Result as SynResult, TypePath};

use crate::{
    attributes::{args::CommonArgs, seps::PopSeps, eval::Eval, pretty::PopIndentRegion},
    syn_err,
};

pub trait Skipped {
    fn skipped(&self) -> bool;
}

pub trait WithCommon {
    // I don't like this cyclic dependency on args::CommonArgs, but whatever
    fn common(&self) -> &CommonArgs;
}

pub trait WithEval {
    fn eval(&self) -> &Option<Eval>;
}

pub trait WithConditional {
    type Normal: WithCommon + WithEval;
    type Else: WithCommon;

    fn conditional(&self) -> (&Self::Normal, &Option<Self::Else>);
}

#[derive(Debug, Clone)]
pub enum FieldKind {
    SelfValue,
    Field(Ident),
}

impl ToTokens for FieldKind {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            FieldKind::SelfValue => tokens.extend(quote! { self }),
            FieldKind::Field(ident) => ident.to_tokens(tokens),
        }
    }
}

pub trait ToContentTokens {
    fn to_content_tokens(&self, field_kind: FieldKind, default_content: &Content) -> TokenStream2;
}

impl<T> ToContentTokens for T
where
    T: WithCommon,
{
    fn to_content_tokens(&self, field_kind: FieldKind, default_content: &Content) -> TokenStream2 {
        let common = self.common();

        let prefix = &common.prefix;
        let suffix = &common.suffix;

        let (push_seps, pop_seps) = if let Some(seps) = &common.seps {
            (Some(seps), Some(PopSeps))
        } else {
            Default::default()
        };

        let content = common
            .content
            .as_ref()
            .unwrap_or(&default_content)
            .to_tokens(&field_kind);

        let (push_indent, pop_indent) = if let Some(indent) = &common.indent {
            (Some(indent), Some(PopIndentRegion))
        } else {
            Default::default()
        };

        let nl_begin = common.nl.into_beg();
        let nl_prefix = common.nl.into_pre();
        let nl_content = common.nl.into_cont();
        let nl_suffix = common.nl.into_suf();

        // Push and pop indent has to be in non-symmetric location
        // This is because indenting is non-symmetric
        let pre = quote! { #nl_begin #prefix #push_indent #nl_prefix #push_seps };
        let post = quote! { #pop_seps #pop_indent #nl_content #suffix #nl_suffix };

        quote! { #pre #content #post }
    }
}

pub trait ToConditionalTokens {
    fn to_conditional_tokens(
        &self,
        field_kind: FieldKind,
        default_content: &Content,
    ) -> TokenStream2;
}

impl<T, N, E> ToConditionalTokens for T
where
    T: WithConditional<Normal = N, Else = E>,
    N: ToContentTokens + WithEval,
    E: ToContentTokens,
{
    fn to_conditional_tokens(
        &self,
        field_kind: FieldKind,
        default_content: &Content,
    ) -> TokenStream2 {
        let (args, args_else) = self.conditional();

        let content = args.to_content_tokens(field_kind.clone(), default_content);
        let content_else = args_else
            .as_ref()
            .map(|a| a.to_content_tokens(field_kind.clone(), default_content));

        let eval = args.eval().as_ref().map(|e| e.to_tokens(&field_kind));

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

#[derive(Debug, Clone)]
pub enum Content {
    Expr(Expr),
    TypePath(TypePath),
    Closure(ExprClosure),
    Tokens(TokenStream2),
}

impl Content {
    #[must_use]
    pub fn from_expr(expr: Expr) -> SynResult<Option<Self>> {
        Ok(Some(match expr {
            // Interpreted as items which implement SyntaxFmt
            e @ Expr::Binary(_) => Self::Expr(e),
            e @ Expr::Cast(_) => Self::Expr(e),
            e @ Expr::Const(_) => Self::Expr(e),
            e @ Expr::Field(_) => Self::Expr(e),
            e @ Expr::Index(_) => Self::Expr(e),
            e @ Expr::Lit(_) => Self::Expr(e),
            e @ Expr::Macro(_) => Self::Expr(e),
            e @ Expr::Range(_) => Self::Expr(e),
            e @ Expr::Reference(_) => Self::Expr(e),
            e @ Expr::Tuple(_) => Self::Expr(e),
            e @ Expr::Unary(_) => Self::Expr(e),
            e @ Expr::Repeat(_) => Self::Expr(e),

            // Interpreted as items which return items which implement SyntaxFmt
            e @ Expr::Call(_) => Self::Expr(e),
            e @ Expr::MethodCall(_) => Self::Expr(e),

            // Interpreted as items which must be written directly as strings
            e @ Expr::Array(_) => Self::Expr(e),

            e => {
                return syn_err(e, "syntaxfmt unsupported content expression");
            }
        }))
    }

    #[must_use]
    pub fn from_type_path(value: TypePath) -> SynResult<Option<Self>> {
        Ok(Some(Self::TypePath(value)))
    }

    #[must_use]
    pub fn from_closure(value: ExprClosure) -> SynResult<Option<Self>> {
        Ok(Some(Self::Closure(value)))
    }
}

impl Content {
    pub fn to_tokens(&self, field: &impl ToTokens) -> TokenStream2 {
        match self {
            // Interpreted as items which implement SyntaxFmt
            Self::Expr(Expr::Binary(e)) => quote! { (#e).syntax_fmt(f)?; },
            Self::Expr(Expr::Cast(e)) => quote! { (#e).syntax_fmt(f)?; },
            Self::Expr(Expr::Const(e)) => quote! { (#e).syntax_fmt(f)?; },
            Self::Expr(Expr::Field(e)) => quote! { (#e).syntax_fmt(f)?; },
            Self::Expr(Expr::Index(e)) => quote! { (#e).syntax_fmt(f)?; },
            Self::Expr(Expr::Lit(e)) => quote! { (#e).syntax_fmt(f)?; },
            Self::Expr(Expr::Macro(e)) => quote! { (#e).syntax_fmt(f)?; },
            Self::Expr(Expr::Range(e)) => quote! { (#e).syntax_fmt(f)?; },
            Self::Expr(Expr::Reference(e)) => quote! { (#e).syntax_fmt(f)?; },
            Self::Expr(Expr::Tuple(e)) => quote! { (#e).syntax_fmt(f)?; },
            Self::Expr(Expr::Unary(e)) => quote! { (#e).syntax_fmt(f)?; },
            Self::Expr(Expr::Repeat(e)) => quote! { (#e).syntax_fmt(f)?; },

            // Interpreted as items which return items which implement SyntaxFmt
            Self::Expr(Expr::Call(e)) => quote! { (#e).syntax_fmt(f)?; },
            Self::Expr(Expr::MethodCall(e)) => quote! { (#e).syntax_fmt(f)?; },

            // Interpreted as items which must be written directly as strings
            Self::Expr(Expr::Array(e)) => quote! { f.write_strs(#e)?; },

            Self::TypePath(p) => quote! { (#p)(#field, f)?; },
            Self::Closure(p) => quote! { (#p)(#field, f)?; },

            Self::Tokens(t) => t.clone(),

            _ => TokenStream2::new(),
        }
    }
}
