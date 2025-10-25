use std::fmt::Debug;

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote_spanned, ToTokens};
use syn::{Expr, spanned::Spanned};

use crate::{
    components::
        parse_basic::ParseBasic
    , SyntaxError
};

#[cfg(feature = "trace")]
use crate::{trace, DEPTH};

#[derive(Debug, Clone)]
pub enum Content {
    Expr(Expr),
    Tokens(TokenStream2),
}

impl<'a> ParseBasic<'a> for Content {
    type Input = Expr;

    #[cfg_attr(feature = "trace", trace)]
    fn parse_basic(input: &'a Self::Input) -> Result<Self, SyntaxError> {
        Ok(match input.clone() {
            e @ Expr::Call(..) => Self::Expr(e),
            e @ Expr::Macro(..) => Self::Expr(e),
            e @ Expr::Closure(..) => Self::Expr(e),
            e @ Expr::Path(..) => Self::Expr(e),
            e @ Expr::Lit(..) => Self::Expr(e),
            e @ Expr::Tuple(..) => Self::Expr(e),
            e => return Err(SyntaxError::UnexpectedContentExpr(e)),
        })
    }
}

impl ToTokens for Content {
    #[cfg_attr(feature = "trace", trace)]
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Expr(Expr::Call(inner)) => {
                let span = inner.span();
                tokens.extend(quote_spanned! { span => (#inner)?; });
            }
            Self::Expr(Expr::Macro(inner)) => {
                let span = inner.span();
                tokens.extend(quote_spanned! { span => (#inner)?; });
            }
            Self::Expr(Expr::Closure(inner)) => {
                let span = inner.span();
                tokens.extend(quote_spanned! { span => (#inner)(field, f)?; });
            }
            Self::Expr(Expr::Path(inner)) => {
                let span = inner.span();
                tokens.extend(quote_spanned! { span => (#inner)(field, f)?; });
            }
            Self::Expr(Expr::Lit(inner)) => {
                let span = inner.span();
                tokens.extend(quote_spanned! { span => (#inner).syntax_fmt(f)?; });
            }
            Self::Expr(Expr::Tuple(inner)) => {
                let span = inner.span();
                tokens.extend(quote_spanned! { span => (#inner).syntax_fmt(f)?; });
            }
            Self::Tokens(t) => tokens.extend(t.clone()),
            _ => {}
        }
    }
}
