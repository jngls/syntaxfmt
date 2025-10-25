use quote::{quote_spanned, ToTokens};
use syn::{spanned::Spanned, Expr};

use crate::{components::parse_basic::ParseBasic, SyntaxError};

#[derive(Debug, Clone)]
pub struct Eval(pub Expr);

impl<'a> ParseBasic<'a> for Eval {
    type Input = Expr;

    #[cfg_attr(feature = "trace", trace)]
    fn parse_basic(input: &'a Self::Input) -> Result<Self, SyntaxError> {
        Ok(match input.clone() {
            e @ Expr::Call(..) => Self(e),
            e @ Expr::Macro(..) => Self(e),
            e @ Expr::Unary(..) => Self(e),
            e @ Expr::Closure(..) => Self(e),
            e @ Expr::Path(..) => Self(e),
            e @ Expr::Lit(..) => Self(e),
            e => return Err(SyntaxError::UnexpectedConditionalExpr(e)),
        })
    }
}

impl ToTokens for Eval {
    #[cfg_attr(feature = "trace", trace)]
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self(Expr::Call(inner)) => {
                let span = inner.span();
                tokens.extend(quote_spanned! { span => (#inner) });
            }
            Self(Expr::Macro(inner)) => {
                let span = inner.span();
                tokens.extend(quote_spanned! { span => (#inner) });
            }
            Self(Expr::Unary(inner)) => {
                let span = inner.span();
                tokens.extend(quote_spanned! { span => (#inner) });
            }
            Self(Expr::Closure(inner)) => {
                let span = inner.span();
                tokens.extend(quote_spanned! { span => (#inner)(field) });
            }
            Self(Expr::Path(inner)) => {
                let span = inner.span();
                tokens.extend(quote_spanned! { span => (#inner)(field) });
            }
            Self(Expr::Lit(inner)) => {
                let span = inner.span();
                tokens.extend(quote_spanned! { span => (#inner) });
            }
            _ => {}
        }
    }
}
