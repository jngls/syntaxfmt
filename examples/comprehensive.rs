use syntaxfmt::{SyntaxFmt, SyntaxFormatter, syntax_fmt};

// State trait for type display control
trait TypeDisplay {
    fn should_show_types(&self) -> bool;
}

struct TypeConfig {
    show_types: bool,
}

impl TypeDisplay for TypeConfig {
    fn should_show_types(&self) -> bool {
        self.show_types
    }
}

// Enum with multiple variants
#[derive(SyntaxFmt)]
#[syntax(bound = TypeDisplay)]
enum Expr<'src> {
    #[syntax(cont_with = |expr: &Expr, f: &mut SyntaxFormatter<_>| {
        if let Expr::Literal(val) = expr {
            write!(f, "{}", val)
        } else {
            Ok(())
        }
    })]
    Literal(i32),

    #[syntax(pre = "(", suf = ")")]
    Binary {
        #[syntax(suf = " ")]
        left: Box<Expr<'src>>,

        #[syntax(suf = " ")]
        op: &'src str,

        right: Box<Expr<'src>>,
    },

    Call(FunctionCall<'src>),
}

// Nested struct with collections
#[derive(SyntaxFmt)]
#[syntax(bound = TypeDisplay)]
struct FunctionCall<'src> {
    name: &'src str,

    #[syntax(pre = "(", suf = ")", sep = [", ", ", "])]
    args: Vec<Expr<'src>>,
}

// Helper for conditional type display based on state
fn format_type_with_state<S: TypeDisplay>(
    ty: &Option<&str>,
    f: &mut SyntaxFormatter<S>
) -> std::fmt::Result {
    if f.state().should_show_types() {
        if let Some(t) = ty {
            write!(f, ": {}", t)
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}

// Parameter with conditional type annotation using state
#[derive(SyntaxFmt)]
#[syntax(bound = TypeDisplay)]
struct Parameter<'src> {
    name: &'src str,

    #[syntax(eval = ty.is_some(), cont_with = format_type_with_state)]
    ty: Option<&'src str>,
}

// Statement with pretty printing
#[derive(SyntaxFmt)]
#[syntax(bound = TypeDisplay, nl = beg)]
enum Statement<'src> {
    #[syntax(pre = "return ", suf = ";", eval = value.is_some())]
    #[syntax_else(cont = "return;")]
    Return { value: Option<Expr<'src>> },

    #[syntax(suf = ";")]
    Expr(Expr<'src>),
}

// Block with indentation
#[derive(SyntaxFmt)]
#[syntax(pre = "{", suf = "}", bound = TypeDisplay)]
struct Block<'src> {
    #[syntax(nl = [cont], ind, sep = "")]
    statements: Vec<Statement<'src>>,
}

// Function with all features combined
#[derive(SyntaxFmt)]
#[syntax(pre = "fn ", bound = TypeDisplay)]
struct Function<'src> {
    name: &'src str,

    #[syntax(pre = "(", suf = ")", sep = [", ", ", "])]
    params: Vec<Parameter<'src>>,

    #[syntax(pre = " -> ", eval = return_type.is_some())]
    return_type: Option<&'src str>,

    #[syntax(pre = " ")]
    body: Block<'src>,
}

fn main() {
    // Create example AST
    let func = Function {
        name: "calculate",
        params: vec![
            Parameter { name: "x", ty: Some("i32") },
            Parameter { name: "y", ty: Some("i32") },
        ],
        return_type: Some("i32"),
        body: Block {
            statements: vec![
                Statement::Expr(Expr::Call(FunctionCall {
                    name: "println",
                    args: vec![Expr::Literal(42)],
                })),
                Statement::Return {
                    value: Some(Expr::Binary {
                        left: Box::new(Expr::Literal(1)),
                        op: "+",
                        right: Box::new(Expr::Literal(2)),
                    }),
                },
            ],
        },
    };

    let config = TypeConfig { show_types: true };

    println!("Normal - with types:");
    println!("{}", syntax_fmt(&func).state(&config));
    println!();

    println!("Pretty - with types:");
    println!("{}", syntax_fmt(&func).state(&config).pretty());
    println!();

    let config_no_types = TypeConfig { show_types: false };

    println!("Normal - without types:");
    println!("{}", syntax_fmt(&func).state(&config_no_types));
    println!();

    println!("Pretty - without types:");
    println!("{}", syntax_fmt(&func).state(&config_no_types).pretty());
}
