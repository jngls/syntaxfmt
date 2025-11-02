use std::fmt::Debug;

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{Expr, ExprClosure, Result as SynResult, TypePath};

use crate::{
    attributes::{
        args::CommonArgs,
        context::{PopContext, PushContext},
        eval::Eval, pretty::Newlines,
    },
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

pub trait ToContentTokens {
    fn to_content_tokens(&self, default_content: &Content) -> TokenStream2;
}

impl<T> ToContentTokens for T
where
    T: WithCommon,
{
    fn to_content_tokens(&self, default_content: &Content) -> TokenStream2 {
        let common = self.common();

        let prefix = &common.prefix;
        let suffix = &common.suffix;

        let nl_beg = common.nl.into_beg();
        let nl_pre = common.nl.into_pre();
        let nl_cont = common.nl.into_cont();
        let nl_suf = common.nl.into_suf();

        let push_context = PushContext {
            sep: common.seps.clone(),
            indent: common.indent,
            nl_sep: common.nl.has(Newlines::SEP),
        };
        let pop_context = PopContext;

        let content = common
            .content
            .as_ref()
            .unwrap_or(&default_content)
            .to_tokens(&common.field_kind);

        // Push and pop context has to be in non-symmetric location
        // This is because indenting is non-symmetric
        let pre = quote! { #nl_beg #prefix #push_context #nl_pre };
        let post = quote! { #pop_context #nl_cont #suffix #nl_suf };

        quote! { #pre #content #post }
    }
}

pub trait ToConditionalTokens {
    fn to_conditional_tokens(&self, default_content: &Content) -> TokenStream2;
}

impl<T, N, E> ToConditionalTokens for T
where
    T: WithConditional<Normal = N, Else = E>,
    N: ToContentTokens + WithCommon + WithEval,
    E: ToContentTokens,
{
    fn to_conditional_tokens(&self, default_content: &Content) -> TokenStream2 {
        let (args, args_else) = self.conditional();

        let content = args.to_content_tokens(default_content);
        let content_else = args_else
            .as_ref()
            .map(|a| a.to_content_tokens(default_content));

        let eval = args
            .eval()
            .as_ref()
            .map(|e| e.to_tokens(&args.common().field_kind));

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
