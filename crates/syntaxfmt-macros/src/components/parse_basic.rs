use syn::{Expr, ExprLit, Lit};

use crate::SyntaxError;

#[cfg(feature = "trace")]
use crate::{trace, DEPTH};

pub trait ParseBasic<'a>: Sized {
    type Input: ?Sized;

    fn parse_basic(input: &'a Self::Input) -> Result<Self, SyntaxError>;
}

impl<'a> ParseBasic<'a> for String {
    type Input = Expr;

    #[cfg_attr(feature = "trace", trace)]
    fn parse_basic(input: &'a Self::Input) -> Result<Self, SyntaxError> {
        if let Expr::Lit(ExprLit {
            lit: Lit::Str(s), ..
        }) = input
        {
            Ok(s.value())
        } else {
            Err(SyntaxError::ExpectedStringLit(input.clone()))
        }
    }
}
