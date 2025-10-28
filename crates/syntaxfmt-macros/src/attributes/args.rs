use std::{
    collections::HashSet,
    fmt::Debug,
};

use syn::{
    Attribute, GenericArgument, Ident, Lifetime, PathArguments, Result as SynResult, TypeParamBound, TypePath, TypeTraitObject, punctuated::Punctuated, token::Comma
};

use crate::{
    attributes::{
        content::{Content, Skipped, WithCommon, WithConditional, WithEval, WithState},
        delims::PushDelims,
        eval::Eval,
        prefix_suffix::{Prefix, Suffix},
        pretty::{Newlines, PushIndentRegion}, unverified_args::{UnverifiedArg, UnverifiedArgKind, UnverifiedArgs},
    },
    syn_err,
};

pub trait TakeArgs: Sized {
    #[must_use]
    fn take_args(self, args: &mut UnverifiedArgs, have_eval: bool) -> SynResult<Self>;

    #[inline]
    #[must_use]
    fn normalise_ident(ident: &Ident) -> String {
        match ident.to_string().as_str() {
            "cont_with" => "cont",
            "eval_with" => "eval",
            s => s,
        }.into()
    }
}

#[derive(Debug, Default, Clone)]
pub struct CommonArgs {
    pub prefix: Option<Prefix>,
    pub suffix: Option<Suffix>,
    pub delims: Option<PushDelims>,
    pub content: Option<Content>,
    pub indent: Option<PushIndentRegion>,
    pub nl: Newlines,
}

impl CommonArgs {
    #[inline]
    fn match_common(arg: &mut UnverifiedArg) -> bool {
        use UnverifiedArgKind::*;
        matches!(
            arg.kind,
            Indent(_)
                | Prefix(_)
                | Suffix(_)
                | Delims(_)
                | Content(_)
                | ContentTypePath(_)
                | ContentClosure(_)
                | Newlines(_)
        )
    }
}

impl TakeArgs for CommonArgs {
    fn take_args(mut self, args: &mut UnverifiedArgs, _have_eval: bool) -> SynResult<Self> {
        use UnverifiedArgKind as Kind;
        let mut visited = HashSet::new();
        for arg in args.args.extract_if(.., Self::match_common) {
            if !visited.insert(Self::normalise_ident(&arg.ident)) {
                return syn_err(&arg.ident, "syntaxfmt found duplicate or conflicting attribute argument");
            }
            match arg.kind {
                Kind::Prefix(i) => self.prefix = Prefix::from_litstrs(i)?,
                Kind::Suffix(i) => self.suffix = Suffix::from_litstrs(i)?,
                Kind::Delims(i) => self.delims = PushDelims::from_litstrs(i)?,
                Kind::Content(i) => self.content = Content::from_expr(i)?,
                Kind::ContentTypePath(i) => self.content = Content::from_type_path(i)?,
                Kind::ContentClosure(i) => self.content = Content::from_closure(i)?,
                Kind::Indent(_) => self.indent = Some(PushIndentRegion),
                Kind::Newlines(i) => self.nl = Newlines::from_idents(i)?,
                _ => unreachable!("match_common should have matched all possibilities"),
            }
        }
        Ok(self)
    }
}

impl WithCommon for CommonArgs {
    fn common(&self) -> &CommonArgs {
        self
    }
}

#[derive(Debug, Default, Clone)]
pub struct TypeArgsNormal {
    pub common: CommonArgs,
    pub eval: Option<Eval>,
    pub state: Option<TypePath>,
    pub state_bound: Option<TypeTraitObject>,
    skip: bool,
}

impl TypeArgsNormal {
    #[inline]
    fn match_args(arg: &mut UnverifiedArg) -> bool {
        use UnverifiedArgKind::*;
        matches!(
            arg.kind,
            Skip(_)
                | Eval(_)
                | EvalTypePath(_)
                | EvalClosure(_)
                | State(_)
                | StateBound(_)
        )
    }
}

impl Skipped for TypeArgsNormal {
    fn skipped(&self) -> bool {
        self.skip
    }
}

impl WithCommon for TypeArgsNormal {
    fn common(&self) -> &CommonArgs {
        &self.common
    }
}

impl WithEval for TypeArgsNormal {
    fn eval(&self) -> &Option<Eval> {
        &self.eval
    }
}

impl TakeArgs for TypeArgsNormal {
    fn take_args(mut self, args: &mut UnverifiedArgs, have_eval: bool) -> SynResult<Self> {
        self.common = self.common.take_args(args, have_eval)?;

        use UnverifiedArgKind as Kind;
        let mut visited = HashSet::new();
        for arg in args.args.extract_if(.., Self::match_args) {
            if !visited.insert(Self::normalise_ident(&arg.ident)) {
                return syn_err(&arg.ident, "syntaxfmt found duplicate or conflicting attribute argument");
            }
            match arg.kind {
                Kind::Eval(i) => self.eval = Eval::from_expr(i)?,
                Kind::EvalTypePath(i) => self.eval = Eval::from_type_path(i)?,
                Kind::EvalClosure(i) => self.eval = Eval::from_closure(i)?,
                Kind::State(i) => self.state = Some(i),
                Kind::StateBound(i) => self.state_bound = Some(i),
                Kind::Skip(_) => self.skip = true,
                _ => unreachable!("match_args should have matched all possibilities"),
            }
        }

        if let Some(unexpected) = args.args.first() {
            return syn_err(&unexpected.ident, "syntaxfmt unexpected attribute argument");
        }

        Ok(self)
    }
}

#[derive(Debug, Default, Clone)]
pub struct TypeArgsElse {
    pub common: CommonArgs,
}

impl TakeArgs for TypeArgsElse {
    fn take_args(mut self, args: &mut UnverifiedArgs, have_eval: bool) -> SynResult<Self> {
        self.common = self.common.take_args(args, have_eval)?;

        if !have_eval {
            // Safety: Safe to unwrap because if we're processing an Else group of args, an ident has been assigned
            return syn_err(
                args.attr.as_ref().unwrap(),
                "syntaxfmt `syntax_else` attribute requires `eval` or `eval_with` in `syntax` attribute",
            );
        }

        if let Some(unexpected) = args.args.first() {
            return syn_err(&unexpected.ident, "syntaxfmt unexpected attribute argument");
        }

        Ok(self)
    }
}

impl WithCommon for TypeArgsElse {
    fn common(&self) -> &CommonArgs {
        &self.common
    }
}

#[derive(Debug, Default, Clone)]
pub struct FieldArgsNormal {
    pub common: CommonArgs,
    pub eval: Option<Eval>,
    skip: bool,
}

impl FieldArgsNormal {
    #[inline]
    fn match_args(arg: &mut UnverifiedArg) -> bool {
        use UnverifiedArgKind::*;
        matches!(
            arg.kind,
            Skip(_)
                | Eval(_)
                | EvalTypePath(_)
                | EvalClosure(_)
        )
    }
}

impl Skipped for FieldArgsNormal {
    fn skipped(&self) -> bool {
        self.skip
    }
}

impl WithCommon for FieldArgsNormal {
    fn common(&self) -> &CommonArgs {
        &self.common
    }
}

impl TakeArgs for FieldArgsNormal {
    fn take_args(mut self, args: &mut UnverifiedArgs, have_eval: bool) -> SynResult<Self> {
        self.common = self.common.take_args(args, have_eval)?;

        use UnverifiedArgKind as Kind;
        let mut visited = HashSet::new();
        for arg in args.args.extract_if(.., Self::match_args) {
            if !visited.insert(Self::normalise_ident(&arg.ident)) {
                return syn_err(&arg.ident, "syntaxfmt found duplicate or conflicting attribute argument");
            }
            match arg.kind {
                Kind::Eval(i) => self.eval = Eval::from_expr(i)?,
                Kind::EvalTypePath(i) => self.eval = Eval::from_type_path(i)?,
                Kind::EvalClosure(i) => self.eval = Eval::from_closure(i)?,
                Kind::Skip(_) => self.skip = true,
                _ => unreachable!("match_args should have matched all possibilities"),
            }
        }

        if let Some(unexpected) = args.args.first() {
            return syn_err(&unexpected.ident, "syntaxfmt unexpected attribute argument");
        }

        Ok(self)
    }
}

impl WithEval for FieldArgsNormal {
    fn eval(&self) -> &Option<Eval> {
        &self.eval
    }
}

#[derive(Debug, Default, Clone)]
pub struct FieldArgsElse {
    pub common: CommonArgs,
}

impl TakeArgs for FieldArgsElse {
    fn take_args(mut self, args: &mut UnverifiedArgs, have_eval: bool) -> SynResult<Self> {
        self.common = self.common.take_args(args, have_eval)?;

        if !have_eval {
            // Safety: Safe to unwrap because if we're processing an Else group of args, an ident has been assigned
            return syn_err(
                args.attr.as_ref().unwrap(),
                "syntaxfmt `syntax_else` attribute requires `eval` or `eval_with` in `syntax` attribute",
            );
        }

        if let Some(unexpected) = args.args.first() {
            return syn_err(&unexpected.ident, "syntaxfmt unexpected attribute argument");
        }

        Ok(self)
    }
}

impl WithCommon for FieldArgsElse {
    fn common(&self) -> &CommonArgs {
        &self.common
    }
}

#[derive(Debug, Default, Clone)]
pub struct TypeArgs {
    pub lifetimes: Vec<Lifetime>,
    pub args: TypeArgsNormal,
    pub args_else: Option<TypeArgsElse>,
}

impl TypeArgs {
    #[must_use]
    pub fn from_attributes(input: &[Attribute]) -> SynResult<Self> {
        let (mut args, args_else) = UnverifiedArgs::collect_args(input)?;
        let mut type_args = Self::default();
        type_args.args = type_args.args.take_args(&mut args, false)?;
        if let Some(mut args_else) = args_else {
            let have_eval = type_args.args.eval.is_some();
            type_args.args_else = Some(TypeArgsElse::default().take_args(&mut args_else, have_eval)?);
        }
        type_args.args.state.as_ref().inspect(|t| Self::type_path_lifetimes(&mut type_args.lifetimes, t));
        type_args.args.state_bound.as_ref().inspect(|t| Self::trait_obj_lifetimes(&mut type_args.lifetimes, t));
        Ok(type_args)
    }

    fn type_path_lifetimes(lifetimes: &mut Vec<Lifetime>, type_path: &TypePath) {
        for segment in &type_path.path.segments {
            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                for arg in &args.args {
                    if let GenericArgument::Lifetime(lt) = arg {
                        lifetimes.push(lt.clone());
                    }
                }
            }
        }
    }

    fn trait_obj_lifetimes(lifetimes: &mut Vec<Lifetime>, trait_obj: &TypeTraitObject) {
        for bound in &trait_obj.bounds {
            if let TypeParamBound::Lifetime(lt) = bound {
                lifetimes.push(lt.clone());
            } else if let TypeParamBound::Trait(trait_bound) = bound {
                for segment in &trait_bound.path.segments {
                    if let PathArguments::AngleBracketed(args) = &segment.arguments {
                        for arg in &args.args {
                            if let GenericArgument::Lifetime(lt) = arg {
                                lifetimes.push(lt.clone());
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Skipped for TypeArgs {
    fn skipped(&self) -> bool {
        self.args.skip
    }
}

impl WithConditional for TypeArgs {
    type Normal = TypeArgsNormal;
    type Else = TypeArgsElse;

    fn conditional(&self) -> (&Self::Normal, &Option<Self::Else>) {
        (&self.args, &self.args_else)
    }
}

impl WithState for TypeArgs {
    fn state(&self) -> (&Option<TypePath>, &Option<TypeTraitObject>) {
        (&self.args.state, &self.args.state_bound)
    }

    fn lifetimes(&self) -> &Vec<Lifetime> {
        &self.lifetimes
    }
}

#[derive(Debug, Default, Clone)]
pub struct FieldArgs {
    pub args: FieldArgsNormal,
    pub args_else: Option<FieldArgsElse>,
}

impl FieldArgs {
    #[must_use]
    pub fn from_attributes(input: &[Attribute]) -> SynResult<Self> {
        let (mut args, args_else) = UnverifiedArgs::collect_args(input)?;
        let mut field_args = Self::default();
        field_args.args = field_args.args.take_args(&mut args, false)?;
        if let Some(mut args_else) = args_else {
            let have_eval = field_args.args.eval.is_some();
            field_args.args_else = Some(FieldArgsElse::default().take_args(&mut args_else, have_eval)?);
        }
        Ok(field_args)
    }
}

impl Skipped for FieldArgs {
    fn skipped(&self) -> bool {
        self.args.skip
    }
}

impl WithConditional for FieldArgs {
    type Normal = FieldArgsNormal;
    type Else = FieldArgsElse;

    fn conditional(&self) -> (&Self::Normal, &Option<Self::Else>) {
        (&self.args, &self.args_else)
    }
}
