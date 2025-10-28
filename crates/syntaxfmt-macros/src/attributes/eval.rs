use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    Error as SynError, Expr, ExprClosure, Result as SynResult, TypePath

};

use crate::syn_err;

#[derive(Debug, Clone)]
pub enum Eval {
    Expr(Expr),
    TypePath(TypePath),
    Closure(ExprClosure),
}

impl Eval {
    #[must_use]
    pub fn from_expr(value: Expr) -> SynResult<Option<Self>> {
        Ok(Some(match value {
            e @ Expr::Binary(_) => Self::Expr(e),
            e @ Expr::Cast(_) => Self::Expr(e),
            e @ Expr::Const(_) => Self::Expr(e),
            e @ Expr::Field(_) => Self::Expr(e),
            e @ Expr::Index(_) => Self::Expr(e),
            e @ Expr::Lit(_) => Self::Expr(e),
            e @ Expr::Macro(_) => Self::Expr(e),
            e @ Expr::Unary(_) => Self::Expr(e),
            e @ Expr::Call(_) => Self::Expr(e),
            e @ Expr::MethodCall(_) => Self::Expr(e),

            e => return syn_err(e, "syntaxfmt unsupported eval expression"),
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

impl Eval {
    pub fn to_tokens(&self, field: &impl ToTokens) -> TokenStream2 {
        match self {
            Self::Expr(Expr::Binary(e)) => quote! { (#e) },
            Self::Expr(Expr::Cast(e)) => quote! { (#e) },
            Self::Expr(Expr::Const(e)) => quote! { (#e) },
            Self::Expr(Expr::Field(e)) => quote! { (#e) },
            Self::Expr(Expr::Index(e)) => quote! { (#e) },
            Self::Expr(Expr::Lit(e)) => quote! { (#e) },
            Self::Expr(Expr::Macro(e)) => quote! { (#e) },
            Self::Expr(Expr::Unary(e)) => quote! { (#e) },
            Self::Expr(Expr::Call(e)) => quote! { (#e) },
            Self::Expr(Expr::MethodCall(e)) => quote! { (#e) },

            Self::TypePath(p) => quote! { (#p)(#field) },
            Self::Closure(p) => quote! { (#p)(#field) },

            _ => TokenStream2::new()
        }
    }
}
