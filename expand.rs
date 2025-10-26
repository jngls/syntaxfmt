#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use syntaxfmt_macros::SyntaxFmt as SyntaxFmtDerive;
use syntaxfmt::syntax_fmt;
struct Ident(&'static str);
impl<__SyntaxFmtState> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState> for Ident
where
    &'static str: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
{
    fn syntax_fmt(
        &self,
        f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>,
    ) -> ::std::fmt::Result {
        let Self(__0) = self;
        __0.syntax_fmt(f)?;
        Ok(())
    }
}
struct Type(&'static str);
impl<__SyntaxFmtState> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState> for Type
where
    &'static str: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
{
    fn syntax_fmt(
        &self,
        f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>,
    ) -> ::std::fmt::Result {
        let Self(__0) = self;
        __0.syntax_fmt(f)?;
        Ok(())
    }
}
struct Param {
    #[syntax(fmt = "{*}: ")]
    name: Ident,
    ty: Type,
}
impl<__SyntaxFmtState> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState> for Param
where
    Ident: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
    Type: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
{
    fn syntax_fmt(
        &self,
        f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>,
    ) -> ::std::fmt::Result {
        let Self { name, ty, .. } = self;
        f.write_strs(["", ""])?;
        name.syntax_fmt(f)?;
        f.write_strs([": ", ": "])?;
        ty.syntax_fmt(f)?;
        Ok(())
    }
}
struct Block {
    #[syntax(fmt = " {{*}}", nl = con, ind, delim = "")]
    statements: Vec<Statement>,
}
impl<__SyntaxFmtState> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState> for Block
where
    Vec<Statement>: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
{
    fn syntax_fmt(
        &self,
        f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>,
    ) -> ::std::fmt::Result {
        let Self { statements, .. } = self;
        f.write_strs([" {", " {"])?;
        f.push_indent();
        f.push_delim(["", ""]);
        statements.syntax_fmt(f)?;
        f.pop_delim();
        f.pop_indent();
        f.write_newline()?;
        f.write_strs(["}", "}"])?;
        Ok(())
    }
}
#[syntax(fmt = "{*};", nl = beg)]
enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
}
impl<__SyntaxFmtState> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState> for Statement
where
    LetStatement: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
    ReturnStatement: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
{
    fn syntax_fmt(
        &self,
        f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>,
    ) -> ::std::fmt::Result {
        f.write_newline()?;
        f.write_strs(["", ""])?;
        match self {
            Self::Let(__0) => {
                __0.syntax_fmt(f)?;
            }
            Self::Return(__0) => {
                __0.syntax_fmt(f)?;
            }
            _ => {}
        }
        f.write_strs([";", ";"])?;
        Ok(())
    }
}
#[syntax(fmt = "let {*}")]
struct LetStatement {
    name: Ident,
    #[syntax(fmt = " = {*}", eval = value.is_some())]
    value: Option<Ident>,
}
impl<__SyntaxFmtState> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState> for LetStatement
where
    Ident: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
    Option<Ident>: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
{
    fn syntax_fmt(
        &self,
        f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>,
    ) -> ::std::fmt::Result {
        f.write_strs(["let ", "let "])?;
        let Self { name, value, .. } = self;
        name.syntax_fmt(f)?;
        if (value.is_some()) {
            f.write_strs([" = ", " = "])?;
            value.syntax_fmt(f)?;
            f.write_strs(["", ""])?;
        }
        f.write_strs(["", ""])?;
        Ok(())
    }
}
#[syntax(fmt = "return {*}")]
struct ReturnStatement {
    value: Ident,
}
impl<__SyntaxFmtState> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState> for ReturnStatement
where
    Ident: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
{
    fn syntax_fmt(
        &self,
        f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>,
    ) -> ::std::fmt::Result {
        f.write_strs(["return ", "return "])?;
        let Self { value, .. } = self;
        value.syntax_fmt(f)?;
        f.write_strs(["", ""])?;
        Ok(())
    }
}
struct Function {
    #[syntax(cont = "pub ", eval = *is_pub)]
    is_pub: bool,
    #[syntax(fmt = "fn {*}")]
    name: Ident,
    #[syntax(fmt = "({*})", delim = ", ")]
    params: Vec<Param>,
    #[syntax(fmt = " -> {*}", eval = return_type.is_some())]
    return_type: Option<Type>,
    body: Block,
}
impl<__SyntaxFmtState> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState> for Function
where
    bool: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
    Ident: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
    Vec<Param>: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
    Option<Type>: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
    Block: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
{
    fn syntax_fmt(
        &self,
        f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>,
    ) -> ::std::fmt::Result {
        let Self { is_pub, name, params, return_type, body, .. } = self;
        if (*is_pub) {
            ("pub ").syntax_fmt(f)?;
        }
        f.write_strs(["fn ", "fn "])?;
        name.syntax_fmt(f)?;
        f.write_strs(["", ""])?;
        f.write_strs(["(", "("])?;
        f.push_delim([", ", ", "]);
        params.syntax_fmt(f)?;
        f.pop_delim();
        f.write_strs([")", ")"])?;
        if (return_type.is_some()) {
            f.write_strs([" -> ", " -> "])?;
            return_type.syntax_fmt(f)?;
            f.write_strs(["", ""])?;
        }
        body.syntax_fmt(f)?;
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "test_function_integration"]
#[doc(hidden)]
pub const test_function_integration: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_function_integration"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/integration.rs",
        start_line: 71usize,
        start_col: 4usize,
        end_line: 71usize,
        end_col: 29usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_function_integration()),
    ),
};
fn test_function_integration() {
    let func = Function {
        is_pub: true,
        name: Ident("add"),
        params: <[_]>::into_vec(
            ::alloc::boxed::box_new([
                Param {
                    name: Ident("a"),
                    ty: Type("i32"),
                },
                Param {
                    name: Ident("b"),
                    ty: Type("i32"),
                },
            ]),
        ),
        return_type: Some(Type("i32")),
        body: Block {
            statements: <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    Statement::Let(LetStatement {
                        name: Ident("result"),
                        value: Some(Ident("a + b")),
                    }),
                    Statement::Return(ReturnStatement {
                        value: Ident("result"),
                    }),
                ]),
            ),
        },
    };
    let normal = ::alloc::__export::must_use({
        ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&func)))
    });
    match (
        &normal,
        &"pub fn add(a: i32, b: i32) -> i32 {let result = a + b;return result;}",
    ) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    let pretty = ::alloc::__export::must_use({
        ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&func).pretty()))
    });
    match (
        &pretty,
        &"pub fn add(a: i32, b: i32) -> i32 {\n    let result = a + b;\n    return result;\n}",
    ) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[rustc_test_marker = "test_function_without_return_type"]
#[doc(hidden)]
pub const test_function_without_return_type: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_function_without_return_type"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/integration.rs",
        start_line: 109usize,
        start_col: 4usize,
        end_line: 109usize,
        end_col: 37usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_function_without_return_type()),
    ),
};
fn test_function_without_return_type() {
    let func = Function {
        is_pub: false,
        name: Ident("print_hello"),
        params: ::alloc::vec::Vec::new(),
        return_type: None,
        body: Block {
            statements: <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    Statement::Return(ReturnStatement {
                        value: Ident("()"),
                    }),
                ]),
            ),
        },
    };
    let normal = ::alloc::__export::must_use({
        ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&func)))
    });
    match (&normal, &"fn print_hello() {{\nreturn ()}}") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[&test_function_integration, &test_function_without_return_type],
    )
}
