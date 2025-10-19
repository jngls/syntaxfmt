use syntaxfmt_macros::SyntaxFmt as SyntaxFmtDerive;
use syntaxfmt::{syntax_fmt, syntax_fmt_pretty, SyntaxFmt, SyntaxFmtContext};

#[track_caller]
fn assert_formats<State, T: SyntaxFmt<State>>(
    state: &State,
    value: &T,
    expected_normal: &str,
    expected_pretty: &str,
) {
    assert_eq!(format!("{}", syntax_fmt(state, value)), expected_normal);
    assert_eq!(format!("{}", syntax_fmt_pretty(state, value)), expected_pretty);
}

// Shared test types
#[derive(SyntaxFmtDerive)]
#[syntax(delim = ", ", pretty_delim = "")]
struct Statement<'src>(#[syntax(format = "{content}", pretty_format = "{content}\n")] &'src str);

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
    let with = WithOptional {
        required: "req",
        optional: Some("opt"),
    };
    assert_formats(&(), &with, "required: req; optional: opt", "required: req;\noptional: opt");

    let without = WithOptional {
        required: "req",
        optional: None,
    };
    assert_formats(&(), &without, "required: req;", "required: req;\n");
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
struct ModuleLike<'src> {
    #[syntax(format = "mod {content}")]
    name: &'src str,
    #[syntax(
        format = " {{{content}}}",
        pretty_format = " {{\n{content}}}",
        indent_inc
    )]
    items: Items<'src>,
}

#[test]
fn test_indent_inc_with_module() {
    let empty = ModuleLike {
        name: "foo",
        items: Items(vec![]),
    };
    assert_formats(&(), &empty, "mod foo {}", "mod foo {\n}");

    let with_items = ModuleLike {
        name: "foo",
        items: Items(vec![Statement("item1"), Statement("item2")]),
    };
    assert_formats(
        &(),
        &with_items,
        "mod foo {item1, item2}",
        "mod foo {\n    item1\n    item2\n}"
    );
}

#[derive(SyntaxFmtDerive)]
struct Block<'src> {
    #[syntax(format = "{{\n{content}}}", pretty_format = "{{\n{content}}}", indent, indent_inc)]
    statements: Items<'src>,
}

#[test]
fn test_indent_attribute() {
    let b = Block {
        statements: Items(vec![Statement("let x = 1;"), Statement("let y = 2;")]),
    };
    assert_formats(
        &(),
        &b,
        "{\nlet x = 1;, let y = 2;}",
        "{\n    let x = 1;\n    let y = 2;\n}"
    );
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
    let public = FunctionDecl {
        is_pub: true,
        name: "test",
    };
    assert_formats(&(), &public, "pub fn test", "pub fn test");

    let private = FunctionDecl {
        is_pub: false,
        name: "test",
    };
    assert_formats(&(), &private, "fn test", "fn test");
}

#[derive(SyntaxFmtDerive)]
struct WithFormatLiteral<'src> {
    #[allow(unused)]
    #[syntax(format = "name: CUSTOM")]
    name: &'src str,
}

#[test]
fn test_format_literal() {
    let s = WithFormatLiteral { name: "foo" };
    assert_formats(&(), &s, "name: CUSTOM", "name: CUSTOM");
}

// Custom formatters
fn custom_formatter<State>(value: &str, ctx: &mut SyntaxFmtContext<State>) -> std::fmt::Result {
    write!(ctx, "{{{}}} ", value)
}

fn uppercase_formatter<State>(value: &&str, ctx: &mut SyntaxFmtContext<State>) -> std::fmt::Result {
    write!(ctx, "{}", value.to_uppercase())
}

#[derive(SyntaxFmtDerive)]
struct WithCustomFormatters<'src> {
    #[syntax(format = "value: {content}", content = custom_formatter)]
    value: &'src str,
    #[syntax(content = uppercase_formatter)]
    text: &'src str,
}

#[test]
fn test_custom_formatters() {
    let s = WithCustomFormatters {
        value: "test",
        text: "hello",
    };
    assert_formats(&(), &s, "value: {test} HELLO", "value: {test} HELLO");
}

// Stateful formatter test
trait NameResolver {
    fn resolve_name(&self, id: &str) -> String;
}

struct TestResolver;

impl NameResolver for TestResolver {
    fn resolve_name(&self, id: &str) -> String {
        format!("resolved_{}", id)
    }
}

fn resolve_formatter<State: NameResolver>(value: &&str, ctx: &mut SyntaxFmtContext<State>) -> std::fmt::Result {
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
    let s = WithStatefulFormatter { id: "foo" };
    let resolver = TestResolver;
    assert_formats(&resolver, &s, "id: resolved_foo", "id: resolved_foo");
}

// Outer format tests
#[derive(SyntaxFmtDerive)]
#[syntax(format = "&{content}")]
struct RefType<'src> {
    #[syntax(format = "mut ")]
    is_mut: bool,
    value: &'src str,
}

#[test]
fn test_outer_format() {
    let mutable = RefType {
        is_mut: true,
        value: "x",
    };
    assert_formats(&(), &mutable, "&mut x", "&mut x");

    let immutable = RefType {
        is_mut: false,
        value: "x",
    };
    assert_formats(&(), &immutable, "&x", "&x");
}

#[derive(SyntaxFmtDerive)]
#[syntax(format = "({content})", pretty_format = "[ {content} ]")]
struct Wrapped<'src> {
    value: &'src str,
}

#[test]
fn test_outer_format_pretty_variant() {
    let w = Wrapped { value: "test" };
    assert_formats(&(), &w, "(test)", "[ test ]");
}
