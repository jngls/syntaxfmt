#![allow(unreachable_patterns)]
#![allow(unused)]


use syntaxfmt_macros::SyntaxFmt as SyntaxFmtDerive;
use syntaxfmt::{syntax_fmt, Mode, SyntaxFmt, SyntaxFormatter};

// Shared test types
#[derive(SyntaxFmtDerive)]
struct Statement<'src>(#[syntax(format = ("{*}", "{*}\n"))] &'src str);

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

#[derive(SyntaxFmtDerive)]
struct SimpleStruct<'src> {
    #[syntax(format = "name: {*}")]
    name: &'src str,
}

#[test]
fn test_basic_struct() {
    let s = SimpleStruct { name: "foo" };
    assert_eq!(format!("{}", syntax_fmt(&s)), "name: foo");
    assert_eq!(format!("{}", syntax_fmt(&s).pretty()), "name: foo");
}

#[derive(SyntaxFmtDerive)]
struct WithOptional<'src> {
    #[syntax(format = ("required: {*};", "required: {*};\n"))]
    required: &'src str,
    #[syntax(format = (" optional: {*}", "optional: {*}"))]
    optional: Option<&'src str>,
}

#[test]
fn test_optional_field() {
    let with_opt = WithOptional { required: "req", optional: Some("opt") };
    assert_eq!(format!("{}", syntax_fmt(&with_opt)), "required: req; optional: opt");
    assert_eq!(format!("{}", syntax_fmt(&with_opt).pretty()), "required: req;\noptional: opt");

    let without_opt = WithOptional { required: "req", optional: None };
    assert_eq!(format!("{}", syntax_fmt(&without_opt)), "required: req;");
    assert_eq!(format!("{}", syntax_fmt(&without_opt).pretty()), "required: req;\n");
}

#[derive(SyntaxFmtDerive)]
enum SimpleEnum<'src> {
    #[syntax(content = "super")]
    Super,
    Ident(&'src str),
}

#[test]
fn test_enum() {
    assert_eq!(format!("{}", syntax_fmt(&SimpleEnum::Super)), "super");
    assert_eq!(format!("{}", syntax_fmt(&SimpleEnum::Super).pretty()), "super");
    assert_eq!(format!("{}", syntax_fmt(&SimpleEnum::Ident("foo"))), "foo");
    assert_eq!(format!("{}", syntax_fmt(&SimpleEnum::Ident("foo")).pretty()), "foo");
}

#[derive(SyntaxFmtDerive)]
struct FunctionDecl<'src> {
    #[syntax(format = "pub {*}", eval = *field)]
    is_pub: bool,
    #[syntax(format = "fn {*}")]
    name: &'src str,
}

#[test]
fn test_bool_field() {
    let pub_fn = FunctionDecl { is_pub: true, name: "test" };
    assert_eq!(format!("{}", syntax_fmt(&pub_fn)), "pub fn test");
    assert_eq!(format!("{}", syntax_fmt(&pub_fn).pretty()), "pub fn test");

    let priv_fn = FunctionDecl { is_pub: false, name: "test" };
    assert_eq!(format!("{}", syntax_fmt(&priv_fn)), "fn test");
    assert_eq!(format!("{}", syntax_fmt(&priv_fn).pretty()), "fn test");
}

#[derive(SyntaxFmtDerive)]
struct WithContentLiteral<'src> {
    #[syntax(content = "name: CUSTOM")]
    name: &'src str,
}

#[test]
fn test_format_literal() {
    assert_eq!(format!("{}", syntax_fmt(&WithContentLiteral { name: "foo" })), "name: CUSTOM");
    assert_eq!(format!("{}", syntax_fmt(&WithContentLiteral { name: "foo" }).pretty()), "name: CUSTOM");
}

// Custom formatters
fn custom_formatter<State>(value: &str, ctx: &mut SyntaxFormatter<State>) -> std::fmt::Result {
    write!(ctx, "{{{}}} ", value)
}

#[derive(SyntaxFmtDerive)]
struct WithCustomFormatter<'src> {
    #[syntax(format = "value: {*}", content = custom_formatter)]
    value: &'src str,
}

#[test]
fn test_custom_formatter() {
    assert_eq!(format!("{}", syntax_fmt(&WithCustomFormatter { value: "test" })), "value: {test} ");
    assert_eq!(format!("{}", syntax_fmt(&WithCustomFormatter { value: "test" }).pretty()), "value: {test} ");
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

fn resolve_formatter<State: NameResolver>(value: &str, ctx: &mut SyntaxFormatter<State>) -> std::fmt::Result {
    let resolved = ctx.state().resolve_name(value);
    write!(ctx, "{}", resolved)
}

#[derive(SyntaxFmtDerive)]
#[syntax(state_bound = NameResolver)]
struct WithStatefulFormatter<'src> {
    #[syntax(format = "id: {*}", content = resolve_formatter)]
    id: &'src str,
}

#[test]
fn test_stateful_formatter() {
    let mut resolver = TestResolver;
    assert_eq!(format!("{}", syntax_fmt(&WithStatefulFormatter { id: "foo" }).state_mut(&mut resolver)), "id: resolved_foo");
    assert_eq!(format!("{}", syntax_fmt(&WithStatefulFormatter { id: "foo" }).state_mut(&mut resolver)), "id: resolved_foo");
}

// Module with indentation and none output
#[derive(SyntaxFmtDerive)]
struct Module<'src> {
    #[syntax(format = "mod {*}")]
    name: &'src str,
    #[syntax(
        format = (" {{{*}}}", " {{\n{*}}}"),
        indent_region,
        eval = |v: &Items<'src>| !v.is_empty(),
        else_content = ";",
    )]
    items: Items<'src>,
}

#[test]
fn test_indent_and_empty_suffix() {
    let empty = Module { name: "empty", items: Items(vec![]) };
    assert_eq!(format!("{}", syntax_fmt(&empty)), "mod empty;");
    assert_eq!(format!("{}", syntax_fmt(&empty).pretty()), "mod empty;");

    let with_items = Module { name: "lib", items: Items(vec![Statement("item1"), Statement("item2")]) };
    assert_eq!(format!("{}", syntax_fmt(&with_items)), "mod lib {item1, item2}");
    assert_eq!(format!("{}", syntax_fmt(&with_items).pretty()), "mod lib {\n    item1\n    item2\n}");
}

// Outer format with pretty variant
#[derive(SyntaxFmtDerive)]
#[syntax(format = ("&{*}", "ref {*}"))]
struct RefType<'src> {
    #[syntax(content = "mut ")]
    is_mut: bool,
    value: &'src str,
}

#[test]
fn test_outer_format() {
    let mut_ref = RefType { is_mut: true, value: "x" };
    assert_eq!(format!("{}", syntax_fmt(&mut_ref)), "&mut x");
    assert_eq!(format!("{}", syntax_fmt(&mut_ref).pretty()), "ref mut x");

    let immut_ref = RefType { is_mut: false, value: "x" };
    assert_eq!(format!("{}", syntax_fmt(&immut_ref)), "&x");
    assert_eq!(format!("{}", syntax_fmt(&immut_ref).pretty()), "ref x");
}

// Collections (Vec, slice, array all work the same)
#[derive(SyntaxFmtDerive)]
struct Ident<'src>(&'src str);

#[derive(SyntaxFmtDerive)]
struct Collections<'src> {
    #[syntax(delim = ", ")]
    vec: Vec<Ident<'src>>,
    #[syntax(delim = ", ")]
    slice: &'src [Ident<'src>],
    #[syntax(delim = ", ")]
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
    assert_eq!(format!("{}", syntax_fmt(&c)), "foo, bara, bx, y");
    assert_eq!(format!("{}", syntax_fmt(&c).pretty()), "foo, bara, bx, y");
}

// Custom delimiter
#[derive(SyntaxFmtDerive)]
struct PathSegment<'src>(&'src str);

#[derive(SyntaxFmtDerive)]
struct QualifiedPath<'src> {
    #[syntax(delim = ("::", " :: "))]
    segments: Vec<PathSegment<'src>>,
}

#[test]
fn test_collection_with_custom_delim() {
    let path = QualifiedPath {
        segments: vec![PathSegment("std"), PathSegment("collections"), PathSegment("HashMap")],
    };
    assert_eq!(format!("{}", syntax_fmt(&path)), "std::collections::HashMap");
    assert_eq!(format!("{}", syntax_fmt(&path).pretty()), "std :: collections :: HashMap");
}

// Collection with wrapper and indentation
#[derive(SyntaxFmtDerive)]
struct Item<'src>(&'src str);

#[derive(SyntaxFmtDerive)]
struct List<'src> {
    #[syntax(format = ("[{*}]", "[\n{*}\n]"), indent_region, delim = (", ", ",\n"))]
    items: Vec<Item<'src>>,
}

#[test]
fn test_collection_with_wrapper() {
    let list = List { items: vec![Item("a"), Item("b"), Item("c")] };
    assert_eq!(format!("{}", syntax_fmt(&list)), "[a, b, c]");
    assert_eq!(format!("{}", syntax_fmt(&list).pretty()), "[\n    a,\n    b,\n    c\n]");
}

// Mutable state
struct Counter {
    count: usize,
}

struct CountedItem;

impl SyntaxFmt<Counter> for CountedItem {
    fn syntax_fmt(&self, ctx: &mut SyntaxFormatter<Counter>) -> std::fmt::Result {
        let count = ctx.state_mut().count;
        ctx.state_mut().count += 1;
        write!(ctx, "item_{}", count)
    }
}

#[test]
fn test_mutable_state() {
    let mut state = Counter { count: 0 };
    let item = CountedItem;

    assert_eq!(format!("{}", syntax_fmt(&item).state_mut(&mut state)), "item_0");
    assert_eq!(state.count, 1);
    assert_eq!(format!("{}", syntax_fmt(&item).state_mut(&mut state).pretty()), "item_1");
    assert_eq!(state.count, 2);
}

#[test]
#[should_panic(expected = "StateRef: state is immutable")]
fn test_immutable_state_panics_on_mut_access() {
    struct BadItem;
    impl SyntaxFmt<Counter> for BadItem {
        fn syntax_fmt(&self, ctx: &mut SyntaxFormatter<Counter>) -> std::fmt::Result {
            ctx.state_mut().count += 1; // Should panic!
            Ok(())
        }
    }

    let counter = Counter { count: 0 };
    let _ = format!("{}", syntax_fmt(&BadItem).state(&counter));
}
