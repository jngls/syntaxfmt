#![allow(unused)]

use std::marker::PhantomData;

use syntaxfmt::{Mode, SyntaxFmt, SyntaxFormatter, syntax_fmt};
use syntaxfmt_macros::SyntaxFmt as SyntaxFmtDerive;

// =============================================================================
// empty structs
// =============================================================================
#[derive(SyntaxFmtDerive)]
struct Unit;

#[test]
fn test_unit() {
    assert_eq!(format!("{}", syntax_fmt(&Unit)), "");
}

#[derive(SyntaxFmtDerive)]
struct EmptyNamed {}

#[test]
fn test_empty_named() {
    assert_eq!(format!("{}", syntax_fmt(&EmptyNamed {})), "");
}

#[derive(SyntaxFmtDerive)]
struct EmptyUnnamed();

#[test]
fn test_empty_unnamed() {
    assert_eq!(format!("{}", syntax_fmt(&EmptyUnnamed())), "");
}

// =============================================================================
// skip
// =============================================================================

#[derive(SyntaxFmtDerive)]
struct WithSkip {
    visible: &'static str,
    #[syntax(skip)]
    invisible: &'static str,
}

#[test]
fn test_skip() {
    let s = WithSkip {
        visible: "visible",
        invisible: "invisible",
    };
    assert_eq!(format!("{}", syntax_fmt(&s)), "visible");
}

#[derive(SyntaxFmtDerive)]
enum EnumWithSkip {
    Visible(&'static str),
    #[syntax(skip)]
    Invisible(&'static str),
}

#[test]
fn test_skip_variant() {
    let visible = EnumWithSkip::Visible("visible");
    assert_eq!(format!("{}", syntax_fmt(&visible)), "visible");

    let invisible = EnumWithSkip::Invisible("invisible");
    assert_eq!(format!("{}", syntax_fmt(&invisible)), "");
}

#[derive(SyntaxFmtDerive)]
#[syntax(skip)]
struct WithOuterSkip {
    invisible: &'static str,
}

#[test]
fn test_outer_skip() {
    let s = WithOuterSkip {
        invisible: "invisible",
    };
    assert_eq!(format!("{}", syntax_fmt(&s)), "");
}

// =============================================================================
// newlines, indenting and pretty printing
// =============================================================================

#[derive(SyntaxFmtDerive)]
struct WithIndent {
    header: &'static str,
    #[syntax(ind, nl = inner)]
    body: &'static str,
    footer: &'static str,
}

#[test]
fn test_indent_pretty() {
    let s = WithIndent {
        header: "indent {",
        body: "foo",
        footer: "}",
    };
    assert_eq!(format!("{}", syntax_fmt(&s)), "indent {foo}");
    assert_eq!(
        format!("{}", syntax_fmt(&s).pretty()),
        "indent {\n    foo\n}"
    );
}

#[derive(SyntaxFmtDerive)]
struct WithNestedIndent {
    header: &'static str,
    // We're using explicit fields here to add header and footer to better isolate tests.
    // But in practice you could just use: `ind, nl = inner, pre = "header" suf = "footer"`
    // to add header, footer, and appropriate newlines.
    #[syntax(ind, nl = inner)]
    body: WithIndent,
    footer: &'static str,
}

#[test]
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
    assert_eq!(format!("{}", syntax_fmt(&s)), "outer {inner {foo}}");
    assert_eq!(
        format!("{}", syntax_fmt(&s).pretty()),
        "outer {\n    inner {\n        foo\n    }\n}"
    );
}

// We need to force a newline with this one - in practice, newline would come
// from the previous element
#[derive(SyntaxFmtDerive)]
#[syntax(ind, nl = pre)]
struct WithOuterIndent {
    indented: &'static str,
}

#[test]
fn test_outer_indent_pretty() {
    let s = WithOuterIndent { indented: "foo" };
    assert_eq!(format!("{}", syntax_fmt(&s)), "foo");
    assert_eq!(format!("{}", syntax_fmt(&s).pretty()), "\n    foo");
}

// =============================================================================
// prefix and suffix
// =============================================================================

#[derive(SyntaxFmtDerive)]
struct WithPreSuf {
    #[syntax(pre = "prefix_", suf = "_suffix")]
    field: &'static str,
}

#[test]
fn test_format_prefix_suffix() {
    let s = WithPreSuf { field: "value" };
    assert_eq!(format!("{}", syntax_fmt(&s)), "prefix_value_suffix");
}

#[derive(SyntaxFmtDerive)]
struct WithModalPreSuf {
    // NOTE - modal args follow order defined in syntaxfmt::Mode
    #[syntax(pre = ["normal[", "pretty[ "], suf = ["]", " ]"])]
    field: &'static str,
}

#[test]
fn test_modal_format() {
    let s = WithModalPreSuf { field: "x" };
    assert_eq!(format!("{}", syntax_fmt(&s)), "normal[x]");
    assert_eq!(format!("{}", syntax_fmt(&s).pretty()), "pretty[ x ]");
}

#[derive(SyntaxFmtDerive)]
#[syntax(pre = "outer<", suf = ">")]
struct WithOuterPreSuf {
    inner: &'static str,
}

#[test]
fn test_format_outer() {
    let s = WithOuterPreSuf { inner: "value" };
    assert_eq!(format!("{}", syntax_fmt(&s)), "outer<value>");
}

// =============================================================================
// content - basic forms
// =============================================================================

#[derive(SyntaxFmtDerive)]
struct WithContentLiteral {
    #[syntax(cont = "LITERAL")]
    field: i32,
}

#[test]
fn test_content_literal() {
    let s = WithContentLiteral { field: 42 };
    assert_eq!(format!("{}", syntax_fmt(&s)), "LITERAL");
}

#[derive(SyntaxFmtDerive)]
struct WithModalContent {
    #[syntax(cont = ["normal", "pretty"])]
    field: &'static str,
}

#[test]
fn test_modal_content() {
    let s = WithModalContent { field: "ignored" };
    assert_eq!(format!("{}", syntax_fmt(&s)), "normal");
    assert_eq!(format!("{}", syntax_fmt(&s).pretty()), "pretty");
}

#[derive(SyntaxFmtDerive)]
#[syntax(cont = ["normal", "pretty"])]
struct WithOuterContent {
    field: &'static str,
}

#[test]
fn test_outer_content() {
    let s = WithOuterContent { field: "ignored" };
    assert_eq!(format!("{}", syntax_fmt(&s)), "normal");
    assert_eq!(format!("{}", syntax_fmt(&s).pretty()), "pretty");
}

// =============================================================================
// content - path or closure treated as formatter
// =============================================================================

fn my_formatter<S>(field: &str, f: &mut SyntaxFormatter<S>) -> std::fmt::Result {
    write!(f, "formatted[{}]", field)
}

#[derive(SyntaxFmtDerive)]
struct WithContentPath {
    #[syntax(cont_with = my_formatter)]
    field: &'static str,
}

#[test]
fn test_content_path_as_formatter() {
    let s = WithContentPath { field: "value" };
    assert_eq!(format!("{}", syntax_fmt(&s)), "formatted[value]");
}

#[derive(SyntaxFmtDerive)]
struct WithContentClosure {
    #[syntax(cont_with = |field: &str, f: &mut SyntaxFormatter<_>| write!(f, "closure[{}]", field))]
    field: &'static str,
}

#[test]
fn test_content_closure_as_formatter() {
    let s = WithContentClosure { field: "value" };
    assert_eq!(format!("{}", syntax_fmt(&s)), "closure[value]");
}

fn my_struct_formatter<S>(
    _struct: &WithOuterContentPath,
    f: &mut SyntaxFormatter<S>,
) -> std::fmt::Result {
    write!(f, "formatted[{}]", _struct.field)
}

#[derive(SyntaxFmtDerive)]
#[syntax(cont_with = my_struct_formatter)]
struct WithOuterContentPath {
    field: &'static str,
}

#[test]
fn test_outer_content_path_as_formatter() {
    let s = WithOuterContentPath { field: "value" };
    assert_eq!(format!("{}", syntax_fmt(&s)), "formatted[value]");
}

#[derive(SyntaxFmtDerive)]
#[syntax(cont_with = |_struct: &Self, f: &mut SyntaxFormatter<_>| write!(f, "closure[{}]", _struct.field))]
struct WithOuterContentClosure {
    field: &'static str,
}

#[test]
fn test_outer_content_closure_as_formatter() {
    let s = WithOuterContentClosure { field: "value" };
    assert_eq!(format!("{}", syntax_fmt(&s)), "closure[value]");
}

// =============================================================================
// content - slices are modal
// =============================================================================

#[derive(SyntaxFmtDerive)]
struct WithContentModalSlice {
    #[syntax(cont = ["normal", "pretty"])]
    field: &'static str,
}

#[test]
fn test_content_modal_slice() {
    let s = WithContentModalSlice { field: "value" };
    assert_eq!(format!("{}", syntax_fmt(&s)), "normal");
    assert_eq!(format!("{}", syntax_fmt(&s).pretty()), "pretty");
}

#[derive(SyntaxFmtDerive)]
#[syntax(cont = ["normal", "pretty"])]
struct WithOuterContentModalSlice {
    field: &'static str,
}

#[test]
fn test_outer_content_modal_slice() {
    let s = WithOuterContentModalSlice { field: "value" };
    assert_eq!(format!("{}", syntax_fmt(&s)), "normal");
    assert_eq!(format!("{}", syntax_fmt(&s).pretty()), "pretty");
}

// =============================================================================
// sep
// =============================================================================

#[derive(SyntaxFmtDerive)]
struct SepItem(&'static str);

#[derive(SyntaxFmtDerive)]
struct WithSep {
    #[syntax(sep = "|")]
    items: Vec<SepItem>,
}

#[test]
fn test_sep() {
    let s = WithSep {
        items: vec![SepItem("a"), SepItem("b"), SepItem("c")],
    };
    assert_eq!(format!("{}", syntax_fmt(&s)), "a|b|c");
}

#[derive(SyntaxFmtDerive)]
struct WithModalSep {
    #[syntax(sep = [":", ": "])]
    items: Vec<SepItem>,
}

#[test]
fn test_modal_sep() {
    let s = WithModalSep {
        items: vec![SepItem("a"), SepItem("b"), SepItem("c")],
    };
    assert_eq!(format!("{}", syntax_fmt(&s)), "a:b:c");
    assert_eq!(format!("{}", syntax_fmt(&s).pretty()), "a: b: c");
}

// Outer `sep` should insert separators between fields
#[derive(SyntaxFmtDerive)]
#[syntax(sep = "+")]
struct WithOuterSep {
    item_a: i32,
    item_b: &'static str,
    item_c: bool,
}

#[test]
fn test_outer_sep() {
    let s = WithOuterSep {
        item_a: 42,
        item_b: "foo",
        item_c: true,
    };
    assert_eq!(format!("{}", syntax_fmt(&s)), "42+foo+true");
}

// =============================================================================
// eval - basic (referencing fields by name)
// =============================================================================

#[derive(SyntaxFmtDerive)]
struct WithEvalBasic {
    #[syntax(eval = *non_zero > 0)]
    non_zero: u32,
}

#[test]
fn test_eval_basic() {
    let enabled = WithEvalBasic { non_zero: 123 };
    assert_eq!(format!("{}", syntax_fmt(&enabled)), "123");

    let disabled = WithEvalBasic { non_zero: 0 };
    assert_eq!(format!("{}", syntax_fmt(&disabled)), "");
}

#[derive(SyntaxFmtDerive)]
struct MaybeKeyword {
    value: &'static str,
}

#[derive(SyntaxFmtDerive)]
struct WithEvalMember {
    #[syntax(eval = maybe_keyword.value == "self")]
    maybe_keyword: MaybeKeyword,
}

#[test]
fn test_eval_member() {
    let enabled = WithEvalMember {
        maybe_keyword: MaybeKeyword { value: "self" },
    };
    assert_eq!(format!("{}", syntax_fmt(&enabled)), "self");

    let disabled = WithEvalMember {
        maybe_keyword: MaybeKeyword { value: "other" },
    };
    assert_eq!(format!("{}", syntax_fmt(&disabled)), "");
}

#[derive(SyntaxFmtDerive)]
#[syntax(eval = self.non_zero > 0)]
struct WithEvalOuter {
    non_zero: u32,
}

#[test]
fn test_outer_eval() {
    let enabled = WithEvalOuter { non_zero: 123 };
    assert_eq!(format!("{}", syntax_fmt(&enabled)), "123");

    let disabled = WithEvalOuter { non_zero: 0 };
    assert_eq!(format!("{}", syntax_fmt(&disabled)), "");
}

// =============================================================================
// eval - closures and paths have field passed to them
// =============================================================================

fn is_long(s: &str) -> bool {
    s.len() > 5
}

#[derive(SyntaxFmtDerive)]
struct WithEvalPath {
    #[syntax(eval_with = is_long)]
    text: &'static str,
}

#[test]
fn test_eval_path() {
    let long = WithEvalPath {
        text: "verylongtext",
    };
    assert_eq!(format!("{}", syntax_fmt(&long)), "verylongtext");

    let short = WithEvalPath { text: "short" };
    assert_eq!(format!("{}", syntax_fmt(&short)), "");
}

#[derive(SyntaxFmtDerive)]
struct WithEvalClosure {
    #[syntax(eval_with = |s: &str| s.contains('p'))]
    text: &'static str,
}

#[test]
fn test_eval_closure() {
    let with_a = WithEvalClosure { text: "apple" };
    assert_eq!(format!("{}", syntax_fmt(&with_a)), "apple");

    let without_a = WithEvalClosure { text: "orange" };
    assert_eq!(format!("{}", syntax_fmt(&without_a)), "");
}

fn is_long_outer(s: &WithOuterEvalPath) -> bool {
    s.text.len() > 5
}

#[derive(SyntaxFmtDerive)]
#[syntax(eval_with = is_long_outer)]
struct WithOuterEvalPath {
    text: &'static str,
}

#[test]
fn test_outer_eval_path() {
    let long = WithOuterEvalPath {
        text: "verylongtext",
    };
    assert_eq!(format!("{}", syntax_fmt(&long)), "verylongtext");

    let short = WithOuterEvalPath { text: "short" };
    assert_eq!(format!("{}", syntax_fmt(&short)), "");
}

#[derive(SyntaxFmtDerive)]
#[syntax(eval_with = |s: &Self| s.text.contains('p'))]
struct WithOuterEvalClosure {
    text: &'static str,
}

#[test]
fn test_outer_eval_closure() {
    let with_a = WithOuterEvalClosure { text: "apple" };
    assert_eq!(format!("{}", syntax_fmt(&with_a)), "apple");

    let without_a = WithOuterEvalClosure { text: "orange" };
    assert_eq!(format!("{}", syntax_fmt(&without_a)), "");
}

// =============================================================================
// eval and else
// =============================================================================

#[derive(SyntaxFmtDerive)]
struct WithElseContent {
    #[syntax(eval = value.is_some())]
    #[syntax_else(cont = "none")]
    value: Option<&'static str>,
}

#[test]
fn test_eval_else_content() {
    let some = WithElseContent {
        value: Some("data"),
    };
    assert_eq!(format!("{}", syntax_fmt(&some)), "data");

    let none = WithElseContent { value: None };
    assert_eq!(format!("{}", syntax_fmt(&none)), "none");
}

// =============================================================================
// state - immutable and mutable
// =============================================================================

trait Resolver {
    fn resolve(&self, name: &str) -> String;
}

struct TestResolver;

impl Resolver for TestResolver {
    fn resolve(&self, name: &str) -> String {
        format!("resolved_{}", name)
    }
}

fn resolve_formatter(field: &str, f: &mut SyntaxFormatter<TestResolver>) -> std::fmt::Result {
    let resolved = f.state().resolve(field);
    write!(f, "{}", resolved)
}

#[derive(SyntaxFmtDerive)]
#[syntax(state = TestResolver)]
struct WithImmutable {
    #[syntax(cont_with = resolve_formatter)]
    name: &'static str,
}

#[test]
fn test_immutable_state() {
    let resolver = TestResolver;
    let s = WithImmutable { name: "foo" };
    assert_eq!(
        format!("{}", syntax_fmt(&s).state(&resolver)),
        "resolved_foo"
    );
}

fn resolve_formatter_bounded<S: Resolver>(
    field: &str,
    f: &mut SyntaxFormatter<S>,
) -> std::fmt::Result {
    let resolved = f.state().resolve(field);
    write!(f, "{}", resolved)
}

#[derive(SyntaxFmtDerive)]
#[syntax(bound = Resolver)]
struct WithImmutableBounded {
    #[syntax(cont_with = resolve_formatter_bounded)]
    name: &'static str,
}

#[test]
fn test_immutable_state_bounded() {
    let resolver = TestResolver;
    let s = WithImmutableBounded { name: "foo" };
    assert_eq!(
        format!("{}", syntax_fmt(&s).state(&resolver)),
        "resolved_foo"
    );
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

fn counting_formatter(field: &str, f: &mut SyntaxFormatter<TestCounter>) -> std::fmt::Result {
    let count = f.state_mut().post_inc();
    write!(f, "{}#{}", field, count)
}

#[derive(SyntaxFmtDerive)]
#[syntax(state = TestCounter)]
struct WithMutableState {
    #[syntax(cont_with = counting_formatter)]
    name: &'static str,
}

#[test]
fn test_mutable_state() {
    let mut counter = TestCounter { count: 0 };
    let s = WithMutableState { name: "item" };

    assert_eq!(
        format!("{}", syntax_fmt(&s).state_mut(&mut counter)),
        "item#0"
    );
    assert_eq!(counter.count, 1);

    assert_eq!(
        format!("{}", syntax_fmt(&s).state_mut(&mut counter)),
        "item#1"
    );
    assert_eq!(counter.count, 2);
}

fn counting_formatter_bounded<S: Counter>(
    field: &str,
    f: &mut SyntaxFormatter<S>,
) -> std::fmt::Result {
    let count = f.state_mut().post_inc();
    write!(f, "{}#{}", field, count)
}

#[derive(SyntaxFmtDerive)]
#[syntax(bound = Counter)]
struct WithMutableStateBounded {
    #[syntax(cont_with = counting_formatter_bounded)]
    name: &'static str,
}

#[test]
fn test_mutable_state_bounded() {
    let mut counter = TestCounter { count: 0 };
    let s = WithMutableStateBounded { name: "item" };

    assert_eq!(
        format!("{}", syntax_fmt(&s).state_mut(&mut counter)),
        "item#0"
    );
    assert_eq!(counter.count, 1);

    assert_eq!(
        format!("{}", syntax_fmt(&s).state_mut(&mut counter)),
        "item#1"
    );
    assert_eq!(counter.count, 2);
}

trait TraitLifetime<'a> {}

struct StateLifetime<'a>(PhantomData<&'a i32>);

impl<'a> TraitLifetime<'a> for StateLifetime<'a> {}

fn dummy_formatter<'a>(
    _: &WithStateLifetime,
    f: &mut SyntaxFormatter<StateLifetime<'a>>,
) -> std::fmt::Result {
    write!(f, "state lifetime works")
}

fn dummy_formatter_bounded<'a, S: TraitLifetime<'a>>(
    _: &WithBoundLifetime,
    f: &mut SyntaxFormatter<S>,
) -> std::fmt::Result {
    write!(f, "bound lifetime works")
}

fn dummy_formatter_same_lifetime<'a, S: TraitLifetime<'a>>(
    _: &WithSameLifetime<'a>,
    f: &mut SyntaxFormatter<S>,
) -> std::fmt::Result {
    write!(f, "bound lifetime works")
}

#[derive(SyntaxFmtDerive)]
#[syntax(state = StateLifetime<'a>, cont_with = dummy_formatter)]
struct WithStateLifetime {}

#[test]
fn test_lifetime_state() {
    let lifetime_state = StateLifetime(Default::default());

    assert_eq!(
        format!(
            "{}",
            syntax_fmt(&WithStateLifetime {}).state(&lifetime_state)
        ),
        "state lifetime works"
    );
}

#[derive(SyntaxFmtDerive)]
#[syntax(bound = TraitLifetime<'a>, cont_with = dummy_formatter_bounded)]
struct WithBoundLifetime {}

#[test]
fn test_lifetime_bound() {
    let lifetime_state = StateLifetime(Default::default());

    assert_eq!(
        format!(
            "{}",
            syntax_fmt(&WithBoundLifetime {}).state(&lifetime_state)
        ),
        "bound lifetime works"
    );
}

#[derive(SyntaxFmtDerive)]
#[syntax(bound = TraitLifetime<'a>, cont_with = dummy_formatter_same_lifetime)]
struct WithSameLifetime<'a> {
    #[syntax(skip)]
    _marker: PhantomData<&'a i32>,
}

impl<'a> TraitLifetime<'a> for WithSameLifetime<'a> {}

#[test]
fn test_same_lifetime() {
    let lifetime_state = StateLifetime(Default::default());

    assert_eq!(
        format!(
            "{}",
            syntax_fmt(&WithSameLifetime { _marker: Default::default() }).state(&lifetime_state)
        ),
        "bound lifetime works"
    );
}

// =============================================================================
// map_state and map_state_mut
// =============================================================================

fn map_state_formatter(field: &str, f: &mut SyntaxFormatter<TestCounter>) -> std::fmt::Result {
    f.map_state(|f, state| write!(f, "{}#{}", field, state.count))
}

#[derive(SyntaxFmtDerive)]
#[syntax(state = TestCounter)]
struct WithMapState {
    #[syntax(cont_with = map_state_formatter)]
    name: &'static str,
}

#[test]
fn test_map_state() {
    let counter = TestCounter { count: 42 };
    let s = WithMapState { name: "item" };

    assert_eq!(format!("{}", syntax_fmt(&s).state(&counter)), "item#42");
    // Counter should not be modified
    assert_eq!(counter.count, 42);
}

fn map_state_mut_formatter<S: Counter>(
    field: &str,
    f: &mut SyntaxFormatter<S>,
) -> std::fmt::Result {
    f.map_state_mut(|f, state| {
        let count = state.post_inc();
        write!(f, "{}#{}", field, count)
    })
}

#[derive(SyntaxFmtDerive)]
#[syntax(bound = Counter)]
struct WithMapStateMut {
    #[syntax(cont_with = map_state_mut_formatter)]
    name: &'static str,
}

#[test]
fn test_map_state_mut() {
    let mut counter = TestCounter { count: 0 };
    let s = WithMapStateMut { name: "item" };

    assert_eq!(
        format!("{}", syntax_fmt(&s).state_mut(&mut counter)),
        "item#0"
    );
    assert_eq!(counter.count, 1);

    assert_eq!(
        format!("{}", syntax_fmt(&s).state_mut(&mut counter)),
        "item#1"
    );
    assert_eq!(counter.count, 2);
}

// =============================================================================
// access mutable state in immutable context (should panic)
// =============================================================================

#[test]
#[should_panic(expected = "StateRef: state is immutable")]
fn test_immutable_context_mut_access_panics() {
    fn bad_formatter(field: &str, f: &mut SyntaxFormatter<TestCounter>) -> std::fmt::Result {
        f.state_mut().post_inc(); // This should panic!
        write!(f, "{}", field)
    }

    #[derive(SyntaxFmtDerive)]
    #[syntax(state = TestCounter)]
    struct Bad {
        #[syntax(cont_with = bad_formatter)]
        name: &'static str,
    }

    let counter = TestCounter { count: 0 };
    let s = Bad { name: "test" };
    let _ = format!("{}", syntax_fmt(&s).state(&counter));
}

// =============================================================================
// recursive types
// =============================================================================

// These tests mainly ensure the type compiles without infinite trait resolution
// The failure case is actually that compiling fails, either by hanging, or by
// emitting `error[E0275]: overflow evaluating the requirement`

trait DummyBound {}
struct DummyState;
impl DummyBound for DummyState {}

#[derive(SyntaxFmtDerive)]
struct BasicRecursive<'a> {
    recursive: Box<BasicRecursive<'a>>,
    _marker: PhantomData<&'a i32>,
}

#[derive(SyntaxFmtDerive)]
#[syntax(bound = DummyBound)]
struct MutualRecursiveA<'a> {
    b: MutualRecursiveB<'a>,
    _marker: PhantomData<&'a i32>,
}

#[derive(SyntaxFmtDerive)]
#[syntax(bound = DummyBound)]
struct MutualRecursiveB<'a> {
    a: Vec<MutualRecursiveA<'a>>,
    _marker: PhantomData<&'a i32>,
}

#[derive(SyntaxFmtDerive)]
#[syntax(bound = DummyBound)]
enum MutualRecursiveEnumA<'a> {
    B(MutualRecursiveEnumB<'a>, PhantomData<&'a i32>),
    End,
}

#[derive(SyntaxFmtDerive)]
#[syntax(bound = DummyBound)]
struct MutualRecursiveEnumB<'a> {
    a: Vec<MutualRecursiveEnumA<'a>>,
}

#[test]
fn test_basic_recursion_struct() {
    // This would trigger infinite trait resolution if there's an issue with generated code bounds
    let state = DummyState;
    let a = MutualRecursiveA {
        b: MutualRecursiveB {
            a: vec![],
            _marker: PhantomData,
        },
        _marker: PhantomData,
    };
    let _ = format!("{}", syntax_fmt(&a).state(&state));
}

#[test]
fn test_mutual_recursion_struct() {
    // This would trigger infinite trait resolution if there's an issue with generated code bounds
    let state = DummyState;
    let a = MutualRecursiveA {
        b: MutualRecursiveB {
            a: vec![],
            _marker: PhantomData,
        },
        _marker: PhantomData,
    };
    let _ = format!("{}", syntax_fmt(&a).state(&state));
}

#[test]
fn test_mutual_recursion_enum() {
    // This would trigger infinite trait resolution if there's an issue with generated code bounds
    let state = DummyState;
    let b = MutualRecursiveEnumB { a: vec![MutualRecursiveEnumA::End] };
    let _ = format!("{}", syntax_fmt(&b).state(&state));
}

// =============================================================================
// macro wrapped
// =============================================================================

// This test mainly ensures the type compiles when wrapped in a macro
// Had a weird error where self values were getting resolved incorrectly but only in
// macros - presumably related to macro hygeine.

macro_rules! impl_in_macro {
    ($name:ident) => {
        #[derive(SyntaxFmtDerive)]
        struct $name(i32);
    };
}
impl_in_macro!(InMacro);

#[test]
fn test_in_macro() {
    let _ = format!("{}", syntax_fmt(&InMacro(123)));
}
