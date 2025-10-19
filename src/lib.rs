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
//!     #[syntax(format = "{content})", pretty_format = " {content} )")]
//!     args: &'src str,
//! }
//!
//! let call = FunctionCall {
//!     name: "println",
//!     args: "\"Hello, world!\"",
//! };
//!
//! // Compact formatting
//! assert_eq!(format!("{}", syntax_fmt(&(), &call)), "println(\"Hello, world!\")");
//!
//! // Pretty formatting with .pretty()
//! assert_eq!(format!("{}", syntax_fmt(&(), &call).pretty()), "println( \"Hello, world!\" )");
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
//! use syntaxfmt::{SyntaxFmt, syntax_fmt};
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
//! use syntaxfmt::{SyntaxFmt, syntax_fmt};
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
//! assert_eq!(format!("{}", syntax_fmt(&(), &block).pretty()), "{\n    return 42;\n}");
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
//! use syntaxfmt::{SyntaxFmt, syntax_fmt};
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
//!     format!("{}", syntax_fmt(&(), &path).pretty()),
//!     "std :: collections :: HashMap"
//! );
//! ```
//!
//! ## Custom formatters
//!
//! The `content` attribute allows you to specify a custom formatting function for a field.
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, SyntaxFormatter, syntax_fmt};
//!
//! fn quote_formatter<S>(value: &str, ctx: &mut SyntaxFormatter<S>) -> std::fmt::Result {
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
//! ## Stateful formatting with mutable state
//!
//! You can manually implement `SyntaxFmt` to access and modify user-provided state
//! during formatting. This is useful for tracking counters, generating unique IDs,
//! or updating symbol tables.
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, SyntaxFormatter, syntax_fmt_mut};
//!
//! // State that tracks variable assignments
//! struct VarTracker {
//!     next_id: usize,
//! }
//!
//! impl VarTracker {
//!     fn allocate(&mut self) -> usize {
//!         let id = self.next_id;
//!         self.next_id += 1;
//!         id
//!     }
//! }
//!
//! // A variable declaration that gets a unique ID
//! struct VarDecl<'src> {
//!     name: &'src str,
//! }
//!
//! impl<'src> SyntaxFmt<VarTracker> for VarDecl<'src> {
//!     fn syntax_fmt(&self, ctx: &mut SyntaxFormatter<VarTracker>) -> std::fmt::Result {
//!         let id = ctx.state_mut().allocate();
//!         write!(ctx, "let {}_{} = ", self.name, id)
//!     }
//! }
//!
//! let mut tracker = VarTracker { next_id: 0 };
//! let decl = VarDecl { name: "x" };
//! assert_eq!(format!("{}", syntax_fmt_mut(&mut tracker, &decl)), "let x_0 = ");
//! assert_eq!(tracker.next_id, 1);
//! ```

use core::panic;
use std::cell::{Ref, RefCell, RefMut};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Deref, DerefMut};

pub use syntaxfmt_macros::SyntaxFmt;

// Holds state reference
enum StateRef<'s, S> {
    Immutable(&'s S),
    Mutable(&'s mut S),
}

impl<'s, S> StateRef<'s, S> {
    #[must_use]
    #[inline]
    fn new_ref(r: &'s S) -> Self {
        StateRef::Immutable(r)
    }

    #[must_use]
    #[inline]
    fn new_mut(r: &'s mut S) -> Self {
        StateRef::Mutable(r)
    }

    #[must_use]
    #[inline]
    fn as_ref(&self) -> &S {
        match self {
            StateRef::Immutable(r) => r,
            StateRef::Mutable(r) => r,
        }
    }

    #[must_use]
    #[inline]
    #[track_caller]
    fn as_mut(&mut self) -> &mut S {
        match self {
            StateRef::Immutable(_) => panic!("StateRef: state is immutable"),
            StateRef::Mutable(r) => r,
        }
    }
}

/// Context passed to formatting implementations, containing the formatter and formatting state.
pub struct SyntaxFormatter<'sr, 's, 'f, 'w, S> {
    f: &'f mut Formatter<'w>,
    state: &'sr RefCell<StateRef<'s, S>>,
    ind: usize,
    pretty: bool,
    indent: &'static str,
}

impl<'sr, 's, 'f, 'w, S> SyntaxFormatter<'sr, 's, 'f, 'w, S> {
    #[must_use]
    #[inline]
    fn new(f: &'f mut Formatter<'w>, state: &'sr RefCell<StateRef<'s, S>>, indent: &'static str) -> Self {
        Self {
            f,
            state,
            ind: 0,
            pretty: false,
            indent,
        }
    }

    #[must_use]
    #[inline]
    fn new_pretty(f: &'f mut Formatter<'w>, state: &'sr RefCell<StateRef<'s, S>>, indent: &'static str) -> Self {
        Self {
            f,
            state,
            ind: 0,
            pretty: true,
            indent,
        }
    }

    /// Returns `true` if pretty printing mode is enabled.
    #[must_use]
    #[inline]
    pub fn is_pretty(&self) -> bool {
        self.pretty
    }

    /// Returns a reference to the user-defined state.
    #[must_use]
    #[inline]
    pub fn state(&self) -> Ref<S> {
        Ref::map(self.state.borrow(), |s| s.as_ref())
    }

    /// Returns a mutable reference to the user-defined state. Panics if state is immutable.
    #[must_use]
    #[inline]
    #[track_caller]
    pub fn state_mut<'a>(&'a mut self) -> RefMut<'a, S> {
        RefMut::map(self.state.borrow_mut(), |s| s.as_mut())
    }

    /// Writes the current indentation to the output.
    #[inline]
    pub fn indent(&mut self) -> FmtResult {
        write!(self.f, "{}", self.indent.repeat(self.ind))
    }

    /// Increases the indentation level by one.
    #[inline]
    pub fn inc_indent(&mut self) {
        self.ind += 1;
    }

    /// Decreases the indentation level by one.
    #[inline]
    pub fn dec_indent(&mut self) {
        self.ind = self.ind.saturating_sub(1);
    }
}

impl<'sr, 's, 'f, 'w, S> Deref for SyntaxFormatter<'sr, 's, 'f, 'w, S> {
    type Target = Formatter<'w>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.f
    }
}

impl<'sr, 's, 'f, 'w, S> DerefMut for SyntaxFormatter<'sr, 's, 'f, 'w, S> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.f
    }
}

/// A wrapper that implements `Display` for types implementing `SyntaxFmt`.
pub struct SyntaxDisplay<'s, 'e, S, E>
where
    E: SyntaxFmt<S>,
{
    state: RefCell<StateRef<'s, S>>,
    elem: &'e E,
    pretty: bool,
    indent: &'static str,
}

impl<'s, 'e, S, E> SyntaxDisplay<'s, 'e, S, E>
where
    E: SyntaxFmt<S>,
{
    #[must_use]
    #[inline]
    fn new(state: &'s S, elem: &'e E) -> Self {
        Self {
            state: RefCell::new(StateRef::new_ref(state)),
            elem,
            pretty: false,
            indent: "    ",
        }
    }

    #[must_use]
    #[inline]
    fn new_mut(state: &'s mut S, elem: &'e E) -> Self {
        Self {
            state: RefCell::new(StateRef::new_mut(state)),
            elem,
            pretty: false,
            indent: "    ",
        }
    }

    /// Enable pretty printing mode.
    #[must_use]
    #[inline]
    pub fn pretty(mut self) -> Self {
        self.pretty = true;
        self
    }

    /// Set the indentation string (default is four spaces).
    #[must_use]
    #[inline]
    pub fn indent(mut self, indent: &'static str) -> Self {
        self.indent = indent;
        self
    }
}

impl<'s, 'e, S, E> Display for SyntaxDisplay<'s, 'e, S, E>
where
    E: SyntaxFmt<S>,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut ctx = if self.pretty {
            SyntaxFormatter::new_pretty(f, &self.state, self.indent)
        } else {
            SyntaxFormatter::new(f, &self.state, self.indent)
        };
        self.elem.syntax_fmt(&mut ctx)
    }
}

/// Formats a syntax tree.
///
/// Returns a [`SyntaxDisplay`] wrapper that implements `Display`, allowing it to be
/// used with `format!`, `println!`, and other formatting macros.
///
/// Chain with `.pretty()` to enable pretty printing mode.
///
/// # Arguments
///
/// * `state` - User-defined state to pass through the formatting process
/// * `e` - The syntax tree to format
///
/// # Examples
///
/// ```
/// use syntaxfmt::{SyntaxFmt, syntax_fmt};
///
/// #[derive(SyntaxFmt)]
/// struct Expr<'src> {
///     #[syntax(format = "({content})", pretty_format = "( {content} )")]
///     value: &'src str,
/// }
///
/// let expr = Expr { value: "42" };
/// assert_eq!(format!("{}", syntax_fmt(&(), &expr)), "(42)");
/// assert_eq!(format!("{}", syntax_fmt(&(), &expr).pretty()), "( 42 )");
/// ```
#[must_use]
#[inline]
pub fn syntax_fmt<'s, 'e, S, E>(
    state: &'s S,
    e: &'e E,
) -> SyntaxDisplay<'s, 'e, S, E>
where
    E: SyntaxFmt<S>,
{
    SyntaxDisplay::new(state, e)
}

/// Formats a syntax tree with mutable state.
///
/// Returns a [`SyntaxDisplay`] wrapper that implements `Display`. This variant allows
/// the state to be modified during formatting, useful for tracking generated identifiers,
/// maintaining counters, or updating symbol tables.
///
/// Chain with `.pretty()` to enable pretty printing mode.
///
/// # Arguments
///
/// * `state` - Mutable user-defined state to pass through the formatting process
/// * `e` - The syntax tree to format
///
/// # Examples
///
/// ```
/// use syntaxfmt::{SyntaxFmt, SyntaxFormatter, syntax_fmt_mut};
///
/// struct Counter {
///     count: usize,
/// }
///
/// struct Item;
///
/// impl SyntaxFmt<Counter> for Item {
///     fn syntax_fmt(&self, ctx: &mut SyntaxFormatter<Counter>) -> std::fmt::Result {
///         let count = ctx.state_mut().count;
///         ctx.state_mut().count += 1;
///         write!(ctx, "item_{}", count)
///     }
/// }
///
/// let mut state = Counter { count: 0 };
/// let item = Item;
/// assert_eq!(format!("{}", syntax_fmt_mut(&mut state, &item)), "item_0");
/// assert_eq!(state.count, 1);
///
/// // Pretty printing with mutable state
/// assert_eq!(format!("{}", syntax_fmt_mut(&mut state, &item).pretty()), "item_1");
/// assert_eq!(state.count, 2);
/// ```
#[must_use]
#[inline]
pub fn syntax_fmt_mut<'s, 'e, S, E>(
    state: &'s mut S,
    e: &'e E,
) -> SyntaxDisplay<'s, 'e, S, E>
where
    E: SyntaxFmt<S>,
{
    SyntaxDisplay::new_mut(state, e)
}

/// Trait for types that can be formatted as syntax.
pub trait SyntaxFmt<S> {
    /// Default delimiter between items in compact mode.
    const DELIM: &'static str = ",";

    /// Delimiter used in pretty printing mode.
    const PRETTY_DELIM: &'static str = ", ";

    /// Formats this value using the given context.
    fn syntax_fmt(&self, ctx: &mut SyntaxFormatter<S>) -> FmtResult;
}

/// Blanket implementation for types implementing `Display`.
impl<S, E> SyntaxFmt<S> for E
where
    E: Display,
{
    fn syntax_fmt(&self, ctx: &mut SyntaxFormatter<S>) -> FmtResult {
        write!(ctx, "{}", *self)
    }
}
