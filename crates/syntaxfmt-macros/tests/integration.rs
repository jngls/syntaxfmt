use syntaxfmt_macros::SyntaxFmt as SyntaxFmtDerive;
use syntaxfmt::syntax_fmt;

// =============================================================================
// Rust-like function declaration AST
// =============================================================================

#[derive(SyntaxFmtDerive)]
struct Ident(&'static str);

#[derive(SyntaxFmtDerive)]
struct Type(&'static str);

#[derive(SyntaxFmtDerive)]
struct Param {
    #[syntax(fmt = "{*}: ")]
    name: Ident,
    ty: Type,
}

#[derive(SyntaxFmtDerive)]
struct Block {
    #[syntax(fmt = " {{*}}", nl = con, ind, delim = "")]
    statements: Vec<Statement>,
}

#[derive(SyntaxFmtDerive)]
#[syntax(fmt = "{*};", nl = beg)]
enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
}

#[derive(SyntaxFmtDerive)]
#[syntax(fmt = "let {*}")]
struct LetStatement {
    name: Ident,
    #[syntax(fmt = " = {*}", eval = value.is_some())]
    value: Option<Ident>,
}

#[derive(SyntaxFmtDerive)]
#[syntax(fmt = "return{*}")]
struct ReturnStatement {
    #[syntax(fmt = " {*}", eval = value.is_some())]
    value: Option<Ident>,
}

#[derive(SyntaxFmtDerive)]
struct Function {
    // "pub " literal when public, empty otherwise
    #[syntax(cont = "pub ", eval = *is_pub)]
    is_pub: bool,

    // "fn name"
    #[syntax(fmt = "fn {*}")]
    name: Ident,

    // Parameters with delimiters and wrapping
    #[syntax(fmt = "({*})", delim = ", ")]
    params: Vec<Param>,

    // Optional return type
    #[syntax(fmt = " -> {*}", eval = return_type.is_some())]
    return_type: Option<Type>,

    // Function body
    body: Block,
}

#[test]
fn test_function_integration() {
    let func = Function {
        is_pub: true,
        name: Ident("add"),
        params: vec![
            Param { name: Ident("a"), ty: Type("i32") },
            Param { name: Ident("b"), ty: Type("i32") },
        ],
        return_type: Some(Type("i32")),
        body: Block {
            statements: vec![
                Statement::Let(LetStatement {
                    name: Ident("result"),
                    value: Some(Ident("a + b")),
                }),
                Statement::Return(ReturnStatement {
                    value: Some(Ident("result")),
                }),
            ],
        },
    };

    // Normal mode: compact
    let normal = format!("{}", syntax_fmt(&func));
    assert_eq!(
        normal,
        "pub fn add(a: i32, b: i32) -> i32 {let result = a + b;return result;}"
    );

    // Pretty mode: with proper indentation
    let pretty = format!("{}", syntax_fmt(&func).pretty());
    assert_eq!(
        pretty,
        "pub fn add(a: i32, b: i32) -> i32 {\n    let result = a + b;\n    return result;\n}"
    );
}

#[test]
fn test_function_without_return_type() {
    let func = Function {
        is_pub: false,
        name: Ident("print_hello"),
        params: vec![],
        return_type: None,
        body: Block {
            statements: vec![
                Statement::Return(ReturnStatement {
                    value: None,
                }),
            ],
        },
    };
    
    let normal = format!("{}", syntax_fmt(&func));
    assert_eq!(normal, "fn print_hello() {return;}");
    
    let pretty = format!("{}", syntax_fmt(&func).pretty());
    assert_eq!(pretty, "fn print_hello() {\n    return;\n}"
    );
}
