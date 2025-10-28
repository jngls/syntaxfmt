use std::fmt::Display;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{
    parse_macro_input, token::Union, DeriveInput, Result as SynResult, Error as SynError, Expr, Ident, Meta, MetaList, Path
};

use crate::intermediate::{parse_type::ParseType, ty::SyntaxType};

mod attributes;
mod intermediate;

fn syn_err<T: ToTokens, U: Display, R>(tokens: T, message: U) -> SynResult<R> {
    Err(SynError::new_spanned(tokens, message))
}


// #[derive(Debug, Clone)]
// enum SyntaxError {
//     SynError(SynError),
//     DuplicateArg(Ident),
//     ExpectedStringLit(Expr),
//     ExpectedStringLits(Expr),
//     ExpectedTraitPath(Path),
//     ExpectedStatePath(Path),
//     ExpectedContent(Expr),
//     ExpectedMetaList(Meta),
//     ExpectedCondition(Path),
//     UnexpectedAttributeArg(Path),
//     UnexpectedAttributeType(Meta),
//     UnexpectedConditionalExpr(Expr),
//     UnsupportedNewlinePath(Path),
//     UnsupportedNewlineExpr(Expr),
//     UnsupportedContentExpr(Expr),
//     UnsupportedConditionalExpr(Expr),
//     FailedToParseMeta(MetaList),
//     Union(Union),
// }

// impl From<SynError> for SyntaxError {
//     fn from(value: SynError) -> Self {
//         Self::SynError(value)
//     }
// }

fn expr_to_str(expr: &Expr) -> &'static str {
    match expr {
        Expr::Array(_) => "Array",
        Expr::Assign(_) => "Assign",
        Expr::Async(_) => "Async",
        Expr::Await(_) => "Await",
        Expr::Binary(_) => "Binary",
        Expr::Block(_) => "Block",
        Expr::Break(_) => "Break",
        Expr::Call(_) => "Call",
        Expr::Cast(_) => "Cast",
        Expr::Closure(_) => "Closure",
        Expr::Const(_) => "Const",
        Expr::Continue(_) => "Continue",
        Expr::Field(_) => "Field",
        Expr::ForLoop(_) => "ForLoop",
        Expr::Group(_) => "Group",
        Expr::If(_) => "If",
        Expr::Index(_) => "Index",
        Expr::Infer(_) => "Infer",
        Expr::Let(_) => "Let",
        Expr::Lit(_) => "Lit",
        Expr::Loop(_) => "Loop",
        Expr::Macro(_) => "Macro",
        Expr::Match(_) => "Match",
        Expr::MethodCall(_) => "MethodCall",
        Expr::Paren(_) => "Paren",
        Expr::Path(_) => "Path",
        Expr::Range(_) => "Range",
        Expr::RawAddr(_) => "RawAddr",
        Expr::Reference(_) => "Reference",
        Expr::Repeat(_) => "Repeat",
        Expr::Return(_) => "Return",
        Expr::Struct(_) => "Struct",
        Expr::Try(_) => "Try",
        Expr::TryBlock(_) => "TryBlock",
        Expr::Tuple(_) => "Tuple",
        Expr::Unary(_) => "Unary",
        Expr::Unsafe(_) => "Unsafe",
        Expr::Verbatim(_) => "Verbatim",
        Expr::While(_) => "While",
        Expr::Yield(_) => "Yield",
        _ => todo!(),
    }
}

#[proc_macro_derive(SyntaxFmt, attributes(syntax, syntax_else))]
pub fn derive_syntax_fmt(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ty = match SyntaxType::parse_type(&mut Vec::new(), &input) {
        Ok(ty) => ty,
        Err(e) => return e.to_compile_error().into(),
//         {
//             return match e {
//                 SyntaxError::SynError(e) => e,
//                 SyntaxError::DuplicateArg(t) =>
//                     SynError::new_spanned(t, "syntaxfmt argument already specified"),
//                 SyntaxError::ExpectedStringLit(t) =>
//                     SynError::new_spanned(t, "syntaxfmt argument expected string literal"),
//                 SyntaxError::ExpectedStringLits(t) =>
//                     SynError::new_spanned(t, "syntaxfmt argument expected string literal or array of string literals `[\"normal\", \"pretty\"]`"),
//                 SyntaxError::ExpectedTraitPath(t) =>
//                     SynError::new_spanned(t, "syntaxfmt argument expected path to custom state trait"),
//                 SyntaxError::ExpectedStatePath(t) =>
//                     SynError::new_spanned(t, "syntaxfmt argument expected path to custom state type"),
//                 SyntaxError::ExpectedContent(t) =>
//                     SynError::new_spanned(t, "syntaxfmt `format` argument must contain `{*}` placeholder.
// If you simply want to replace the content, please use the `content = \"xyz\"` argument instead."),
//                 SyntaxError::ExpectedMetaList(t) =>
//                     SynError::new_spanned(t, "syntaxfmt argument expected meta list"),
//                 SyntaxError::ExpectedCondition(t) =>
//                     SynError::new_spanned(t, "syntaxfmt argument expected `eval` argument"),
//                 SyntaxError::UnexpectedAttributeArg(t) =>
//                     SynError::new_spanned(t, "syntaxfmt unexpected attribute argument"),
//                 SyntaxError::UnexpectedAttributeType(t) =>
//                     SynError::new_spanned(t, "syntaxfmt unexpected attribute argument type"),
//                 SyntaxError::UnexpectedConditionalExpr(t) =>
//                     SynError::new_spanned(t, "syntaxfmt unexpected conditional expression"),
//                 SyntaxError::UnsupportedNewlinePath(t) => {
//                     SynError::new_spanned(t.clone(), format!("syntaxfmt unsupported newline argument: {}", t.get_ident().map(|i| i.to_string()).unwrap_or(String::from("unknown"))))
//                 }
//                 SyntaxError::UnsupportedNewlineExpr(t) => {
//                     SynError::new_spanned(t.clone(), format!("syntaxfmt unsupported newline expression: {}", expr_to_str(&t)))
//                 }
//                 SyntaxError::UnsupportedContentExpr(t) => {
//                     SynError::new_spanned(t.clone(), format!("syntaxfmt unsupported content expression: {}", expr_to_str(&t)))
//                 }
//                 SyntaxError::UnsupportedConditionalExpr(t) => {
//                     SynError::new_spanned(t.clone(), format!("syntaxfmt unsupported conditional expression: {}", expr_to_str(&t)))
//                 }
//                 SyntaxError::FailedToParseMeta(t) =>
//                     SynError::new_spanned(t, "syntaxfmt failed to parse attribute meta list"),
//                 SyntaxError::Union(t) =>
//                     SynError::new_spanned(t, "syntaxfmt cannot be derived for unions"),
//             }.to_compile_error().into();
//         }
    };

    ty.to_token_stream().into()
}
