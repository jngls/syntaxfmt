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
    #[syntax(suf = ": ")]
    name: Ident,
    ty: Type,
}

#[derive(SyntaxFmtDerive)]
struct Block {
    #[syntax(pre = " {", suf = "}", nl = cont, ind, sep = "")]
    statements: Vec<Statement>,
}

#[derive(SyntaxFmtDerive)]
#[syntax(suf = ";", nl = beg)]
enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
}

#[derive(SyntaxFmtDerive)]
#[syntax(pre = "let ")]
struct LetStatement {
    name: Ident,
    #[syntax(pre = " = ", eval = value.is_some())]
    value: Option<Ident>,
}

#[derive(SyntaxFmtDerive)]
#[syntax(pre = "return")]
struct ReturnStatement {
    #[syntax(pre = " ")]
    value: Ident,
}

#[derive(SyntaxFmtDerive)]
struct Function {
    // "pub " literal when public, empty otherwise
    #[syntax(cont = "pub ", eval = *is_pub)]
    is_pub: bool,

    // "fn name"
    #[syntax(pre = "fn ")]
    name: Ident,

    // Parameters with separators and wrapping
    #[syntax(pre = "(", suf = ")", sep = ", ")]
    params: Vec<Param>,

    // Optional return type
    #[syntax(pre = " -> ", eval = return_type.is_some())]
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
                    value: Ident("result"),
                }),
            ],
        },
    };

    let normal = format!("{}", syntax_fmt(&func));
    assert_eq!(
        normal,
        "pub fn add(a: i32, b: i32) -> i32 {let result = a + b;return result;}"
    );

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
        name: Ident("nop"),
        params: vec![],
        return_type: None,
        body: Block { statements: vec![] },
    };
    
    let normal = format!("{}", syntax_fmt(&func));
    assert_eq!(normal, "fn nop() {}");
    
    let pretty = format!("{}", syntax_fmt(&func).pretty());
    assert_eq!(pretty, "fn nop() {\n}");
}
