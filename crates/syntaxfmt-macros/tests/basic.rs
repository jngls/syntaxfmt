use syntaxfmt_macros::SyntaxFmt as SyntaxFmtDerive;
use syntaxfmt::{syntax_fmt, syntax_fmt_mut, SyntaxFmt, SyntaxFmtContext};

#[track_caller]
fn assert_formats<State, T: SyntaxFmt<State>>(
    state: &State,
    value: &T,
    expected_normal: &str,
    expected_pretty: &str,
) {
    assert_eq!(format!("{}", syntax_fmt(state, value)), expected_normal);
    assert_eq!(format!("{}", syntax_fmt(state, value).pretty()), expected_pretty);
}

// Shared test types
#[derive(SyntaxFmtDerive)]
#[syntax(delim = ", ", pretty_delim = "")]
struct Statement<'src>(#[syntax(pretty_format = "{content}\n")] &'src str);

struct Items<'src>(Vec<Statement<'src>>);

impl<'src> Items<'src> {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'src> SyntaxFmt<()> for Items<'src> {
    fn syntax_fmt(&self, ctx: &mut SyntaxFmtContext<()>) -> ::std::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }
        let delim = if ctx.is_pretty() {
            <Statement as SyntaxFmt<()>>::PRETTY_DELIM
        } else {
            <Statement as SyntaxFmt<()>>::DELIM
        };
        for (i, item) in self.0.iter().enumerate() {
            if i > 0 {
                write!(ctx, "{}", delim)?;
            }
            if ctx.is_pretty() {
                ctx.indent(Self::INDENT)?;
            }
            item.syntax_fmt(ctx)?;
        }
        Ok(())
    }
}

#[derive(SyntaxFmtDerive)]
struct SimpleStruct<'src> {
    #[syntax(format = "name: {content}")]
    name: &'src str,
}

#[test]
fn test_basic_struct() {
    let s = SimpleStruct { name: "foo" };
    assert_formats(&(), &s, "name: foo", "name: foo");
}

#[derive(SyntaxFmtDerive)]
struct WithOptional<'src> {
    #[syntax(format = "required: {content};", pretty_format = "required: {content};\n")]
    required: &'src str,
    #[syntax(format = " optional: {content}", pretty_format = "optional: {content}")]
    optional: Option<&'src str>,
}

#[test]
fn test_optional_field() {
    assert_formats(&(), &WithOptional { required: "req", optional: Some("opt") },
        "required: req; optional: opt", "required: req;\noptional: opt");
    assert_formats(&(), &WithOptional { required: "req", optional: None },
        "required: req;", "required: req;\n");
}

#[derive(SyntaxFmtDerive)]
#[syntax(delim = "::", pretty_delim = "::")]
enum SimpleEnum<'src> {
    #[syntax(format = "super")]
    Super,
    Ident(&'src str),
}

#[test]
fn test_enum() {
    assert_formats(&(), &SimpleEnum::Super, "super", "super");
    assert_formats(&(), &SimpleEnum::Ident("foo"), "foo", "foo");
}

#[derive(SyntaxFmtDerive)]
struct FunctionDecl<'src> {
    #[syntax(format = "pub ")]
    is_pub: bool,
    #[syntax(format = "fn {content}")]
    name: &'src str,
}

#[test]
fn test_bool_field() {
    assert_formats(&(), &FunctionDecl { is_pub: true, name: "test" }, "pub fn test", "pub fn test");
    assert_formats(&(), &FunctionDecl { is_pub: false, name: "test" }, "fn test", "fn test");
}

#[derive(SyntaxFmtDerive)]
struct WithFormatLiteral<'src> {
    #[allow(unused)]
    #[syntax(format = "name: CUSTOM")]
    name: &'src str,
}

#[test]
fn test_format_literal() {
    assert_formats(&(), &WithFormatLiteral { name: "foo" }, "name: CUSTOM", "name: CUSTOM");
}

// Custom formatters
fn custom_formatter<State>(value: &str, ctx: &mut SyntaxFmtContext<State>) -> std::fmt::Result {
    write!(ctx, "{{{}}} ", value)
}

#[derive(SyntaxFmtDerive)]
struct WithCustomFormatter<'src> {
    #[syntax(format = "value: {content}", content = custom_formatter)]
    value: &'src str,
}

#[test]
fn test_custom_formatter() {
    assert_formats(&(), &WithCustomFormatter { value: "test" }, "value: {test} ", "value: {test} ");
}

// Stateful formatter
trait NameResolver {
    fn resolve_name(&self, id: &str) -> String;
}

struct TestResolver;

impl NameResolver for TestResolver {
    fn resolve_name(&self, id: &str) -> String {
        format!("resolved_{}", id)
    }
}

fn resolve_formatter<State: NameResolver>(value: &str, ctx: &mut SyntaxFmtContext<State>) -> std::fmt::Result {
    let resolved = ctx.state().resolve_name(value);
    write!(ctx, "{}", resolved)
}

#[derive(SyntaxFmtDerive)]
#[syntax(state_bound = "NameResolver")]
struct WithStatefulFormatter<'src> {
    #[syntax(format = "id: {content}", content = resolve_formatter)]
    id: &'src str,
}

#[test]
fn test_stateful_formatter() {
    assert_formats(&TestResolver, &WithStatefulFormatter { id: "foo" }, "id: resolved_foo", "id: resolved_foo");
}

// Module with indentation and empty_suffix
#[derive(SyntaxFmtDerive)]
struct Module<'src> {
    #[syntax(format = "mod {content}")]
    name: &'src str,
    #[syntax(
        format = " {{{content}}}",
        pretty_format = " {{\n{content}}}",
        empty_suffix = ";",
        indent_inc
    )]
    items: Items<'src>,
}

#[test]
fn test_indent_and_empty_suffix() {
    assert_formats(&(), &Module { name: "empty", items: Items(vec![]) },
        "mod empty;", "mod empty;");
    assert_formats(&(), &Module { name: "lib", items: Items(vec![Statement("item1"), Statement("item2")]) },
        "mod lib {item1, item2}", "mod lib {\n    item1\n    item2\n}");
}

// Outer format with pretty variant
#[derive(SyntaxFmtDerive)]
#[syntax(format = "&{content}", pretty_format = "ref {content}")]
struct RefType<'src> {
    #[syntax(format = "mut ")]
    is_mut: bool,
    value: &'src str,
}

#[test]
fn test_outer_format() {
    assert_formats(&(), &RefType { is_mut: true, value: "x" }, "&mut x", "ref mut x");
    assert_formats(&(), &RefType { is_mut: false, value: "x" }, "&x", "ref x");
}

// Collections (Vec, slice, array all work the same)
#[derive(SyntaxFmtDerive)]
#[syntax(delim = ", ", pretty_delim = ", ")]
struct Ident<'src>(&'src str);

#[derive(SyntaxFmtDerive)]
struct Collections<'src> {
    vec: Vec<Ident<'src>>,
    slice: &'src [Ident<'src>],
    array: [Ident<'src>; 2],
}

#[test]
fn test_collections() {
    let idents = [Ident("a"), Ident("b")];
    let c = Collections {
        vec: vec![Ident("foo"), Ident("bar")],
        slice: &idents,
        array: [Ident("x"), Ident("y")],
    };
    assert_formats(&(), &c, "foo, bara, bx, y", "foo, bara, bx, y");
}

// Custom delimiter
#[derive(SyntaxFmtDerive)]
#[syntax(delim = "::", pretty_delim = " :: ")]
struct PathSegment<'src>(&'src str);

#[derive(SyntaxFmtDerive)]
struct QualifiedPath<'src> {
    segments: Vec<PathSegment<'src>>,
}

#[test]
fn test_collection_with_custom_delim() {
    let path = QualifiedPath {
        segments: vec![PathSegment("std"), PathSegment("collections"), PathSegment("HashMap")],
    };
    assert_formats(&(), &path, "std::collections::HashMap", "std :: collections :: HashMap");
}

// Collection with wrapper and indentation
#[derive(SyntaxFmtDerive)]
#[syntax(delim = ", ", pretty_delim = ",\n")]
struct Item<'src>(&'src str);

#[derive(SyntaxFmtDerive)]
struct List<'src> {
    #[syntax(format = "[{content}]", pretty_format = "[\n{content}\n]", indent_inc)]
    items: Vec<Item<'src>>,
}

#[test]
fn test_collection_with_wrapper() {
    assert_formats(&(), &List { items: vec![Item("a"), Item("b"), Item("c")] },
        "[a, b, c]", "[\n    a,\n    b,\n    c\n]");
}

// Mutable state
struct Counter {
    count: usize,
}

struct CountedItem;

impl SyntaxFmt<Counter> for CountedItem {
    fn syntax_fmt(&self, ctx: &mut SyntaxFmtContext<Counter>) -> std::fmt::Result {
        let count = ctx.state_mut().count;
        ctx.state_mut().count += 1;
        write!(ctx, "item_{}", count)
    }
}

#[test]
fn test_mutable_state() {
    let mut state = Counter { count: 0 };
    let item = CountedItem;

    assert_eq!(format!("{}", syntax_fmt_mut(&mut state, &item)), "item_0");
    assert_eq!(state.count, 1);
    assert_eq!(format!("{}", syntax_fmt_mut(&mut state, &item).pretty()), "item_1");
    assert_eq!(state.count, 2);
}

#[test]
#[should_panic(expected = "StateRef: state is immutable")]
fn test_immutable_state_panics_on_mut_access() {
    struct BadItem;
    impl SyntaxFmt<Counter> for BadItem {
        fn syntax_fmt(&self, ctx: &mut SyntaxFmtContext<Counter>) -> std::fmt::Result {
            ctx.state_mut().count += 1; // Should panic!
            Ok(())
        }
    }

    let _ = format!("{}", syntax_fmt(&Counter { count: 0 }, &BadItem));
}
