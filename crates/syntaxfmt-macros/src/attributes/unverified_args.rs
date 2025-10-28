use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{Attribute, Expr, ExprClosure, Ident, LitStr, Result as SynResult, TypePath, TypeTraitObject, bracketed, parse::{Parse, ParseStream}, punctuated::Punctuated, token::{Comma, Eq as SynEq, Or as SynOr}};

use crate::syn_err;

#[derive(Debug, Clone)]
pub enum UnverifiedArgKind {
    Indent(Ident),
    Skip(Ident),
    Newlines(Punctuated<Ident, Comma>),
    Prefix(Punctuated<LitStr, Comma>),
    Suffix(Punctuated<LitStr, Comma>),
    Delims(Punctuated<LitStr, Comma>),
    Eval(Expr),
    EvalTypePath(TypePath),
    EvalClosure(ExprClosure),
    Content(Expr),
    ContentTypePath(TypePath),
    ContentClosure(ExprClosure),
    State(TypePath),
    StateBound(TypeTraitObject),
}

impl ToTokens for UnverifiedArgKind {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Indent(i) => i.to_tokens(tokens),
            Self::Skip(i) => i.to_tokens(tokens),
            Self::Newlines(i) => i.to_tokens(tokens),
            Self::Prefix(i) => i.to_tokens(tokens),
            Self::Suffix(i) => i.to_tokens(tokens),
            Self::Delims(i) => i.to_tokens(tokens),
            Self::Eval(i) => i.to_tokens(tokens),
            Self::EvalTypePath(i) => i.to_tokens(tokens),
            Self::EvalClosure(i) => i.to_tokens(tokens),
            Self::Content(i) => i.to_tokens(tokens),
            Self::ContentTypePath(i) => i.to_tokens(tokens),
            Self::ContentClosure(i) => i.to_tokens(tokens),
            Self::State(i) => i.to_tokens(tokens),
            Self::StateBound(i) => i.to_tokens(tokens),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnverifiedArg {
    pub ident: Ident,
    pub kind: UnverifiedArgKind,
}

#[derive(Debug, Clone)]
pub struct UnverifiedArgs {
    pub attr: Option<Ident>,
    pub args: Vec<UnverifiedArg>,
}

impl UnverifiedArgs {
    fn parse_idents(input: &mut ParseStream) -> SynResult<Punctuated<Ident, Comma>> {
        if input.peek(Ident) {
            let mut punct = Punctuated::new();
            punct.push(input.parse::<Ident>()?);
            Ok(punct)
        } else {
            let content;
            bracketed!(content in input);
            Ok(content.parse_terminated(Ident::parse, Comma)?)
        }
    }

    fn parse_litstrs(input: &mut ParseStream) -> SynResult<Punctuated<LitStr, Comma>> {
        if input.peek(LitStr) {
            let mut punct = Punctuated::new();
            punct.push(input.parse::<LitStr>()?);
            Ok(punct)
        } else {
            let content;
            bracketed!(content in input);
            Ok(content.parse_terminated(|i| i.parse::<LitStr>(), Comma)?)
        }
    }

    pub fn collect_args<'a>(input: &'a [Attribute]) -> SynResult<(UnverifiedArgs, Option<UnverifiedArgs>)> {
        let mut args = Vec::new();
        let mut args_else = Vec::new();
        let mut last_attr = None;
        let mut last_attr_else = None;
        for attr in input {
            if let Some(ident) = attr.path().get_ident() {
                if ident == "syntax" {
                    let a = attr.parse_args_with(UnverifiedArgs::parse)?;
                    args.extend(a.args);
                    last_attr = Some(ident.clone());
                } else if ident == "syntax_else" {
                    let a = attr.parse_args_with(UnverifiedArgs::parse)?;
                    args_else.extend(a.args);
                    last_attr_else = Some(ident.clone());
                }
            }
        }
        let args = UnverifiedArgs {
            attr: last_attr,
            args,
        };
        let args_else = (!args_else.is_empty()).then_some(UnverifiedArgs {
            attr: last_attr_else,
            args: args_else,
        });
        Ok((args, args_else))
    }
}

impl Parse for UnverifiedArgs {
    fn parse(mut input: ParseStream) -> syn::Result<Self> {
        let mut attrs = Vec::new();

        let mut subsequent = false;
        while !input.is_empty() {
            if subsequent {
                input.parse::<Comma>()?;
            }
            subsequent = true;

            let ident = input.parse::<Ident>()?;

            if ident == "ind" {
                attrs.push(UnverifiedArg {
                    ident: ident.clone(),
                    kind: UnverifiedArgKind::Indent(ident),
                });
            } else if ident == "skip" {
                attrs.push(UnverifiedArg {
                    ident: ident.clone(),
                    kind: UnverifiedArgKind::Skip(ident),
                });
            } else if ident == "nl" {
                if input.peek(SynEq) {
                    input.parse::<SynEq>()?;
                    let newlines = Self::parse_idents(&mut input)?;
                    attrs.push(UnverifiedArg {
                        ident,
                        kind: UnverifiedArgKind::Newlines(newlines),
                    });
                } else {
                    attrs.push(UnverifiedArg {
                        ident,
                        kind: UnverifiedArgKind::Newlines(Punctuated::new()),
                    });
                }
            } else if ident == "pre" {
                input.parse::<SynEq>()?;
                let strs = Self::parse_litstrs(&mut input)?;
                attrs.push(UnverifiedArg {
                    ident,
                    kind: UnverifiedArgKind::Prefix(strs),
                });
            } else if ident == "suf" {
                input.parse::<SynEq>()?;
                let strs = Self::parse_litstrs(&mut input)?;
                attrs.push(UnverifiedArg {
                    ident,
                    kind: UnverifiedArgKind::Suffix(strs),
                });
            } else if ident == "delim" {
                input.parse::<SynEq>()?;
                let strs = Self::parse_litstrs(&mut input)?;
                attrs.push(UnverifiedArg {
                    ident,
                    kind: UnverifiedArgKind::Delims(strs),
                });
            } else if ident == "eval" {
                input.parse::<SynEq>()?;
                let expr = input.parse::<Expr>()?;
                attrs.push(UnverifiedArg {
                    ident,
                    kind: UnverifiedArgKind::Eval(expr),
                });
            } else if ident == "eval_with" {
                input.parse::<SynEq>()?;
                if input.peek(SynOr) {
                    let closure = input.parse::<ExprClosure>()?;
                    attrs.push(UnverifiedArg {
                        ident,
                        kind: UnverifiedArgKind::EvalClosure(closure),
                    });
                } else {
                    let type_path = input.parse::<TypePath>()?;
                    attrs.push(UnverifiedArg {
                        ident,
                        kind: UnverifiedArgKind::EvalTypePath(type_path),
                    });
                }
            } else if ident == "cont" {
                input.parse::<SynEq>()?;
                let expr = input.parse::<Expr>()?;
                attrs.push(UnverifiedArg {
                    ident,
                    kind: UnverifiedArgKind::Content(expr),
                });
            } else if ident == "cont_with" {
                input.parse::<SynEq>()?;
                if input.peek(SynOr) {
                    let closure = input.parse::<ExprClosure>()?;
                    attrs.push(UnverifiedArg {
                        ident,
                        kind: UnverifiedArgKind::ContentClosure(closure),
                    });
                } else {
                    let type_path = input.parse::<TypePath>()?;
                    attrs.push(UnverifiedArg {
                        ident,
                        kind: UnverifiedArgKind::ContentTypePath(type_path),
                    });
                }
            } else if ident == "state" {
                input.parse::<SynEq>()?;
                let type_path = input.parse::<TypePath>()?;
                attrs.push(UnverifiedArg {
                    ident,
                    kind: UnverifiedArgKind::State(type_path),
                });
            } else if ident == "bound" {
                input.parse::<SynEq>()?;
                let trait_obj = input.parse::<TypeTraitObject>()?;
                attrs.push(UnverifiedArg {
                    ident,
                    kind: UnverifiedArgKind::StateBound(trait_obj),
                });
            } else {
                return syn_err(ident, "syntaxfmt unexpected attribute argument");
            }
        }

        Ok(Self {
            attr: None,
            args: attrs,
        })
    }
}
