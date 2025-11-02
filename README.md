# syntaxfmt

[![Crates.io](https://img.shields.io/crates/d/syntaxfmt.svg)](https://crates.io/crates/syntaxfmt)
[![MIT License](https://img.shields.io/badge/license-MIT-brightgreen)](LICENSE-MIT)
[![APACHE 2.0 License](https://img.shields.io/badge/license-APACHE%202.0-brightgreen)](LICENSE-APACHE)

A derive macro-based library for flexible syntax tree formatting with pretty printing support.

`syntaxfmt` provides a trait and builder based approach to formatting syntax trees with both compact and pretty-printed output modes. It's designed for compiler frontends, code generators, and any application that needs to format structured data as text with dynamic formatting.

## Features

- **Derive Macro** - Automatic implementation via `#[derive(SyntaxFmt)]`
- **Flexible Decorations** - Add prefixes, suffixes, and collection separators
- **Modal Formatting** - Customise formatting output for different modes, normal and pretty
- **Intuitive Layout Options** - Semi-automated layout control with newlines and indentation
- **Content Replacement** - Override field formatting with literals or custom functions
- **Conditional Formatting** - Format based on arbitrary boolean expressions, with else support
- **Stateful Formatting** - Pass mutable or immutable state for context-aware output

## Cargo Features

- **`derive`** (enabled by default) - Enables the `SyntaxFmt` derive macro

## Getting Started

Add `syntaxfmt` to your `Cargo.toml`:

```toml
[dependencies]
syntaxfmt = "0.2.3"
```

### Quick Start

The simplest use case is to derive `SyntaxFmt` on your types and they'll format themselves by printing each field in order:

```rust
use syntaxfmt::{SyntaxFmt, syntax_fmt};

#[derive(SyntaxFmt)]
struct BinaryOp<'src> {
    left: &'src str,
    op: &'src str,
    right: &'src str,
}

let expr = BinaryOp { left: "x", op: "+", right: "y" };
assert_eq!(format!("{}", syntax_fmt(&expr)), "x+y");
```

### Adding Decorations

Use `pre` (prefix) and `suf` (suffix) attributes to add syntax around fields:

```rust
use syntaxfmt::{SyntaxFmt, syntax_fmt};

#[derive(SyntaxFmt)]
#[syntax(pre = "let ", suf = ";")]
struct LetStatement<'src> {
    name: &'src str,

    #[syntax(pre = " = ")]
    value: &'src str,
}

let stmt = LetStatement { name: "x", value: "42" };
assert_eq!(format!("{}", syntax_fmt(&stmt)), "let x = 42;");
```

### Basic Pretty Printing

Enable pretty printing with the `.pretty()` method. Use modal attributes (arrays) to specify different formatting for normal vs pretty mode:

```rust
use syntaxfmt::{SyntaxFmt, syntax_fmt};

#[derive(SyntaxFmt)]
struct FunctionCall<'src> {
    name: &'src str,

    #[syntax(pre = ["(", "( "], suf = [")", " )"], sep = [", ", ",  "])]
    args: Vec<&'src str>,
}

let call = FunctionCall {
    name: "max",
    args: vec!["x", "y", "z"],
};

assert_eq!(format!("{}", syntax_fmt(&call)), "max(x, y, z)");
assert_eq!(format!("{}", syntax_fmt(&call).pretty()), "max( x,  y,  z )");
```

More advanced pretty printing is available via [Indentation and Layout](https://docs.rs/syntaxfmt/latest/syntaxfmt/#indentation-and-layout).

### Further Reading and More Examples

For complete documentation including many more examples, visit [docs.rs/syntaxfmt](https://docs.rs/syntaxfmt).

## Attribute Argument Summary

Each attribute argument may be applied at the type, field, or `syntax_else` level.
Check the table below to see valid locations for each argument.

| Argument | Description | Valid Location |
|----------|-------------|----------------|
| `pre` | Text before content | field/type/else |
| `suf` | Text after content | field/type/else |
| `sep` | Separator between collection elements | field/type/else |
| `cont` | Literal replacement for field value | field/type/else |
| `cont_with` | Custom formatter function/closure | field/type/else |
| `eval` | Conditional expression | field/type |
| `eval_with` | Conditional function/closure | field/type |
| `nl` | Newline positions (see table below) | field/type/else |
| `ind` | Increase indent level for field content | field/type/else |
| `skip` | Omit field from formatting | field/type |
| `state` | Specify state type (type-level only) | type |
| `bound` | Add trait bound to state (type-level only) | type |

### Newline (`nl`) Positions

| Position | Description |
|----------|-------------|
| `beg` | Beginning (before prefix) |
| `pre` | After prefix |
| `sep` | After separator |
| `cont` | After content |
| `suf` | After suffix |
| `inner` | Shorthand for `[pre, sep, cont]` - useful for block inner content |
| `outer` | Shorthand for `[beg, suf]` - useful for padding |

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is dual licensed under:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

Unless explicitly stated otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
