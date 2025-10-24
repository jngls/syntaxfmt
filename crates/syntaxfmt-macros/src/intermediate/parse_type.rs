use syn::Type;

use crate::SyntaxError;

pub trait ParseType<'a>: Sized {
    type Input: ?Sized;

    fn parse_type(types: &mut Vec<&'a Type>, input: &'a Self::Input) -> Result<Self, SyntaxError>;
}

// impl<T> ParseType for T where T: ParseTokens {
//     type Input = Expr;

//     fn parse_type<'a>(_types: &mut Vec<&'a Type>, expr: &Self::Input) -> Result<Self, SyntaxError> {
//         T::parse_tokens(input)
//     }
// }
