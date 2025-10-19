//! A derive macro based library for flexible syntax tree formatting with pretty printing support.
//!
//! `syntaxfmt` provides a trait based approach to formatting syntax trees with both compact
//! and pretty-printed output modes. It's designed for compiler frontends, code generators,
//! and any application that needs to format structured data as text with optional formatting.
//!
//! # Features
//!
//! - **Derive macro** - for automatic implementation of formatting logic
//! - **Dual formatting modes** - compact and pretty-printed
//! - **Collection support** - automatic formatting for `Vec<T>`, `&[T]`, and `[T; N]` types
//! - **Stateful formatting** - pass user defined context through the formatting process
//! - **Custom formatters** - override default behavior with custom functions
//! - **Flexible attributes** - control delimiters, indentation, and format strings
//!
//! # Cargo Features
//!
//! - **`derive`** - enables `SyntaxFmt` derive macro (on by default)
//!
//! # Quick Start
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, syntax_fmt};
//!
//! #[derive(SyntaxFmt)]
//! struct FunctionCall<'src> {
//!     #[syntax(format = "{content}(")]
//!     name: &'src str,
//!     #[syntax(format = "{content})")]
//!     args: &'src str,
//! }
//!
//! let call = FunctionCall {
//!     name: "println",
//!     args: "\"Hello, world!\"",
//! };
//!
//! assert_eq!(format!("{}", syntax_fmt(&(), &call)), "println(\"Hello, world!\")");
//! ```
//!
//! # Derive Macro Attributes
//!
//! ## Type-level attributes
//!
//! - `#[syntax(delim = ", ")]` - Delimiter between items (default: `","`)
//! - `#[syntax(pretty_delim = ",\n")]` - Delimiter in pretty mode (default: `", "`)
//! - `#[syntax(format = "prefix{content}suffix")]` - Outer format wrapper
//! - `#[syntax(state_bound = "MyTrait")]` - Add trait bound for exposing functionality to custom formatters
//!
//! ## Field-level attributes
//!
//! - `#[syntax(format = "{content}")]` - Format string with `{content}` placeholder
//! - `#[syntax(pretty_format = "{content}")]` - Format string for pretty mode
//! - `#[syntax(content = my_formatter)]` - Custom formatter function
//! - `#[syntax(empty_suffix = ";")]` - Output this instead of normal formatting when field is empty
//! - `#[syntax(indent)]` - Write indentation before this field (pretty mode only)
//! - `#[syntax(indent_inc)]` - Increase indent level for this field's content
//! - `#[syntax(skip)]` - Skip this field during formatting
//!
//! # Examples
//!
//! ## Basic struct formatting
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, syntax_fmt, syntax_fmt_pretty};
//!
//! #[derive(SyntaxFmt)]
//! struct LetStatement<'src> {
//!     #[syntax(format = "let {content} = ")]
//!     name: &'src str,
//!     #[syntax(format = "{content};")]
//!     value: &'src str,
//! }
//!
//! let stmt = LetStatement {
//!     name: "x",
//!     value: "42",
//! };
//!
//! assert_eq!(format!("{}", syntax_fmt(&(), &stmt)), "let x = 42;");
//! ```
//!
//! ## Optional fields
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, syntax_fmt};
//!
//! #[derive(SyntaxFmt)]
//! struct TypeAnnotation<'src> {
//!     name: &'src str,
//!     #[syntax(format = ": {content}")]
//!     type_name: Option<&'src str>,
//! }
//!
//! let with_type = TypeAnnotation {
//!     name: "x",
//!     type_name: Some("i32"),
//! };
//! assert_eq!(format!("{}", syntax_fmt(&(), &with_type)), "x: i32");
//!
//! let without_type = TypeAnnotation {
//!     name: "y",
//!     type_name: None,
//! };
//! assert_eq!(format!("{}", syntax_fmt(&(), &without_type)), "y");
//! ```
//!
//! ## Pretty printing with indentation
//!
//! The `indent_inc` attribute increases the indent level, and `indent` writes indentation
//! before formatting a field in pretty mode.
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, syntax_fmt, syntax_fmt_pretty};
//!
//! #[derive(SyntaxFmt)]
//! struct Statement<'src> {
//!     #[syntax(format = "{content};", indent)]
//!     code: &'src str,
//! }
//!
//! #[derive(SyntaxFmt)]
//! struct Block<'src> {
//!     #[syntax(
//!         format = "{{{content}}}",
//!         pretty_format = "{{\n{content}\n}}",
//!         indent_inc
//!     )]
//!     body: Statement<'src>,
//! }
//!
//! let block = Block { body: Statement { code: "return 42" } };
//!
//! assert_eq!(format!("{}", syntax_fmt(&(), &block)), "{return 42;}");
//! assert_eq!(format!("{}", syntax_fmt_pretty(&(), &block)), "{\n    return 42;\n}");
//! ```
//!
//! ## Using `empty_suffix` for empty collections
//!
//! The `empty_suffix` attribute provides early-out formatting for empty collection fields.
//! When the field's `is_empty()` returns true, only the suffix is output instead of the
//! normal format string. This is useful for syntax like `mod name;` vs `mod name { items }`.
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, syntax_fmt};
//!
//! #[derive(SyntaxFmt)]
//! #[syntax(delim = " ", pretty_delim = " ")]
//! struct Statement<'src>(&'src str);
//!
//! #[derive(SyntaxFmt)]
//! struct Module<'src> {
//!     #[syntax(format = "mod {content}")]
//!     name: &'src str,
//!     #[syntax(format = " {{{content}}}", empty_suffix = ";")]
//!     items: Vec<Statement<'src>>,
//! }
//!
//! let empty = Module {
//!     name: "empty",
//!     items: vec![],
//! };
//! assert_eq!(format!("{}", syntax_fmt(&(), &empty)), "mod empty;");
//!
//! let with_items = Module {
//!     name: "lib",
//!     items: vec![Statement("fn main()")],
//! };
//! assert_eq!(format!("{}", syntax_fmt(&(), &with_items)), "mod lib {fn main()}");
//! ```
//!
//! ## Collection formatting
//!
//! Collections (`Vec<T>`, `&[T]`, `[T; N]`) are automatically formatted by iterating over their
//! elements and using the element type's delimiter configuration.
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, syntax_fmt, syntax_fmt_pretty};
//!
//! #[derive(SyntaxFmt)]
//! #[syntax(delim = "::", pretty_delim = " :: ")]
//! struct Segment<'src>(&'src str);
//!
//! #[derive(SyntaxFmt)]
//! struct Path<'src> {
//!     segments: Vec<Segment<'src>>,
//! }
//!
//! let path = Path {
//!     segments: vec![Segment("std"), Segment("collections"), Segment("HashMap")],
//! };
//!
//! assert_eq!(
//!     format!("{}", syntax_fmt(&(), &path)),
//!     "std::collections::HashMap"
//! );
//! assert_eq!(
//!     format!("{}", syntax_fmt_pretty(&(), &path)),
//!     "std :: collections :: HashMap"
//! );
//! ```
//!
//! ## Custom formatters
//!
//! The `content` attribute allows you to specify a custom formatting function for a field.
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, SyntaxFmtContext, syntax_fmt};
//!
//! fn quote_formatter<State>(value: &str, ctx: &mut SyntaxFmtContext<State>) -> std::fmt::Result {
//!     write!(ctx, "\"{}\"", value)
//! }
//!
//! #[derive(SyntaxFmt)]
//! struct StringLiteral<'src> {
//!     #[syntax(content = quote_formatter)]
//!     value: &'src str,
//! }
//!
//! let lit = StringLiteral { value: "hello" };
//! assert_eq!(format!("{}", syntax_fmt(&(), &lit)), "\"hello\"");
//! ```
//!
//! ## Stateful formatting
//!
//! You can manually implement `SyntaxFmt` with state bounds to access user-provided state
//! like symbol tables or configuration during formatting.
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, SyntaxFmtContext, syntax_fmt};
//!
//! // Define a trait for name resolution
//! trait NameResolver {
//!     fn resolve(&self, id: &str) -> String;
//! }
//!
//! struct SymbolTable;
//! impl NameResolver for SymbolTable {
//!     fn resolve(&self, id: &str) -> String {
//!         format!("resolved_{}", id)
//!     }
//! }
//!
//! // Manually implement SyntaxFmt with a state bound
//! struct Identifier<'src> {
//!     name: &'src str,
//! }
//!
//! impl<'src, State: NameResolver> SyntaxFmt<State> for Identifier<'src> {
//!     fn syntax_fmt(&self, ctx: &mut SyntaxFmtContext<State>) -> std::fmt::Result {
//!         write!(ctx, "{}", ctx.state().resolve(self.name))
//!     }
//! }
//!
//! let symbols = SymbolTable;
//! let ident = Identifier { name: "x" };
//! assert_eq!(format!("{}", syntax_fmt(&symbols, &ident)), "resolved_x");
//! ```

use std::cell::Cell;
use std::fmt::{Arguments, Display, Formatter, Result as FmtResult};

pub use syntaxfmt_macros::SyntaxFmt;

/// Context passed to formatting implementations, containing the formatter and formatting state.
///
/// This type is automatically provided to `SyntaxFmt::syntax_fmt` implementations and should
/// be used for all write operations and state access.
///
/// # Type Parameters
///
/// - `'fmt` - Lifetime of the formatter reference
/// - `'wrt` - Lifetime of the formatting write operation
/// - `State` - User-defined state type for contextual information
///
/// # Examples
///
/// ```
/// use syntaxfmt::SyntaxFmtContext;
///
/// fn custom_formatter<State>(
///     value: &str,
///     ctx: &mut SyntaxFmtContext<State>,
/// ) -> std::fmt::Result {
///     write!(ctx, "custom({})", value)
/// }
/// ```
pub struct SyntaxFmtContext<'fmt, 'wrt, State> {
    f: &'fmt mut Formatter<'wrt>,
    state: Cell<&'fmt State>,
    ind: usize,
    pretty: bool,
}

impl<'fmt, 'wrt, State> SyntaxFmtContext<'fmt, 'wrt, State> {
    #[must_use]
    #[inline]
    fn new(f: &'fmt mut Formatter<'wrt>, state: &'fmt State) -> Self {
        Self {
            f,
            state: Cell::new(state),
            ind: 0,
            pretty: false,
        }
    }

    #[must_use]
    #[inline]
    fn new_pretty(f: &'fmt mut Formatter<'wrt>, state: &'fmt State) -> Self {
        Self {
            f,
            state: Cell::new(state),
            ind: 0,
            pretty: true,
        }
    }

    /// Returns `true` if pretty printing mode is enabled.
    ///
    /// Use this to conditionally emit different formatting based on the mode.
    ///
    /// # Examples
    ///
    /// ```
    /// use syntaxfmt::{SyntaxFmt, SyntaxFmtContext};
    ///
    /// struct MyType;
    ///
    /// impl SyntaxFmt<()> for MyType {
    ///     fn syntax_fmt(&self, ctx: &mut SyntaxFmtContext<()>) -> std::fmt::Result {
    ///         if ctx.is_pretty() {
    ///             write!(ctx, "pretty")
    ///         } else {
    ///             write!(ctx, "compact")
    ///         }
    ///     }
    /// }
    /// ```
    #[must_use]
    #[inline]
    pub fn is_pretty(&self) -> bool {
        self.pretty
    }

    /// Returns a reference to the user-defined state.
    ///
    /// The state can be used to pass contextual information through the formatting process,
    /// such as symbol tables, configuration, or other data needed by custom formatters.
    ///
    /// # Examples
    ///
    /// ```
    /// use syntaxfmt::SyntaxFmtContext;
    ///
    /// struct Config {
    ///     verbose: bool,
    /// }
    ///
    /// fn formatter(value: &str, ctx: &mut SyntaxFmtContext<Config>) -> std::fmt::Result {
    ///     if ctx.state().verbose {
    ///         write!(ctx, "verbose: {}", value)
    ///     } else {
    ///         write!(ctx, "{}", value)
    ///     }
    /// }
    /// ```
    #[must_use]
    #[inline]
    pub fn state(&self) -> &'fmt State {
        self.state.get()
    }

    /// Writes formatted arguments to the output.
    ///
    /// This is typically called via the `write!` macro rather than directly.
    ///
    /// # Examples
    ///
    /// ```
    /// use syntaxfmt::SyntaxFmtContext;
    ///
    /// fn my_formatter<State>(ctx: &mut SyntaxFmtContext<State>) -> std::fmt::Result {
    ///     write!(ctx, "Hello, {}!", "world")
    /// }
    /// ```
    #[inline]
    pub fn write_fmt(&mut self, args: Arguments<'_>) -> FmtResult {
        self.f.write_fmt(args)
    }

    /// Writes a string slice to the output.
    ///
    /// # Examples
    ///
    /// ```
    /// use syntaxfmt::SyntaxFmtContext;
    ///
    /// fn my_formatter<State>(ctx: &mut SyntaxFmtContext<State>) -> std::fmt::Result {
    ///     ctx.write_str("Hello, world!")
    /// }
    /// ```
    #[inline]
    pub fn write_str(&mut self, s: &str) -> FmtResult {
        self.f.write_str(s)
    }

    /// Writes the current indentation to the output.
    ///
    /// The indent string is repeated according to the current indentation level.
    /// This is typically only used in pretty printing mode.
    ///
    /// # Arguments
    ///
    /// * `indent` - The string to repeat for each indentation level (e.g., `"    "` for 4 spaces)
    ///
    /// # Examples
    ///
    /// ```
    /// use syntaxfmt::{SyntaxFmt, SyntaxFmtContext};
    ///
    /// struct Indented;
    ///
    /// impl SyntaxFmt<()> for Indented {
    ///     fn syntax_fmt(&self, ctx: &mut SyntaxFmtContext<()>) -> std::fmt::Result {
    ///         if ctx.is_pretty() {
    ///             ctx.indent("  ")?;
    ///         }
    ///         write!(ctx, "content")
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn indent(&mut self, indent: &str) -> FmtResult {
        write!(self.f, "{}", indent.repeat(self.ind))
    }

    /// Increases the indentation level by one.
    ///
    /// Should be paired with a corresponding `dec_indent()` call.
    /// This is automatically handled by the `indent_inc` attribute.
    ///
    /// # Examples
    ///
    /// ```
    /// use syntaxfmt::SyntaxFmtContext;
    ///
    /// fn nested_formatter<State>(ctx: &mut SyntaxFmtContext<State>) -> std::fmt::Result {
    ///     ctx.inc_indent();
    ///     write!(ctx, "nested")?;
    ///     ctx.dec_indent();
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn inc_indent(&mut self) {
        self.ind += 1;
    }

    /// Decreases the indentation level by one.
    ///
    /// Uses saturating subtraction to prevent underflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use syntaxfmt::SyntaxFmtContext;
    ///
    /// fn nested_formatter<State>(ctx: &mut SyntaxFmtContext<State>) -> std::fmt::Result {
    ///     ctx.inc_indent();
    ///     write!(ctx, "nested")?;
    ///     ctx.dec_indent();
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn dec_indent(&mut self) {
        self.ind = self.ind.saturating_sub(1);
    }
}

/// A wrapper that implements `Display` for types implementing `SyntaxFmt`.
///
/// This type is returned by [`syntax_fmt`] and [`syntax_fmt_pretty`] and should typically
/// not be constructed directly.
pub struct SyntaxDisplay<'state, 'syn, State, T>
where
    'state: 'syn,
    T: SyntaxFmt<State>,
{
    state: &'state State,
    syn: &'syn T,
    pretty: bool,
}

impl<'state, 'syn, State, T> SyntaxDisplay<'state, 'syn, State, T>
where
    'state: 'syn,
    T: SyntaxFmt<State>,
{
    #[must_use]
    #[inline]
    fn new(state: &'state State, syn: &'syn T) -> Self {
        Self {
            state,
            syn,
            pretty: false,
        }
    }

    #[must_use]
    #[inline]
    fn new_pretty(state: &'state State, syn: &'syn T) -> Self {
        Self {
            state,
            syn,
            pretty: true,
        }
    }
}

impl<'state, 'syn, State, T> Display for SyntaxDisplay<'state, 'syn, State, T>
where
    'state: 'syn,
    T: SyntaxFmt<State>,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut ctx = if self.pretty {
            SyntaxFmtContext::new_pretty(f, self.state)
        } else {
            SyntaxFmtContext::new(f, self.state)
        };
        self.syn.syntax_fmt(&mut ctx)
    }
}

/// Formats a syntax tree in compact mode.
///
/// Returns a [`SyntaxDisplay`] wrapper that implements `Display`, allowing it to be
/// used with `format!`, `println!`, and other formatting macros.
///
/// # Arguments
///
/// * `state` - User-defined state to pass through the formatting process
/// * `syn` - The syntax tree to format
///
/// # Examples
///
/// ```
/// use syntaxfmt::{SyntaxFmt, syntax_fmt};
///
/// #[derive(SyntaxFmt)]
/// struct Expr<'src> {
///     value: &'src str,
/// }
///
/// let expr = Expr { value: "42" };
/// let output = format!("{}", syntax_fmt(&(), &expr));
/// assert_eq!(output, "42");
/// ```
#[must_use]
#[inline]
pub fn syntax_fmt<'state, 'syn, State, T>(
    state: &'state State,
    syn: &'syn T,
) -> SyntaxDisplay<'state, 'syn, State, T>
where
    'state: 'syn,
    T: SyntaxFmt<State>,
{
    SyntaxDisplay::new(state, syn)
}

/// Formats a syntax tree in pretty printing mode.
///
/// Returns a [`SyntaxDisplay`] wrapper that implements `Display`. In pretty mode,
/// formatting may include additional whitespace, newlines, and indentation.
///
/// # Arguments
///
/// * `state` - User-defined state to pass through the formatting process
/// * `syn` - The syntax tree to format
///
/// # Examples
///
/// ```
/// use syntaxfmt::{SyntaxFmt, syntax_fmt, syntax_fmt_pretty};
///
/// #[derive(SyntaxFmt)]
/// struct Block<'src> {
///     #[syntax(format = "{{{content}}}", pretty_format = "{{\n{content}\n}}")]
///     body: &'src str,
/// }
///
/// let block = Block { body: "code" };
///
/// assert_eq!(format!("{}", syntax_fmt(&(), &block)), "{code}");
/// assert_eq!(format!("{}", syntax_fmt_pretty(&(), &block)), "{\ncode\n}");
/// ```
#[must_use]
#[inline]
pub fn syntax_fmt_pretty<'state, 'syn, State, T>(
    state: &'state State,
    syn: &'syn T,
) -> SyntaxDisplay<'state, 'syn, State, T>
where
    'state: 'syn,
    T: SyntaxFmt<State>,
{
    SyntaxDisplay::new_pretty(state, syn)
}

/// Trait for types that can be formatted as syntax.
///
/// This trait is typically implemented via the `#[derive(SyntaxFmt)]` macro, but can
/// also be implemented manually for custom formatting logic.
///
/// # Type Parameters
///
/// * `State` - User-defined state type that is passed through the formatting process
///
/// # Associated Constants
///
/// * `DELIM` - Default delimiter between items (default: `","`)
/// * `PRETTY_DELIM` - Delimiter used in pretty mode (default: `", "`)
/// * `INDENT` - Indentation string (default: `"    "`)
///
/// # Examples
///
/// ## Automatic implementation via derive
///
/// ```
/// use syntaxfmt::SyntaxFmt;
///
/// #[derive(SyntaxFmt)]
/// struct Point {
///     #[syntax(format = "x: {content}, ")]
///     x: i32,
///     #[syntax(format = "y: {content}")]
///     y: i32,
/// }
/// ```
///
/// ## Manual implementation
///
/// ```
/// use syntaxfmt::{SyntaxFmt, SyntaxFmtContext};
///
/// struct Custom(String);
///
/// impl SyntaxFmt<()> for Custom {
///     fn syntax_fmt(&self, ctx: &mut SyntaxFmtContext<()>) -> std::fmt::Result {
///         write!(ctx, "Custom({})", self.0)
///     }
/// }
/// ```
pub trait SyntaxFmt<State> {
    /// Default delimiter between items in compact mode.
    const DELIM: &'static str = ",";

    /// Delimiter used in pretty printing mode.
    const PRETTY_DELIM: &'static str = ", ";

    /// Indentation string repeated for each indentation level.
    const INDENT: &'static str = "    ";

    /// Formats this value using the given context.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The formatting context
    ///
    /// # Examples
    ///
    /// ```
    /// use syntaxfmt::{SyntaxFmt, SyntaxFmtContext};
    ///
    /// struct MyType(i32);
    ///
    /// impl SyntaxFmt<()> for MyType {
    ///     fn syntax_fmt(&self, ctx: &mut SyntaxFmtContext<()>) -> std::fmt::Result {
    ///         write!(ctx, "MyType({})", self.0)
    ///     }
    /// }
    /// ```
    fn syntax_fmt(&self, ctx: &mut SyntaxFmtContext<State>) -> FmtResult;
}

/// Blanket implementation for types implementing `Display`.
///
/// Any type that implements `Display` automatically implements `SyntaxFmt<State>` for any `State`.
impl<State, T> SyntaxFmt<State> for T
where
    T: Display,
{
    fn syntax_fmt(&self, ctx: &mut SyntaxFmtContext<State>) -> FmtResult {
        write!(ctx, "{}", *self)
    }
}
