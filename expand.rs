#![feature(prelude_import)]
#![allow(unused)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use std::marker::PhantomData;
use syntaxfmt_macros::SyntaxFmt as SyntaxFmtDerive;
use syntaxfmt::{syntax_fmt, Mode, SyntaxFmt, SyntaxFormatter};
struct Unit;
extern crate test;
#[rustc_test_marker = "test_unit"]
#[doc(hidden)]
pub const test_unit: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_unit"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 15usize,
        start_col: 4usize,
        end_line: 15usize,
        end_col: 13usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(test_unit())),
};
fn test_unit() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&Unit)))
        }),
        &"",
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
struct EmptyNamed {}
extern crate test;
#[rustc_test_marker = "test_empty_named"]
#[doc(hidden)]
pub const test_empty_named: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_empty_named"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 23usize,
        start_col: 4usize,
        end_line: 23usize,
        end_col: 20usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_empty_named()),
    ),
};
fn test_empty_named() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&EmptyNamed {})))
        }),
        &"",
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
struct EmptyUnnamed();
extern crate test;
#[rustc_test_marker = "test_empty_unnamed"]
#[doc(hidden)]
pub const test_empty_unnamed: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_empty_unnamed"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 31usize,
        start_col: 4usize,
        end_line: 31usize,
        end_col: 22usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_empty_unnamed()),
    ),
};
fn test_empty_unnamed() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&EmptyUnnamed())))
        }),
        &"",
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
struct WithSkip {
    visible: &'static str,
    #[syntax(skip)]
    invisible: &'static str,
}
extern crate test;
#[rustc_test_marker = "test_skip"]
#[doc(hidden)]
pub const test_skip: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_skip"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 47usize,
        start_col: 4usize,
        end_line: 47usize,
        end_col: 13usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(test_skip())),
};
fn test_skip() {
    let s = WithSkip {
        visible: "visible",
        invisible: "invisible",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s)))
        }),
        &"visible",
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
enum EnumWithSkip {
    Visible(&'static str),
    #[syntax(skip)]
    Invisible(&'static str),
}
extern crate test;
#[rustc_test_marker = "test_skip_variant"]
#[doc(hidden)]
pub const test_skip_variant: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_skip_variant"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 60usize,
        start_col: 4usize,
        end_line: 60usize,
        end_col: 21usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_skip_variant()),
    ),
};
fn test_skip_variant() {
    let visible = EnumWithSkip::Visible("visible");
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&visible)))
        }),
        &"visible",
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
    let invisible = EnumWithSkip::Invisible("invisible");
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&invisible)))
        }),
        &"",
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
#[syntax(skip)]
struct WithOuterSkip {
    invisible: &'static str,
}
extern crate test;
#[rustc_test_marker = "test_outer_skip"]
#[doc(hidden)]
pub const test_outer_skip: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_outer_skip"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 75usize,
        start_col: 4usize,
        end_line: 75usize,
        end_col: 19usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_outer_skip()),
    ),
};
fn test_outer_skip() {
    let s = WithOuterSkip {
        invisible: "invisible",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s)))
        }),
        &"",
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
struct WithIndent {
    header: &'static str,
    #[syntax(ind, nl = [pre, cont])]
    body: &'static str,
    footer: &'static str,
}
extern crate test;
#[rustc_test_marker = "test_indent_pretty"]
#[doc(hidden)]
pub const test_indent_pretty: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_indent_pretty"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 93usize,
        start_col: 4usize,
        end_line: 93usize,
        end_col: 22usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_indent_pretty()),
    ),
};
fn test_indent_pretty() {
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
struct WithNestedIndent {
    header: &'static str,
    #[syntax(ind, nl = [pre, cont])]
    body: WithIndent,
    footer: &'static str,
}
extern crate test;
#[rustc_test_marker = "test_nested_indent_pretty"]
#[doc(hidden)]
pub const test_nested_indent_pretty: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_nested_indent_pretty"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 113usize,
        start_col: 4usize,
        end_line: 113usize,
        end_col: 29usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_nested_indent_pretty()),
    ),
};
fn test_nested_indent_pretty() {
    let s = WithNestedIndent {
        header: "outer {",
        body: WithIndent {
            header: "inner {",
            body: "foo",
            footer: "}",
        },
        footer: "}",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s)))
        }),
        &"outer {inner {foo}}",
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
        &"outer {\n    inner {\n        foo\n    }\n}",
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
#[syntax(ind, nl = pre)]
struct WithOuterIndent {
    indented: &'static str,
}
extern crate test;
#[rustc_test_marker = "test_outer_indent_pretty"]
#[doc(hidden)]
pub const test_outer_indent_pretty: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_outer_indent_pretty"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 139usize,
        start_col: 4usize,
        end_line: 139usize,
        end_col: 28usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_outer_indent_pretty()),
    ),
};
fn test_outer_indent_pretty() {
    let s = WithOuterIndent { indented: "foo" };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s)))
        }),
        &"foo",
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
        &"\n    foo",
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
struct WithPreSuf {
    #[syntax(pre = "prefix_", suf = "_suffix")]
    field: &'static str,
}
extern crate test;
#[rustc_test_marker = "test_format_prefix_suffix"]
#[doc(hidden)]
pub const test_format_prefix_suffix: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_format_prefix_suffix"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 156usize,
        start_col: 4usize,
        end_line: 156usize,
        end_col: 29usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_format_prefix_suffix()),
    ),
};
fn test_format_prefix_suffix() {
    let s = WithPreSuf { field: "value" };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s)))
        }),
        &"prefix_value_suffix",
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
struct WithModalPreSuf {
    #[syntax(pre = ["normal[", "pretty[ "], suf = ["]", " ]"])]
    field: &'static str,
}
extern crate test;
#[rustc_test_marker = "test_modal_format"]
#[doc(hidden)]
pub const test_modal_format: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_modal_format"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 169usize,
        start_col: 4usize,
        end_line: 169usize,
        end_col: 21usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_modal_format()),
    ),
};
fn test_modal_format() {
    let s = WithModalPreSuf { field: "x" };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s)))
        }),
        &"normal[x]",
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
        &"pretty[ x ]",
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
#[syntax(pre = "outer<", suf = ">")]
struct WithOuterPreSuf {
    inner: &'static str,
}
extern crate test;
#[rustc_test_marker = "test_format_outer"]
#[doc(hidden)]
pub const test_format_outer: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_format_outer"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 182usize,
        start_col: 4usize,
        end_line: 182usize,
        end_col: 21usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_format_outer()),
    ),
};
fn test_format_outer() {
    let s = WithOuterPreSuf { inner: "value" };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s)))
        }),
        &"outer<value>",
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
struct WithContentLiteral {
    #[syntax(cont = "LITERAL")]
    field: i32,
}
extern crate test;
#[rustc_test_marker = "test_content_literal"]
#[doc(hidden)]
pub const test_content_literal: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_content_literal"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 198usize,
        start_col: 4usize,
        end_line: 198usize,
        end_col: 24usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_content_literal()),
    ),
};
fn test_content_literal() {
    let s = WithContentLiteral { field: 42 };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s)))
        }),
        &"LITERAL",
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
struct WithModalContent {
    #[syntax(cont = ["normal", "pretty"])]
    field: &'static str,
}
extern crate test;
#[rustc_test_marker = "test_modal_content"]
#[doc(hidden)]
pub const test_modal_content: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_modal_content"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 210usize,
        start_col: 4usize,
        end_line: 210usize,
        end_col: 22usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_modal_content()),
    ),
};
fn test_modal_content() {
    let s = WithModalContent {
        field: "ignored",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s)))
        }),
        &"normal",
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
        &"pretty",
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
#[syntax(cont = ["normal", "pretty"])]
struct WithOuterContent {
    field: &'static str,
}
extern crate test;
#[rustc_test_marker = "test_outer_content"]
#[doc(hidden)]
pub const test_outer_content: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_outer_content"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 223usize,
        start_col: 4usize,
        end_line: 223usize,
        end_col: 22usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_outer_content()),
    ),
};
fn test_outer_content() {
    let s = WithOuterContent {
        field: "ignored",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s)))
        }),
        &"normal",
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
        &"pretty",
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
fn my_formatter<S>(field: &str, f: &mut SyntaxFormatter<S>) -> std::fmt::Result {
    f.write_fmt(format_args!("formatted[{0}]", field))
}
struct WithContentPath {
    #[syntax(cont_with = my_formatter)]
    field: &'static str,
}
extern crate test;
#[rustc_test_marker = "test_content_path_as_formatter"]
#[doc(hidden)]
pub const test_content_path_as_formatter: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_content_path_as_formatter"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 244usize,
        start_col: 4usize,
        end_line: 244usize,
        end_col: 34usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_content_path_as_formatter()),
    ),
};
fn test_content_path_as_formatter() {
    let s = WithContentPath { field: "value" };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s)))
        }),
        &"formatted[value]",
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
struct WithContentClosure {
    #[syntax(
        cont_with = |field:&str,
        f:&mut
        SyntaxFormatter<_>|write!(f, "closure[{}]", field)
    )]
    field: &'static str,
}
extern crate test;
#[rustc_test_marker = "test_content_closure_as_formatter"]
#[doc(hidden)]
pub const test_content_closure_as_formatter: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_content_closure_as_formatter"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 256usize,
        start_col: 4usize,
        end_line: 256usize,
        end_col: 37usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_content_closure_as_formatter()),
    ),
};
fn test_content_closure_as_formatter() {
    let s = WithContentClosure {
        field: "value",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s)))
        }),
        &"closure[value]",
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
fn my_struct_formatter<S>(
    _struct: &WithOuterContentPath,
    f: &mut SyntaxFormatter<S>,
) -> std::fmt::Result {
    f.write_fmt(format_args!("formatted[{0}]", _struct.field))
}
#[syntax(cont_with = my_struct_formatter)]
struct WithOuterContentPath {
    field: &'static str,
}
extern crate test;
#[rustc_test_marker = "test_outer_content_path_as_formatter"]
#[doc(hidden)]
pub const test_outer_content_path_as_formatter: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_outer_content_path_as_formatter"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 272usize,
        start_col: 4usize,
        end_line: 272usize,
        end_col: 40usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_outer_content_path_as_formatter()),
    ),
};
fn test_outer_content_path_as_formatter() {
    let s = WithOuterContentPath {
        field: "value",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s)))
        }),
        &"formatted[value]",
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
#[syntax(
    cont_with = |_struct:&Self,
    f:&mut
    SyntaxFormatter<_>|write!(f, "closure[{}]", _struct.field)
)]
struct WithOuterContentClosure {
    field: &'static str,
}
extern crate test;
#[rustc_test_marker = "test_outer_content_closure_as_formatter"]
#[doc(hidden)]
pub const test_outer_content_closure_as_formatter: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_outer_content_closure_as_formatter"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 284usize,
        start_col: 4usize,
        end_line: 284usize,
        end_col: 43usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_outer_content_closure_as_formatter()),
    ),
};
fn test_outer_content_closure_as_formatter() {
    let s = WithOuterContentClosure {
        field: "value",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s)))
        }),
        &"closure[value]",
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
struct WithContentModalSlice {
    #[syntax(cont = ["normal", "pretty"])]
    field: &'static str,
}
extern crate test;
#[rustc_test_marker = "test_content_modal_slice"]
#[doc(hidden)]
pub const test_content_modal_slice: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_content_modal_slice"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 300usize,
        start_col: 4usize,
        end_line: 300usize,
        end_col: 28usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_content_modal_slice()),
    ),
};
fn test_content_modal_slice() {
    let s = WithContentModalSlice {
        field: "value",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s)))
        }),
        &"normal",
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
        &"pretty",
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
#[syntax(cont = ["normal", "pretty"])]
struct WithOuterContentModalSlice {
    field: &'static str,
}
extern crate test;
#[rustc_test_marker = "test_outer_content_modal_slice"]
#[doc(hidden)]
pub const test_outer_content_modal_slice: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_outer_content_modal_slice"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 313usize,
        start_col: 4usize,
        end_line: 313usize,
        end_col: 34usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_outer_content_modal_slice()),
    ),
};
fn test_outer_content_modal_slice() {
    let s = WithOuterContentModalSlice {
        field: "value",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s)))
        }),
        &"normal",
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
        &"pretty",
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
struct DelimItem(&'static str);
struct WithDefaultDelim {
    items: Vec<DelimItem>,
}
extern crate test;
#[rustc_test_marker = "test_default_delim"]
#[doc(hidden)]
pub const test_default_delim: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_default_delim"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 332usize,
        start_col: 4usize,
        end_line: 332usize,
        end_col: 22usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_default_delim()),
    ),
};
fn test_default_delim() {
    let s = WithDefaultDelim {
        items: <[_]>::into_vec(
            ::alloc::boxed::box_new([DelimItem("a"), DelimItem("b"), DelimItem("c")]),
        ),
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s)))
        }),
        &"a,b,c",
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
struct WithDelim {
    #[syntax(delim = "|")]
    items: Vec<DelimItem>,
}
extern crate test;
#[rustc_test_marker = "test_delim"]
#[doc(hidden)]
pub const test_delim: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_delim"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 344usize,
        start_col: 4usize,
        end_line: 344usize,
        end_col: 14usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_delim()),
    ),
};
fn test_delim() {
    let s = WithDelim {
        items: <[_]>::into_vec(
            ::alloc::boxed::box_new([DelimItem("a"), DelimItem("b"), DelimItem("c")]),
        ),
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s)))
        }),
        &"a|b|c",
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
struct WithModalDelim {
    #[syntax(delim = [":", ": "])]
    items: Vec<DelimItem>,
}
extern crate test;
#[rustc_test_marker = "test_modal_delim"]
#[doc(hidden)]
pub const test_modal_delim: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_modal_delim"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 356usize,
        start_col: 4usize,
        end_line: 356usize,
        end_col: 20usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_modal_delim()),
    ),
};
fn test_modal_delim() {
    let s = WithModalDelim {
        items: <[_]>::into_vec(
            ::alloc::boxed::box_new([DelimItem("a"), DelimItem("b"), DelimItem("c")]),
        ),
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s)))
        }),
        &"a:b:c",
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
        &"a: b: c",
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
#[syntax(delim = "|")]
struct WithOuterDelim {
    items: Vec<DelimItem>,
}
extern crate test;
#[rustc_test_marker = "test_outer_delim"]
#[doc(hidden)]
pub const test_outer_delim: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_outer_delim"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 369usize,
        start_col: 4usize,
        end_line: 369usize,
        end_col: 20usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_outer_delim()),
    ),
};
fn test_outer_delim() {
    let s = WithOuterDelim {
        items: <[_]>::into_vec(
            ::alloc::boxed::box_new([DelimItem("a"), DelimItem("b"), DelimItem("c")]),
        ),
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s)))
        }),
        &"a|b|c",
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
struct WithEvalBasic {
    #[syntax(eval = *non_zero>0)]
    non_zero: u32,
}
extern crate test;
#[rustc_test_marker = "test_eval_basic"]
#[doc(hidden)]
pub const test_eval_basic: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_eval_basic"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 385usize,
        start_col: 4usize,
        end_line: 385usize,
        end_col: 19usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_eval_basic()),
    ),
};
fn test_eval_basic() {
    let enabled = WithEvalBasic { non_zero: 123 };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&enabled)))
        }),
        &"123",
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
    let disabled = WithEvalBasic { non_zero: 0 };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&disabled)))
        }),
        &"",
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
struct MaybeKeyword {
    value: &'static str,
}
struct WithEvalMember {
    #[syntax(eval = maybe_keyword.value = = "self")]
    maybe_keyword: MaybeKeyword,
}
extern crate test;
#[rustc_test_marker = "test_eval_member"]
#[doc(hidden)]
pub const test_eval_member: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_eval_member"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 405usize,
        start_col: 4usize,
        end_line: 405usize,
        end_col: 20usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_eval_member()),
    ),
};
fn test_eval_member() {
    let enabled = WithEvalMember {
        maybe_keyword: MaybeKeyword { value: "self" },
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&enabled)))
        }),
        &"self",
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
    let disabled = WithEvalMember {
        maybe_keyword: MaybeKeyword { value: "other" },
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&disabled)))
        }),
        &"",
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
#[syntax(eval = self.non_zero>0)]
struct WithEvalOuter {
    non_zero: u32,
}
extern crate test;
#[rustc_test_marker = "test_outer_eval"]
#[doc(hidden)]
pub const test_outer_eval: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_outer_eval"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 420usize,
        start_col: 4usize,
        end_line: 420usize,
        end_col: 19usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_outer_eval()),
    ),
};
fn test_outer_eval() {
    let enabled = WithEvalOuter { non_zero: 123 };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&enabled)))
        }),
        &"123",
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
    let disabled = WithEvalOuter { non_zero: 0 };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&disabled)))
        }),
        &"",
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
fn is_long(s: &str) -> bool {
    s.len() > 5
}
struct WithEvalPath {
    #[syntax(eval_with = is_long)]
    text: &'static str,
}
extern crate test;
#[rustc_test_marker = "test_eval_path"]
#[doc(hidden)]
pub const test_eval_path: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_eval_path"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 443usize,
        start_col: 4usize,
        end_line: 443usize,
        end_col: 18usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_eval_path()),
    ),
};
fn test_eval_path() {
    let long = WithEvalPath {
        text: "verylongtext",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&long)))
        }),
        &"verylongtext",
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
    let short = WithEvalPath { text: "short" };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&short)))
        }),
        &"",
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
struct WithEvalClosure {
    #[syntax(eval_with = |s:&str|s.contains('p'))]
    text: &'static str,
}
extern crate test;
#[rustc_test_marker = "test_eval_closure"]
#[doc(hidden)]
pub const test_eval_closure: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_eval_closure"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 458usize,
        start_col: 4usize,
        end_line: 458usize,
        end_col: 21usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_eval_closure()),
    ),
};
fn test_eval_closure() {
    let with_a = WithEvalClosure { text: "apple" };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&with_a)))
        }),
        &"apple",
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
    let without_a = WithEvalClosure { text: "orange" };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&without_a)))
        }),
        &"",
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
fn is_long_outer(s: &WithOuterEvalPath) -> bool {
    s.text.len() > 5
}
#[syntax(eval_with = is_long_outer)]
struct WithOuterEvalPath {
    text: &'static str,
}
extern crate test;
#[rustc_test_marker = "test_outer_eval_path"]
#[doc(hidden)]
pub const test_outer_eval_path: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_outer_eval_path"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 477usize,
        start_col: 4usize,
        end_line: 477usize,
        end_col: 24usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_outer_eval_path()),
    ),
};
fn test_outer_eval_path() {
    let long = WithOuterEvalPath {
        text: "verylongtext",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&long)))
        }),
        &"verylongtext",
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
    let short = WithOuterEvalPath { text: "short" };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&short)))
        }),
        &"",
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
#[syntax(eval_with = |s:&Self|s.text.contains('p'))]
struct WithOuterEvalClosure {
    text: &'static str,
}
extern crate test;
#[rustc_test_marker = "test_outer_eval_closure"]
#[doc(hidden)]
pub const test_outer_eval_closure: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_outer_eval_closure"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 492usize,
        start_col: 4usize,
        end_line: 492usize,
        end_col: 27usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_outer_eval_closure()),
    ),
};
fn test_outer_eval_closure() {
    let with_a = WithOuterEvalClosure {
        text: "apple",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&with_a)))
        }),
        &"apple",
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
    let without_a = WithOuterEvalClosure {
        text: "orange",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&without_a)))
        }),
        &"",
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
struct WithElseContent {
    #[syntax(eval = value.is_some())]
    #[syntax_else(cont = "none")]
    value: Option<&'static str>,
}
extern crate test;
#[rustc_test_marker = "test_eval_else_content"]
#[doc(hidden)]
pub const test_eval_else_content: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_eval_else_content"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 512usize,
        start_col: 4usize,
        end_line: 512usize,
        end_col: 26usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_eval_else_content()),
    ),
};
fn test_eval_else_content() {
    let some = WithElseContent {
        value: Some("data"),
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&some)))
        }),
        &"data",
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
    let none = WithElseContent { value: None };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&none)))
        }),
        &"none",
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
trait Resolver {
    fn resolve(&self, name: &str) -> String;
}
struct TestResolver;
impl Resolver for TestResolver {
    fn resolve(&self, name: &str) -> String {
        ::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("resolved_{0}", name))
        })
    }
}
fn resolve_formatter(
    field: &str,
    f: &mut SyntaxFormatter<TestResolver>,
) -> std::fmt::Result {
    let resolved = f.state().resolve(field);
    f.write_fmt(format_args!("{0}", resolved))
}
#[syntax(state = TestResolver)]
struct WithImmutable {
    #[syntax(cont_with = resolve_formatter)]
    name: &'static str,
}
extern crate test;
#[rustc_test_marker = "test_immutable_state"]
#[doc(hidden)]
pub const test_immutable_state: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_immutable_state"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 549usize,
        start_col: 4usize,
        end_line: 549usize,
        end_col: 24usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_immutable_state()),
    ),
};
fn test_immutable_state() {
    let resolver = TestResolver;
    let s = WithImmutable { name: "foo" };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s).state(&resolver)))
        }),
        &"resolved_foo",
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
fn resolve_formatter_bounded<S: Resolver>(
    field: &str,
    f: &mut SyntaxFormatter<S>,
) -> std::fmt::Result {
    let resolved = f.state().resolve(field);
    f.write_fmt(format_args!("{0}", resolved))
}
#[syntax(bound = Resolver)]
struct WithImmutableBounded {
    #[syntax(cont_with = resolve_formatter_bounded)]
    name: &'static str,
}
extern crate test;
#[rustc_test_marker = "test_immutable_state_bounded"]
#[doc(hidden)]
pub const test_immutable_state_bounded: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_immutable_state_bounded"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 571usize,
        start_col: 4usize,
        end_line: 571usize,
        end_col: 32usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_immutable_state_bounded()),
    ),
};
fn test_immutable_state_bounded() {
    let resolver = TestResolver;
    let s = WithImmutableBounded {
        name: "foo",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s).state(&resolver)))
        }),
        &"resolved_foo",
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
trait Counter {
    fn post_inc(&mut self) -> usize;
}
struct TestCounter {
    count: usize,
}
impl Counter for TestCounter {
    fn post_inc(&mut self) -> usize {
        let count = self.count;
        self.count += 1;
        count
    }
}
fn counting_formatter(
    field: &str,
    f: &mut SyntaxFormatter<TestCounter>,
) -> std::fmt::Result {
    let count = f.state_mut().post_inc();
    f.write_fmt(format_args!("{0}#{1}", field, count))
}
#[syntax(state = TestCounter)]
struct WithMutableState {
    #[syntax(cont_with = counting_formatter)]
    name: &'static str,
}
extern crate test;
#[rustc_test_marker = "test_mutable_state"]
#[doc(hidden)]
pub const test_mutable_state: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_mutable_state"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 609usize,
        start_col: 4usize,
        end_line: 609usize,
        end_col: 22usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_mutable_state()),
    ),
};
fn test_mutable_state() {
    let mut counter = TestCounter { count: 0 };
    let s = WithMutableState { name: "item" };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!("{0}", syntax_fmt(&s).state_mut(&mut counter)),
            )
        }),
        &"item#0",
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
    match (&counter.count, &1) {
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
            ::alloc::fmt::format(
                format_args!("{0}", syntax_fmt(&s).state_mut(&mut counter)),
            )
        }),
        &"item#1",
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
    match (&counter.count, &2) {
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
fn counting_formatter_bounded<S: Counter>(
    field: &str,
    f: &mut SyntaxFormatter<S>,
) -> std::fmt::Result {
    let count = f.state_mut().post_inc();
    f.write_fmt(format_args!("{0}#{1}", field, count))
}
#[syntax(bound = Counter)]
struct WithMutableStateBounded {
    #[syntax(cont_with = counting_formatter_bounded)]
    name: &'static str,
}
extern crate test;
#[rustc_test_marker = "test_mutable_state_bounded"]
#[doc(hidden)]
pub const test_mutable_state_bounded: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_mutable_state_bounded"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 639usize,
        start_col: 4usize,
        end_line: 639usize,
        end_col: 30usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_mutable_state_bounded()),
    ),
};
fn test_mutable_state_bounded() {
    let mut counter = TestCounter { count: 0 };
    let s = WithMutableStateBounded {
        name: "item",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!("{0}", syntax_fmt(&s).state_mut(&mut counter)),
            )
        }),
        &"item#0",
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
    match (&counter.count, &1) {
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
            ::alloc::fmt::format(
                format_args!("{0}", syntax_fmt(&s).state_mut(&mut counter)),
            )
        }),
        &"item#1",
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
    match (&counter.count, &2) {
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
struct StateWithLifetime<'a>(PhantomData<&'a i32>);
#[syntax(state = StateWithLifetime<'a>)]
struct WithLifetime {}
impl<'a> ::syntaxfmt::SyntaxFmt<StateWithLifetime<'a>> for WithLifetime {
    fn syntax_fmt(
        &self,
        f: &mut ::syntaxfmt::SyntaxFormatter<StateWithLifetime<'a>>,
    ) -> ::std::fmt::Result {
        let Self { .. } = self;
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "test_lifetime_state"]
#[doc(hidden)]
pub const test_lifetime_state: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_lifetime_state"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 663usize,
        start_col: 4usize,
        end_line: 663usize,
        end_col: 23usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_lifetime_state()),
    ),
};
fn test_lifetime_state() {
    let lifetime_state = StateWithLifetime(Default::default());
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&WithLifetime {})))
        }),
        &"",
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
fn map_state_formatter(
    field: &str,
    f: &mut SyntaxFormatter<TestCounter>,
) -> std::fmt::Result {
    f.map_state(|f, state| { f.write_fmt(format_args!("{0}#{1}", field, state.count)) })
}
#[syntax(state = TestCounter)]
struct WithMapState {
    #[syntax(cont_with = map_state_formatter)]
    name: &'static str,
}
extern crate test;
#[rustc_test_marker = "test_map_state"]
#[doc(hidden)]
pub const test_map_state: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_map_state"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 687usize,
        start_col: 4usize,
        end_line: 687usize,
        end_col: 18usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_map_state()),
    ),
};
fn test_map_state() {
    let counter = TestCounter { count: 42 };
    let s = WithMapState { name: "item" };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s).state(&counter)))
        }),
        &"item#42",
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
    match (&counter.count, &42) {
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
fn map_state_mut_formatter<S: Counter>(
    field: &str,
    f: &mut SyntaxFormatter<S>,
) -> std::fmt::Result {
    f.map_state_mut(|f, state| {
        let count = state.post_inc();
        f.write_fmt(format_args!("{0}#{1}", field, count))
    })
}
#[syntax(bound = Counter)]
struct WithMapStateMut {
    #[syntax(cont_with = map_state_mut_formatter)]
    name: &'static str,
}
extern crate test;
#[rustc_test_marker = "test_map_state_mut"]
#[doc(hidden)]
pub const test_map_state_mut: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_map_state_mut"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 714usize,
        start_col: 4usize,
        end_line: 714usize,
        end_col: 22usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_map_state_mut()),
    ),
};
fn test_map_state_mut() {
    let mut counter = TestCounter { count: 0 };
    let s = WithMapStateMut { name: "item" };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!("{0}", syntax_fmt(&s).state_mut(&mut counter)),
            )
        }),
        &"item#0",
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
    match (&counter.count, &1) {
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
            ::alloc::fmt::format(
                format_args!("{0}", syntax_fmt(&s).state_mut(&mut counter)),
            )
        }),
        &"item#1",
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
    match (&counter.count, &2) {
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
#[rustc_test_marker = "test_immutable_context_mut_access_panics"]
#[doc(hidden)]
pub const test_immutable_context_mut_access_panics: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_immutable_context_mut_access_panics"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 737usize,
        start_col: 4usize,
        end_line: 737usize,
        end_col: 44usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::YesWithMessage("StateRef: state is immutable"),
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_immutable_context_mut_access_panics()),
    ),
};
#[should_panic(expected = "StateRef: state is immutable")]
fn test_immutable_context_mut_access_panics() {
    fn bad_formatter(
        field: &str,
        f: &mut SyntaxFormatter<TestCounter>,
    ) -> std::fmt::Result {
        f.state_mut().post_inc();
        f.write_fmt(format_args!("{0}", field))
    }
    #[syntax(state = TestCounter)]
    struct Bad {
        #[syntax(cont_with = bad_formatter)]
        name: &'static str,
    }
    let counter = TestCounter { count: 0 };
    let s = Bad { name: "test" };
    let _ = ::alloc::__export::must_use({
        ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s).state(&counter)))
    });
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[
            &test_content_closure_as_formatter,
            &test_content_literal,
            &test_content_modal_slice,
            &test_content_path_as_formatter,
            &test_default_delim,
            &test_delim,
            &test_empty_named,
            &test_empty_unnamed,
            &test_eval_basic,
            &test_eval_closure,
            &test_eval_else_content,
            &test_eval_member,
            &test_eval_path,
            &test_format_outer,
            &test_format_prefix_suffix,
            &test_immutable_context_mut_access_panics,
            &test_immutable_state,
            &test_immutable_state_bounded,
            &test_indent_pretty,
            &test_lifetime_state,
            &test_map_state,
            &test_map_state_mut,
            &test_modal_content,
            &test_modal_delim,
            &test_modal_format,
            &test_mutable_state,
            &test_mutable_state_bounded,
            &test_nested_indent_pretty,
            &test_outer_content,
            &test_outer_content_closure_as_formatter,
            &test_outer_content_modal_slice,
            &test_outer_content_path_as_formatter,
            &test_outer_delim,
            &test_outer_eval,
            &test_outer_eval_closure,
            &test_outer_eval_path,
            &test_outer_indent_pretty,
            &test_outer_skip,
            &test_skip,
            &test_skip_variant,
            &test_unit,
        ],
    )
}
