use syn::{Expr, ExprLit, Lit};

use crate::SyntaxError;

pub trait ParseTokens: Sized {
    type Input: ?Sized;

    fn parse_tokens(input: &Self::Input) -> Result<Self, SyntaxError>;
}

impl ParseTokens for String {
    type Input = Expr;

    fn parse_tokens(expr: &Self::Input) -> Result<Self, SyntaxError> {
        if let Expr::Lit(ExprLit {
            lit: Lit::Str(s), ..
        }) = expr
        {
            Ok(s.value())
        } else {
            Err(SyntaxError::ExpectedStringLit(expr.clone()))
        }
    }
}
