use syn::{Result as SynResult, Type};

pub trait ParseType<'a>: Sized {
    type Input: ?Sized;

    fn parse_type(types: &mut Vec<&'a Type>, input: &'a Self::Input) -> SynResult<Self>;
}
