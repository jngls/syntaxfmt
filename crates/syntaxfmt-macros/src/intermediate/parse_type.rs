use syn::{Type,    Result as SynResult,
    Error as SynError,
};

pub trait ParseType<'a>: Sized {
    type Input: ?Sized;

    fn parse_type(types: &mut Vec<&'a Type>, input: &'a Self::Input) -> SynResult<Self>;
}

// impl<T> ParseType for T where T: ParseTokens {
//     type Input = Expr;

//     fn parse_type<'a>(_types: &mut Vec<&'a Type>, expr: &Self::Input) -> SynResult<Self> {
//         T::parse_tokens(input)
//     }
// }
