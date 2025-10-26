use std::fmt::Debug;

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::Expr;

use crate::{
    components::
        parse_basic::ParseBasic
    , SyntaxError
};

#[derive(Debug, Clone)]
pub enum Content {
    Expr(Expr),
    Tokens(TokenStream2),
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

            // Interpreted as items which must be called to perform formatting
            Self::Expr(Expr::Closure(e)) => quote! { (#e)(#field, f)?; },
            Self::Expr(Expr::Path(e)) => quote! { (#e)(#field, f)?; },

            Self::Tokens(t) => t.clone(),

            _ => TokenStream2::new(),
        }
    }
}

impl<'a> ParseBasic<'a> for Content {
    type Input = Expr;

    fn parse_basic(input: &'a Self::Input) -> Result<Self, SyntaxError> {
        Ok(match input.clone() {
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

            // Interpreted as items which must be called to perform formatting
            e @ Expr::Closure(_) => Self::Expr(e),
            e @ Expr::Path(_) => Self::Expr(e),

            e => return Err(SyntaxError::UnsupportedContentExpr(e)),
        })
    }
}
