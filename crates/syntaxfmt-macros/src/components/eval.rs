use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::Expr;

use crate::{components::parse_basic::ParseBasic, SyntaxError};

#[derive(Debug, Clone)]
pub struct Eval(pub Expr);

impl<'a> ParseBasic<'a> for Eval {
    type Input = Expr;

    #[cfg_attr(feature = "trace", trace)]
    fn parse_basic(input: &'a Self::Input) -> Result<Self, SyntaxError> {
        Ok(match input.clone() {
            // Interpreted as items which result in bool
            e @ Expr::Binary(_) => Self(e),
            e @ Expr::Cast(_) => Self(e),
            e @ Expr::Const(_) => Self(e),
            e @ Expr::Field(_) => Self(e),
            e @ Expr::Index(_) => Self(e),
            e @ Expr::Lit(_) => Self(e),
            e @ Expr::Macro(_) => Self(e),
            e @ Expr::Unary(_) => Self(e),

            // Interpreted as items which return bool
            e @ Expr::Call(_) => Self(e),
            e @ Expr::MethodCall(_) => Self(e),

            // Interpreted as items which must be called with field and return bool
            e @ Expr::Closure(_) => Self(e),
            e @ Expr::Path(_) => Self(e),

            e => return Err(SyntaxError::UnsupportedConditionalExpr(e)),
        })
    }
}

impl Eval {
    #[cfg_attr(feature = "trace", trace)]
    pub fn to_tokens(&self, field: &impl ToTokens) -> TokenStream2 {
        match self {
            // Interpreted as items which result in bool
            Self(Expr::Binary(e)) => quote! { (#e) },
            Self(Expr::Cast(e)) => quote! { (#e) },
            Self(Expr::Const(e)) => quote! { (#e) },
            Self(Expr::Field(e)) => quote! { (#e) },
            Self(Expr::Index(e)) => quote! { (#e) },
            Self(Expr::Lit(e)) => quote! { (#e) },
            Self(Expr::Macro(e)) => quote! { (#e) },
            Self(Expr::Unary(e)) => quote! { (#e) },

            // Interpreted as items which return bool
            Self(Expr::Call(e)) => quote! { (#e) },
            Self(Expr::MethodCall(e)) => quote! { (#e) },

            // Interpreted as items which must be called with field and return bool
            Self(Expr::Closure(e)) => quote! { (#e)(#field) },
            Self(Expr::Path(e)) => quote! { (#e)(#field) },

            _ => TokenStream2::new()
        }
    }
}
