#![feature(prelude_import)]
#![allow(unreachable_patterns)]
#![allow(unused)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use syntaxfmt_macros::SyntaxFmt as SyntaxFmtDerive;
use syntaxfmt::{syntax_fmt, Mode, SyntaxFmt, SyntaxFormatter};
struct Statement<'src>(#[syntax(format = ("{*}", "{*}\n"))] &'src str);
impl<'src, __SyntaxFmtState> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState> for Statement<'src>
where
    &'src str: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
{
    fn syntax_fmt(
        &self,
        f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>,
    ) -> ::std::fmt::Result {
        let field = self;
        let Self(__0) = self;
        let field = __0;
        f.write_strs(["", ""]);
        __0.syntax_fmt(f)?;
        f.write_strs(["", "\n"]);
        Ok(())
    }
}
struct Items<'src>(Vec<Statement<'src>>);
impl<'src> Items<'src> {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl<'src> SyntaxFmt<()> for Items<'src> {
    fn syntax_fmt(&self, ctx: &mut SyntaxFormatter<()>) -> ::std::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }
        ctx.push_delim([", ", ""]);
        for (i, item) in self.0.iter().enumerate() {
            if i > 0 {
                ctx.write_delim()?;
            }
            if ctx.mode() == Mode::Pretty {
                ctx.write_indent()?;
            }
            item.syntax_fmt(ctx)?;
        }
        ctx.pop_delim();
        Ok(())
    }
}
struct SimpleStruct<'src> {
    #[syntax(format = "name: {*}")]
    name: &'src str,
}
impl<'src, __SyntaxFmtState> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>
for SimpleStruct<'src>
where
    &'src str: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
{
    fn syntax_fmt(
        &self,
        f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>,
    ) -> ::std::fmt::Result {
        let field = self;
        let Self { name } = self;
        let field = name;
        f.write_strs(["name: ", "name: "]);
        name.syntax_fmt(f)?;
        f.write_strs(["", ""]);
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "test_basic_struct"]
#[doc(hidden)]
pub const test_basic_struct: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_basic_struct"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 47usize,
        start_col: 4usize,
        end_line: 47usize,
        end_col: 21usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_basic_struct()),
    ),
};
fn test_basic_struct() {
    let s = SimpleStruct { name: "foo" };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&s)))
        }),
        &"name: foo",
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
        &"name: foo",
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
struct WithOptional<'src> {
    #[syntax(format = ("required: {*};", "required: {*};\n"))]
    required: &'src str,
    #[syntax(format = (" optional: {*}", "optional: {*}"))]
    optional: Option<&'src str>,
}
impl<'src, __SyntaxFmtState> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>
for WithOptional<'src>
where
    &'src str: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
    Option<&'src str>: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
{
    fn syntax_fmt(
        &self,
        f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>,
    ) -> ::std::fmt::Result {
        let field = self;
        let Self { required, optional } = self;
        let field = required;
        f.write_strs(["required: ", "required: "]);
        required.syntax_fmt(f)?;
        f.write_strs([";", ";\n"]);
        let field = optional;
        f.write_strs([" optional: ", "optional: "]);
        optional.syntax_fmt(f)?;
        f.write_strs(["", ""]);
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "test_optional_field"]
#[doc(hidden)]
pub const test_optional_field: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_optional_field"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 62usize,
        start_col: 4usize,
        end_line: 62usize,
        end_col: 23usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_optional_field()),
    ),
};
fn test_optional_field() {
    let with_opt = WithOptional {
        required: "req",
        optional: Some("opt"),
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&with_opt)))
        }),
        &"required: req; optional: opt",
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
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&with_opt).pretty()))
        }),
        &"required: req;\noptional: opt",
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
    let without_opt = WithOptional {
        required: "req",
        optional: None,
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&without_opt)))
        }),
        &"required: req;",
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
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&without_opt).pretty()))
        }),
        &"required: req;\n",
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
enum SimpleEnum<'src> {
    #[syntax(content = "super")]
    Super,
    Ident(&'src str),
}
impl<'src, __SyntaxFmtState> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>
for SimpleEnum<'src>
where
    &'src str: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
{
    fn syntax_fmt(
        &self,
        f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>,
    ) -> ::std::fmt::Result {
        let field = self;
        match self {
            Self::Super => {
                let field = self;
                ("super").syntax_fmt(f)?;
            }
            Self::Ident(__0) => {
                let field = self;
                let field = __0;
                __0.syntax_fmt(f)?;
            }
            _ => {}
        }
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "test_enum"]
#[doc(hidden)]
pub const test_enum: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_enum"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 80usize,
        start_col: 4usize,
        end_line: 80usize,
        end_col: 13usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(test_enum())),
};
fn test_enum() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&SimpleEnum::Super)))
        }),
        &"super",
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
            ::alloc::fmt::format(
                format_args!("{0}", syntax_fmt(&SimpleEnum::Super).pretty()),
            )
        }),
        &"super",
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
            ::alloc::fmt::format(
                format_args!("{0}", syntax_fmt(&SimpleEnum::Ident("foo"))),
            )
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
            ::alloc::fmt::format(
                format_args!("{0}", syntax_fmt(&SimpleEnum::Ident("foo")).pretty()),
            )
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
}
struct FunctionDecl<'src> {
    #[syntax(format = "pub {*}", eval = *field)]
    is_pub: bool,
    #[syntax(format = "fn {*}")]
    name: &'src str,
}
impl<'src, __SyntaxFmtState> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>
for FunctionDecl<'src>
where
    bool: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
    &'src str: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
{
    fn syntax_fmt(
        &self,
        f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>,
    ) -> ::std::fmt::Result {
        let field = self;
        let Self { is_pub, name } = self;
        let field = is_pub;
        if (*field) {
            f.write_strs(["pub ", "pub "]);
            is_pub.syntax_fmt(f)?;
            f.write_strs(["", ""]);
        }
        let field = name;
        f.write_strs(["fn ", "fn "]);
        name.syntax_fmt(f)?;
        f.write_strs(["", ""]);
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "test_bool_field"]
#[doc(hidden)]
pub const test_bool_field: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_bool_field"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 96usize,
        start_col: 4usize,
        end_line: 96usize,
        end_col: 19usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_bool_field()),
    ),
};
fn test_bool_field() {
    let pub_fn = FunctionDecl {
        is_pub: true,
        name: "test",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&pub_fn)))
        }),
        &"pub fn test",
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
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&pub_fn).pretty()))
        }),
        &"pub fn test",
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
    let priv_fn = FunctionDecl {
        is_pub: false,
        name: "test",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&priv_fn)))
        }),
        &"fn test",
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
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&priv_fn).pretty()))
        }),
        &"fn test",
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
struct WithContentLiteral<'src> {
    #[syntax(content = "name: CUSTOM")]
    name: &'src str,
}
impl<'src, __SyntaxFmtState> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>
for WithContentLiteral<'src>
where
    &'src str: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
{
    fn syntax_fmt(
        &self,
        f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>,
    ) -> ::std::fmt::Result {
        let field = self;
        let Self { name } = self;
        let field = name;
        ("name: CUSTOM").syntax_fmt(f)?;
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "test_format_literal"]
#[doc(hidden)]
pub const test_format_literal: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_format_literal"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 113usize,
        start_col: 4usize,
        end_line: 113usize,
        end_col: 23usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_format_literal()),
    ),
};
fn test_format_literal() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!("{0}", syntax_fmt(&WithContentLiteral { name: "foo" })),
            )
        }),
        &"name: CUSTOM",
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
            ::alloc::fmt::format(
                format_args!(
                    "{0}",
                    syntax_fmt(&WithContentLiteral { name: "foo" }).pretty(),
                ),
            )
        }),
        &"name: CUSTOM",
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
fn custom_formatter<State>(
    value: &str,
    ctx: &mut SyntaxFormatter<State>,
) -> std::fmt::Result {
    ctx.write_fmt(format_args!("{{{0}}} ", value))
}
struct WithCustomFormatter<'src> {
    #[syntax(format = "value: {*}", content = custom_formatter)]
    value: &'src str,
}
impl<'src, __SyntaxFmtState> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>
for WithCustomFormatter<'src>
where
    &'src str: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
{
    fn syntax_fmt(
        &self,
        f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>,
    ) -> ::std::fmt::Result {
        let field = self;
        let Self { value } = self;
        let field = value;
        f.write_strs(["value: ", "value: "]);
        (custom_formatter)(field, f)?;
        f.write_strs(["", ""]);
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "test_custom_formatter"]
#[doc(hidden)]
pub const test_custom_formatter: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_custom_formatter"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 130usize,
        start_col: 4usize,
        end_line: 130usize,
        end_col: 25usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_custom_formatter()),
    ),
};
fn test_custom_formatter() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!(
                    "{0}",
                    syntax_fmt(
                        &WithCustomFormatter {
                            value: "test",
                        },
                    ),
                ),
            )
        }),
        &"value: {test} ",
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
            ::alloc::fmt::format(
                format_args!(
                    "{0}",
                    syntax_fmt(
                            &WithCustomFormatter {
                                value: "test",
                            },
                        )
                        .pretty(),
                ),
            )
        }),
        &"value: {test} ",
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
trait NameResolver {
    fn resolve_name(&self, id: &str) -> String;
}
struct TestResolver;
impl NameResolver for TestResolver {
    fn resolve_name(&self, id: &str) -> String {
        ::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("resolved_{0}", id))
        })
    }
}
fn resolve_formatter<State: NameResolver>(
    value: &str,
    ctx: &mut SyntaxFormatter<State>,
) -> std::fmt::Result {
    let resolved = ctx.state().resolve_name(value);
    ctx.write_fmt(format_args!("{0}", resolved))
}
#[syntax(state_bound = NameResolver)]
struct WithStatefulFormatter<'src> {
    #[syntax(format = "id: {*}", content = resolve_formatter)]
    id: &'src str,
}
impl<'src, __SyntaxFmtState> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>
for WithStatefulFormatter<'src>
where
    __SyntaxFmtState: NameResolver,
    &'src str: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
{
    fn syntax_fmt(
        &self,
        f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>,
    ) -> ::std::fmt::Result {
        let field = self;
        let Self { id } = self;
        let field = id;
        f.write_strs(["id: ", "id: "]);
        (resolve_formatter)(field, f)?;
        f.write_strs(["", ""]);
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "test_stateful_formatter"]
#[doc(hidden)]
pub const test_stateful_formatter: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_stateful_formatter"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 161usize,
        start_col: 4usize,
        end_line: 161usize,
        end_col: 27usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_stateful_formatter()),
    ),
};
fn test_stateful_formatter() {
    let mut resolver = TestResolver;
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!(
                    "{0}",
                    syntax_fmt(&WithStatefulFormatter { id: "foo" })
                        .state_mut(&mut resolver),
                ),
            )
        }),
        &"id: resolved_foo",
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
            ::alloc::fmt::format(
                format_args!(
                    "{0}",
                    syntax_fmt(&WithStatefulFormatter { id: "foo" })
                        .state_mut(&mut resolver),
                ),
            )
        }),
        &"id: resolved_foo",
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
struct Module<'src> {
    #[syntax(format = "mod {*}")]
    name: &'src str,
    #[syntax(
        format = (" {{{*}}}", " {{\n{*}}}"),
        indent_region,
        eval = |v:&Items<'src>|!v.is_empty(),
    )]
    items: Items<'src>,
}
impl<'src, __SyntaxFmtState> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState> for Module<'src>
where
    &'src str: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
    Items<'src>: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
{
    fn syntax_fmt(
        &self,
        f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>,
    ) -> ::std::fmt::Result {
        let field = self;
        let Self { name, items } = self;
        let field = name;
        f.write_strs(["mod ", "mod "]);
        name.syntax_fmt(f)?;
        f.write_strs(["", ""]);
        let field = items;
        if (|v: &Items<'src>| !v.is_empty())(field) {
            f.write_strs([" {{", " {{\n"]);
            f.push_indent();
            items.syntax_fmt(f)?;
            f.pop_indent();
            f.write_strs(["}}", "}}"]);
        }
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "test_indent_and_empty_suffix"]
#[doc(hidden)]
pub const test_indent_and_empty_suffix: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_indent_and_empty_suffix"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 181usize,
        start_col: 4usize,
        end_line: 181usize,
        end_col: 32usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_indent_and_empty_suffix()),
    ),
};
fn test_indent_and_empty_suffix() {
    let empty = Module {
        name: "empty",
        items: Items(::alloc::vec::Vec::new()),
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&empty)))
        }),
        &"mod empty;",
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
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&empty).pretty()))
        }),
        &"mod empty;",
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
    let with_items = Module {
        name: "lib",
        items: Items(
            <[_]>::into_vec(
                ::alloc::boxed::box_new([Statement("item1"), Statement("item2")]),
            ),
        ),
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&with_items)))
        }),
        &"mod lib {item1, item2}",
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
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&with_items).pretty()))
        }),
        &"mod lib {\n    item1\n    item2\n}",
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
#[syntax(format = ("&{*}", "ref {*}"))]
struct RefType<'src> {
    #[syntax(content = "mut ")]
    is_mut: bool,
    value: &'src str,
}
impl<'src, __SyntaxFmtState> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState> for RefType<'src>
where
    bool: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
    &'src str: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
{
    fn syntax_fmt(
        &self,
        f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>,
    ) -> ::std::fmt::Result {
        let field = self;
        f.write_strs(["&", "ref "]);
        let Self { is_mut, value } = self;
        let field = is_mut;
        ("mut ").syntax_fmt(f)?;
        let field = value;
        value.syntax_fmt(f)?;
        f.write_strs(["", ""]);
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "test_outer_format"]
#[doc(hidden)]
pub const test_outer_format: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_outer_format"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 201usize,
        start_col: 4usize,
        end_line: 201usize,
        end_col: 21usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_outer_format()),
    ),
};
fn test_outer_format() {
    let mut_ref = RefType {
        is_mut: true,
        value: "x",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&mut_ref)))
        }),
        &"&mut x",
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
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&mut_ref).pretty()))
        }),
        &"ref mut x",
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
    let immut_ref = RefType {
        is_mut: false,
        value: "x",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&immut_ref)))
        }),
        &"&x",
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
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&immut_ref).pretty()))
        }),
        &"ref x",
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
struct Ident<'src>(&'src str);
impl<'src, __SyntaxFmtState> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState> for Ident<'src>
where
    &'src str: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
{
    fn syntax_fmt(
        &self,
        f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>,
    ) -> ::std::fmt::Result {
        let field = self;
        let Self(__0) = self;
        let field = __0;
        __0.syntax_fmt(f)?;
        Ok(())
    }
}
struct Collections<'src> {
    #[syntax(delim = ", ")]
    vec: Vec<Ident<'src>>,
    #[syntax(delim = ", ")]
    slice: &'src [Ident<'src>],
    #[syntax(delim = ", ")]
    array: [Ident<'src>; 2],
}
impl<'src, __SyntaxFmtState> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>
for Collections<'src>
where
    Vec<Ident<'src>>: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
    &'src [Ident<'src>]: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
    [Ident<'src>; 2]: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
{
    fn syntax_fmt(
        &self,
        f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>,
    ) -> ::std::fmt::Result {
        let field = self;
        let Self { vec, slice, array } = self;
        let field = vec;
        f.push_delim([", ", ", "]);
        vec.syntax_fmt(f)?;
        f.pop_delim();
        let field = slice;
        f.push_delim([", ", ", "]);
        slice.syntax_fmt(f)?;
        f.pop_delim();
        let field = array;
        f.push_delim([", ", ", "]);
        array.syntax_fmt(f)?;
        f.pop_delim();
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "test_collections"]
#[doc(hidden)]
pub const test_collections: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_collections"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 226usize,
        start_col: 4usize,
        end_line: 226usize,
        end_col: 20usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_collections()),
    ),
};
fn test_collections() {
    let idents = [Ident("a"), Ident("b")];
    let c = Collections {
        vec: <[_]>::into_vec(::alloc::boxed::box_new([Ident("foo"), Ident("bar")])),
        slice: &idents,
        array: [Ident("x"), Ident("y")],
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&c)))
        }),
        &"foo, bara, bx, y",
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
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&c).pretty()))
        }),
        &"foo, bara, bx, y",
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
struct PathSegment<'src>(&'src str);
impl<'src, __SyntaxFmtState> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>
for PathSegment<'src>
where
    &'src str: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
{
    fn syntax_fmt(
        &self,
        f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>,
    ) -> ::std::fmt::Result {
        let field = self;
        let Self(__0) = self;
        let field = __0;
        __0.syntax_fmt(f)?;
        Ok(())
    }
}
struct QualifiedPath<'src> {
    #[syntax(delim = ("::", " :: "))]
    segments: Vec<PathSegment<'src>>,
}
impl<'src, __SyntaxFmtState> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>
for QualifiedPath<'src>
where
    Vec<PathSegment<'src>>: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
{
    fn syntax_fmt(
        &self,
        f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>,
    ) -> ::std::fmt::Result {
        let field = self;
        let Self { segments } = self;
        let field = segments;
        f.push_delim(["::", " :: "]);
        segments.syntax_fmt(f)?;
        f.pop_delim();
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "test_collection_with_custom_delim"]
#[doc(hidden)]
pub const test_collection_with_custom_delim: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_collection_with_custom_delim"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 248usize,
        start_col: 4usize,
        end_line: 248usize,
        end_col: 37usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_collection_with_custom_delim()),
    ),
};
fn test_collection_with_custom_delim() {
    let path = QualifiedPath {
        segments: <[_]>::into_vec(
            ::alloc::boxed::box_new([
                PathSegment("std"),
                PathSegment("collections"),
                PathSegment("HashMap"),
            ]),
        ),
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&path)))
        }),
        &"std::collections::HashMap",
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
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&path).pretty()))
        }),
        &"std :: collections :: HashMap",
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
struct Item<'src>(&'src str);
impl<'src, __SyntaxFmtState> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState> for Item<'src>
where
    &'src str: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
{
    fn syntax_fmt(
        &self,
        f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>,
    ) -> ::std::fmt::Result {
        let field = self;
        let Self(__0) = self;
        let field = __0;
        __0.syntax_fmt(f)?;
        Ok(())
    }
}
struct List<'src> {
    #[syntax(format = ("[{*}]", "[\n{*}\n]"), indent_region, delim = (", ", ",\n"))]
    items: Vec<Item<'src>>,
}
impl<'src, __SyntaxFmtState> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState> for List<'src>
where
    Vec<Item<'src>>: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>,
{
    fn syntax_fmt(
        &self,
        f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>,
    ) -> ::std::fmt::Result {
        let field = self;
        let Self { items } = self;
        let field = items;
        f.write_strs(["[", "[\n"]);
        f.push_delim([", ", ",\n"]);
        f.push_indent();
        items.syntax_fmt(f)?;
        f.pop_indent();
        f.pop_delim();
        f.write_strs(["]", "\n]"]);
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "test_collection_with_wrapper"]
#[doc(hidden)]
pub const test_collection_with_wrapper: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_collection_with_wrapper"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 267usize,
        start_col: 4usize,
        end_line: 267usize,
        end_col: 32usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_collection_with_wrapper()),
    ),
};
fn test_collection_with_wrapper() {
    let list = List {
        items: <[_]>::into_vec(
            ::alloc::boxed::box_new([Item("a"), Item("b"), Item("c")]),
        ),
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&list)))
        }),
        &"[a, b, c]",
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
            ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&list).pretty()))
        }),
        &"[\n    a,\n    b,\n    c\n]",
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
struct Counter {
    count: usize,
}
struct CountedItem;
impl SyntaxFmt<Counter> for CountedItem {
    fn syntax_fmt(&self, ctx: &mut SyntaxFormatter<Counter>) -> std::fmt::Result {
        let count = ctx.state_mut().count;
        ctx.state_mut().count += 1;
        ctx.write_fmt(format_args!("item_{0}", count))
    }
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
        start_line: 289usize,
        start_col: 4usize,
        end_line: 289usize,
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
    let mut state = Counter { count: 0 };
    let item = CountedItem;
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!("{0}", syntax_fmt(&item).state_mut(&mut state)),
            )
        }),
        &"item_0",
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
    match (&state.count, &1) {
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
                format_args!("{0}", syntax_fmt(&item).state_mut(&mut state).pretty()),
            )
        }),
        &"item_1",
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
    match (&state.count, &2) {
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
#[rustc_test_marker = "test_immutable_state_panics_on_mut_access"]
#[doc(hidden)]
pub const test_immutable_state_panics_on_mut_access: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_immutable_state_panics_on_mut_access"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/syntaxfmt-macros/tests/basic.rs",
        start_line: 301usize,
        start_col: 4usize,
        end_line: 301usize,
        end_col: 45usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::YesWithMessage("StateRef: state is immutable"),
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_immutable_state_panics_on_mut_access()),
    ),
};
#[should_panic(expected = "StateRef: state is immutable")]
fn test_immutable_state_panics_on_mut_access() {
    struct BadItem;
    impl SyntaxFmt<Counter> for BadItem {
        fn syntax_fmt(&self, ctx: &mut SyntaxFormatter<Counter>) -> std::fmt::Result {
            ctx.state_mut().count += 1;
            Ok(())
        }
    }
    let counter = Counter { count: 0 };
    let _ = ::alloc::__export::must_use({
        ::alloc::fmt::format(format_args!("{0}", syntax_fmt(&BadItem).state(&counter)))
    });
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[
            &test_basic_struct,
            &test_bool_field,
            &test_collection_with_custom_delim,
            &test_collection_with_wrapper,
            &test_collections,
            &test_custom_formatter,
            &test_enum,
            &test_format_literal,
            &test_immutable_state_panics_on_mut_access,
            &test_indent_and_empty_suffix,
            &test_mutable_state,
            &test_optional_field,
            &test_outer_format,
            &test_stateful_formatter,
        ],
    )
}
