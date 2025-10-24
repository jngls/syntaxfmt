# syntaxfmt

[![Crates.io](https://img.shields.io/crates/d/syntaxfmt.svg)](https://crates.io/crates/syntaxfmt)
[![MIT License](https://img.shields.io/badge/license-MIT-brightgreen)](LICENSE-MIT)
[![APACHE 2.0 License](https://img.shields.io/badge/license-APACHE%202.0-brightgreen)](LICENSE-APACHE)

A derive macro-based library for flexible syntax tree formatting with pretty printing support.

`syntaxfmt` provides a trait-based approach to formatting syntax trees with both compact and pretty-printed output modes. It's designed for compiler frontends, code generators, and any application that needs to format structured data as text with optional formatting.

# WARNING
Please be aware, this an alpha stage project. We have a bug https://github.com/jngls/syntaxfmt/issues/2 which is causing massive RAM usage in rustc. This is due to a blanket implementation choking rust-analyzer. I'm aware of it and actively fixing it in https://github.com/jngls/syntaxfmt/tree/fix-blanket-impl.

## Features

- **Derive macro** - Automatic implementation of formatting logic with `#[derive(SyntaxFmt)]`
- **Dual formatting modes** - Compact and pretty-printed output with `.pretty()` method chaining
- **Optional state** - No boilerplate for stateless formatting, easy state passing when needed
- **Collection support** - Automatic formatting for `Vec<T>`, `&[T]`, and `[T; N]` types
- **Boolean and Option support** - Conditional formatting for `bool` and `Option<T>` types
- **Stateful formatting** - Pass user-defined context through the formatting process
- **Custom formatters** - Override default behavior with custom functions or by explicitly implementing `SyntaxFmt`
- **Flexible attributes** - Control delimiters, indentation, and format strings per-field and per-type
- **Customizable indentation** - Use spaces, tabs, or any custom string

## Cargo Features

- **`derive`** (enabled by default) - Enables the `SyntaxFmt` derive macro

## Getting Started

Add `syntaxfmt` to your `Cargo.toml`:

```toml
[dependencies]
syntaxfmt = "0.1.0"
```

### Basic Example

```rust
use syntaxfmt::{SyntaxFmt, syntax_fmt};

#[derive(SyntaxFmt)]
struct FunctionCall<'src> {
    name: &'src str,
    #[syntax(format = "({content})", pretty_format = "( {content} )")]
    args: &'src str,
}

let call = FunctionCall {
    name: "println",
    args: "\"Hello, world!\"",
};

// Compact formatting
assert_eq!(format!("{}", syntax_fmt(&call)), "println(\"Hello, world!\")");

// Pretty formatting
assert_eq!(format!("{}", syntax_fmt(&call).pretty()), "println( \"Hello, world!\" )");
```

### With Custom State

```rust
use syntaxfmt::{SyntaxFmt, SyntaxFormatter, syntax_fmt};

struct VarTracker {
    next_id: usize,
}

struct VarDecl<'src> {
    name: &'src str,
}

impl<'src> SyntaxFmt<VarTracker> for VarDecl<'src> {
    fn syntax_fmt(&self, ctx: &mut SyntaxFormatter<VarTracker>) -> std::fmt::Result {
        let id = ctx.state_mut().next_id;
        ctx.state_mut().next_id += 1;
        write!(ctx, "let {}_{} = ", self.name, id)
    }
}

let mut tracker = VarTracker { next_id: 0 };
let decl = VarDecl { name: "x" };
assert_eq!(format!("{}", syntax_fmt(&decl).state_mut(&mut tracker)), "let x_0 = ");
```

For complete documentation, visit [docs.rs/syntaxfmt](https://docs.rs/syntaxfmt).

## Derive Macro Attributes

### Type-level attributes

- `#[syntax(delim = ", ")]` - Delimiter between items of this type, used by Vec and slice implementations (default: `","`)
- `#[syntax(pretty_delim = ",\n")]` - Delimiter in pretty mode (default: `", "`)
- `#[syntax(format = "prefix{content}suffix")]` - For prefixes and suffixes around the whole type (default: `"{content}"`)
- `#[syntax(pretty_format = "prefix{content}suffix")]` - For pretty prefixes and suffixes around the whole type (default: `"{content}"`)
- `#[syntax(state_bound = "MyTrait")]` - Add trait bound for exposing functionality to custom formatter functions

### Field-level attributes

- `#[syntax(format = "prefix{content}suffix")]` - For prefixes and suffixes around the field (default: `"{content}"`)
- `#[syntax(pretty_format = "prefix{content}suffix")]` - For pretty prefixes and suffixes around the field (default: `"{content}"`)
- `#[syntax(content = my_formatter)]` - Custom content formatter function
- `#[syntax(empty_suffix = ";")]` - Early out with this string when field is empty (for types which implement `is_empty()` function)
- `#[syntax(indent)]` - Write indentation before this field (pretty mode only)
- `#[syntax(indent_region)]` - Increase indent level for this field's content
- `#[syntax(skip)]` - Skip this field during formatting

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is dual licensed under:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE)
- MIT license ([LICENSE-MIT](LICENSE-MIT)

Unless explicitly stated otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
