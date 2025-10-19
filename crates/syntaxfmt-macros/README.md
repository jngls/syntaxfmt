# syntaxfmt-macros

Procedural macros for the [syntaxfmt](https://crates.io/crates/syntaxfmt) crate.

This crate provides the `#[derive(SyntaxFmt)]` macro that automatically implements the `SyntaxFmt` trait for your types.

## Usage

You should not use this crate directly. Instead, use the `syntaxfmt` crate which re-exports this functionality:

```rust
use syntaxfmt::SyntaxFmt;

#[derive(SyntaxFmt)]
struct MyType {
    field: String,
}
```

For complete documentation and examples, see the [syntaxfmt documentation](https://docs.rs/syntaxfmt).

## License

This project is dual licensed under MIT OR Apache-2.0.
