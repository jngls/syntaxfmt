#![feature(prelude_import)]
#![allow(unused)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use syntaxfmt_macros::SyntaxFmt as SyntaxFmtDerive;
use syntaxfmt::{syntax_fmt, Mode, SyntaxFmt, SyntaxFormatter};
struct WithIndent {
    #[syntax(nl)]
    header: &'static str,
    #[syntax(nl, ind)]
    body: &'static str,
    footer: &'static str,
}
impl<__SyntaxFmtState> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState> for WithIndent
where
    &'static str: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
    &'static str: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
    &'static str: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
{
    fn syntax_fmt(
        &self,
        f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>,
    ) -> ::std::fmt::Result {
        let Self { header, body, footer, .. } = self;
        header.syntax_fmt(f)?;
        f.write_newline()?;
        f.push_indent();
        body.syntax_fmt(f)?;
        f.pop_indent();
        f.write_newline()?;
        footer.syntax_fmt(f)?;
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "test_indent_region"]
#[doc(hidden)]
pub const test_indent_region: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_indent_region"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 23usize,
        start_col: 4usize,
        end_line: 23usize,
        end_col: 22usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_indent_region()),
    ),
};
fn test_indent_region() {
    let s = WithIndent {
        header: "indent {",
        body: "foo",
        footer: "}",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s)))
        }),
        &"indent {foo}",
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
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s).pretty()))
        }),
        &"indent {\n    foo\n}",
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
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&test_indent_region])
}
