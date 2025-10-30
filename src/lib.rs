//! A derive macro based library for flexible syntax tree formatting with pretty printing support.
//!
//! `syntaxfmt` provides a trait and builder based approach to formatting syntax trees with both
//! compact and pretty-printed output modes. It's designed for compiler frontends, code generators,
//! and any application that needs to format structured data as text with dynamic formatting.
//!
//! # Features
//!
//! - **Derive Macro** - Automatic implementation via `#[derive(SyntaxFmt)]`
//! - **Flexible Decorations** - Add prefixes, suffixes, and collection delimiters
//! - **Modal Formatting** - Customise formatting output for different modes, normal and pretty
//! - **Automatic Layout** - Automated layout control with newlines and indentation
//! - **Content Replacement** - Override field formatting with literals or custom functions
//! - **Conditional Formatting** - Format based on arbitrary boolean expressions, with else support
//! - **Stateful Formatting** - Pass mutable or immutable state for context-aware output
//!
//! # Cargo Features
//!
//! - **`derive`** - enables `SyntaxFmt` derive macro (on by default)
//!
//! # Quick Start
//!
//! The simplest use case is to derive `SyntaxFmt` on your types and they'll format
//! themselves by printing each field in order.
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, syntax_fmt};
//!
//! #[derive(SyntaxFmt)]
//! struct BinaryOp<'src> {
//!     left: &'src str,
//!     op: &'src str,
//!     right: &'src str,
//! }
//!
//! let expr = BinaryOp { left: "x", op: "+", right: "y" };
//! assert_eq!(format!("{}", syntax_fmt(&expr)), "x+y");
//! ```
//!
//! # Adding Decorations
//!
//! Use `pre` (prefix) and `suf` (suffix) attributes to add syntax around fields.
//! 
//! The `pre` and `suf` attribute arguments can be applied at field, type, or
//! `syntax_else` level.
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, syntax_fmt};
//!
//! #[derive(SyntaxFmt)]
//! #[syntax(pre = "let ", suf = ";")]
//! struct LetStatement<'src> {
//!     name: &'src str,
//!
//!     #[syntax(pre = " = ")]
//!     value: &'src str,
//! }
//!
//! let stmt = LetStatement { name: "x", value: "42" };
//! assert_eq!(format!("{}", syntax_fmt(&stmt)), "let x = 42;");
//! ```
//!
//! # Collections and Delimiters
//!
//! Collections (`Vec<T>`, `&[T]`, `[T; N]`) are formatted automatically with customizable
//! delimiters. The default delimiter is `,` for normal mode and `, ` for pretty mode.
//! 
//! The `delim` attribute argument can be applied at field, type, or `syntax_else` level.
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, syntax_fmt};
//!
//! #[derive(SyntaxFmt)]
//! struct Ident<'src>(&'src str);
//!
//! #[derive(SyntaxFmt)]
//! struct Path<'src> {
//!     #[syntax(delim = "::")]
//!     segments: Vec<Ident<'src>>,
//! }
//!
//! let path = Path {
//!     segments: vec![Ident("std"), Ident("collections"), Ident("HashMap")],
//! };
//! assert_eq!(format!("{}", syntax_fmt(&path)), "std::collections::HashMap");
//! ```
//!
//! # Skip Types and Fields
//!
//! Use `skip` to exclude fields or entire types from output, useful for metadata or internal state.
//! 
//! The `skip` attribute argument can be applied at field or type level.
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, syntax_fmt};
//!
//! #[derive(SyntaxFmt)]
//! struct Node<'src> {
//!     value: &'src str,
//!
//!     #[syntax(skip)]
//!     metadata: u32,
//! }
//!
//! let node = Node { value: "data", metadata: 42 };
//! assert_eq!(format!("{}", syntax_fmt(&node)), "data");
//! ```
//!
//! # Pretty Mode
//!
//! Enable pretty printing with the `.pretty()` builder method. Use modal attributes
//! (arrays with two values) to specify different formatting for normal vs pretty mode.
//! Most attributes support modal values and can be applied at field, type, or
//! `syntax_else` level. Pretty mode also influences newlines and indentation, which
//! we'll cover next.
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, syntax_fmt};
//!
//! #[derive(SyntaxFmt)]
//! struct FunctionCall<'src> {
//!     name: &'src str,
//!
//!     #[syntax(pre = ["(", "( "], suf = [")", " )"], delim = [", ", ",  "])]
//!     args: Vec<&'src str>,
//! }
//!
//! let call = FunctionCall {
//!     name: "max",
//!     args: vec!["x", "y", "z"],
//! };
//!
//! assert_eq!(format!("{}", syntax_fmt(&call)), "max(x, y, z)");
//! assert_eq!(format!("{}", syntax_fmt(&call).pretty()), "max( x,  y,  z )");
//! ```
//!
//! # Indentation and Layout
//!
//! Use `ind` (indent) to increase the indentation level for a field's content. Use `nl` to
//! control newline positions:
//! 
//! - `beg` - beginning
//! - `pre` - after prefix
//! - `cont` - after content
//! - `suf` - after suffix
//! 
//! Newlines default to `""` (normal) and `"\n"`` (pretty), and you can alter them with the
//! `.newline(["", "\r\n"])` builder method.
//! 
//! Indentation segments default to `""` (normal) and `"    "` (pretty), and you can
//! alter them with the `.indent(["", "\t"])` builder method.
//!
//! The `ind` and `nl` attribute arguments can be applied at field, type, or
//! `syntax_else` level.
//! 
//! ```
//! use syntaxfmt::{SyntaxFmt, syntax_fmt};
//!
//! #[derive(SyntaxFmt)]
//! struct Statement<'src> {
//!     #[syntax(suf = ";")]
//!     code: &'src str,
//! }
//!
//! #[derive(SyntaxFmt)]
//! struct Block<'src> {
//!     #[syntax(pre = "{", suf = "}", nl = [pre, cont], ind, delim = "")]
//!     statements: Vec<Statement<'src>>,
//! }
//!
//! let block = Block {
//!     statements: vec![Statement { code: "return 42" }],
//! };
//!
//! // Normal mode - no newlines or indentation
//! assert_eq!(format!("{}", syntax_fmt(&block)), "{return 42;}");
//!
//! // Pretty mode - default newlines ("\n") and indentation ("    ")
//! assert_eq!(format!("{}", syntax_fmt(&block).pretty()), "{\n    return 42;\n}");
//!
//! // Customize indentation to tabs
//! assert_eq!(
//!     format!("{}", syntax_fmt(&block).pretty().indent(["", "\t"])),
//!     "{\n\treturn 42;\n}"
//! );
//!
//! // Customize newlines (e.g., Windows line endings)
//! assert_eq!(
//!     format!("{}", syntax_fmt(&block).pretty().newline(["", "\r\n"])),
//!     "{\r\n    return 42;\r\n}"
//! );
//! ```
//!
//! # Content Replacement
//!
//! Replace a field's value with literal text (or any value that implements
//! SyntaxFmt) using `cont`.
//! 
//! The `cont` attribute argument can be applied at field, type, or `syntax_else` level.
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, syntax_fmt};
//!
//! #[derive(SyntaxFmt)]
//! struct Decl {
//!     #[syntax(cont = "pub ", eval = *is_pub)]
//!     is_pub: bool,
//!     kw: &'static str,
//! }
//!
//! let decl = Decl { is_pub: false, kw: "mod" };
//! assert_eq!(format!("{}", syntax_fmt(&decl)), "mod");
//!
//! let decl = Decl { is_pub: true, kw: "fn" };
//! assert_eq!(format!("{}", syntax_fmt(&decl)), "pub fn");
//! ```
//!
//! Use `cont_with` with a function for custom formatting logic.
//! 
//! The `cont_with` attribute argument can be applied at field, type, or
//! `syntax_else` level.
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, SyntaxFormatter, syntax_fmt};
//!
//! fn quote_formatter<S>(value: &str, f: &mut SyntaxFormatter<S>) -> std::fmt::Result {
//!     write!(f, "\"{}\"", value)
//! }
//!
//! #[derive(SyntaxFmt)]
//! struct StringLiteral<'src> {
//!     #[syntax(cont_with = quote_formatter)]
//!     value: &'src str,
//! }
//!
//! let lit = StringLiteral { value: "hello" };
//! assert_eq!(format!("{}", syntax_fmt(&lit)), "\"hello\"");
//! ```
//!
//! Or use `cont_with` with a closure inline.
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, SyntaxFormatter, syntax_fmt};
//!
//! #[derive(SyntaxFmt)]
//! struct Hex {
//!     #[syntax(cont_with = |v: &u32, f: &mut SyntaxFormatter<_>| write!(f, "0x{:x}", v))]
//!     value: u32,
//! }
//!
//! let hex = Hex { value: 255 };
//! assert_eq!(format!("{}", syntax_fmt(&hex)), "0xff");
//! ```
//!
//! # Conditional Formatting
//!
//! For conditional logic with arbitrary expressions, use `eval`.
//! 
//! The `eval` attribute argument can be applied at field or type level.
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, syntax_fmt};
//!
//! #[derive(SyntaxFmt)]
//! #[syntax(pre = "const ", suf = ";")]
//! struct ConstDecl<'src> {
//!     name: &'src str,
//!
//!     #[syntax(pre = " = ", eval = *value > 100)]
//!     value: u32,
//! }
//!
//! let small = ConstDecl { name: "X", value: 50 };
//! assert_eq!(format!("{}", syntax_fmt(&small)), "const X;");
//!
//! let large = ConstDecl { name: "Y", value: 200 };
//! assert_eq!(format!("{}", syntax_fmt(&large)), "const Y = 200;");
//! ```
//!
//! Use `eval_with` with a function for reusable conditional logic.
//! 
//! The `eval_with` attribute argument can be applied at field or type level.
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, syntax_fmt};
//!
//! fn is_long(s: &str) -> bool {
//!     s.len() > 5
//! }
//!
//! #[derive(SyntaxFmt)]
//! struct Comment<'src> {
//!     #[syntax(pre = "// ", eval_with = is_long)]
//!     text: &'src str,
//! }
//!
//! let short = Comment { text: "ok" };
//! assert_eq!(format!("{}", syntax_fmt(&short)), "");
//!
//! let long = Comment { text: "this is long" };
//! assert_eq!(format!("{}", syntax_fmt(&long)), "// this is long");
//! ```
//!
//! Or use `eval_with` with a closure inline.
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, syntax_fmt};
//!
//! #[derive(SyntaxFmt)]
//! struct Tagged<'src> {
//!     #[syntax(pre = "#[", suf = "]", eval_with = |s: &str| s.starts_with("derive"))]
//!     tag: &'src str,
//! }
//!
//! let other = Tagged { tag: "allow(dead_code)" };
//! assert_eq!(format!("{}", syntax_fmt(&other)), "");
//!
//! let derive = Tagged { tag: "derive(Debug)" };
//! assert_eq!(format!("{}", syntax_fmt(&derive)), "#[derive(Debug)]");
//! ```
//!
//! `Option<T>` fields are only formatted when `Some`, and skip formatting when `None`.
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, syntax_fmt};
//!
//! #[derive(SyntaxFmt)]
//! #[syntax(pre = "return ")]
//! struct Return<'src> {
//!     value: Option<&'src str>,
//! }
//!
//! let with_value = Return { value: Some("42") };
//! assert_eq!(format!("{}", syntax_fmt(&with_value)), "return 42");
//!
//! let without = Return { value: None };
//! assert_eq!(format!("{}", syntax_fmt(&without)), "return ");
//! ```
//!
//! When `Option<T>` fields have decorations, use `eval` or `eval_with` to
//! prevent decorations from appearing when `None`.
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, syntax_fmt};
//!
//! #[derive(SyntaxFmt)]
//! #[syntax(pre = "let ", suf = ";")]
//! struct VarDecl<'src> {
//!     name: &'src str,
//!
//!     #[syntax(pre = ": ", eval = ty.is_some())]
//!     ty: Option<&'src str>,
//!
//!     #[syntax(pre = " = ", eval = value.is_some())]
//!     value: Option<&'src str>,
//! }
//!
//! let untyped = VarDecl { name: "x", ty: None, value: Some("42") };
//! assert_eq!(format!("{}", syntax_fmt(&untyped)), "let x = 42;");
//!
//! let typed = VarDecl { name: "y", ty: Some("i32"), value: None };
//! assert_eq!(format!("{}", syntax_fmt(&typed)), "let y: i32;");
//! ```
//!
//! # Fallback Formatting
//!
//! An additional attribute `#[syntax_else]`, which enables alternate
//! formatting when an `eval` or `eval_with` attribute argument is specified
//! and its result is `false`.
//! 
//! Fallback formatting has a restricted set of accepted attribute arguments:
//! `pre`, `suf`, `delim`, `cont`, `ind`, and `nl`.
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, syntax_fmt};
//!
//! #[derive(SyntaxFmt)]
//! struct MaybeValue {
//!     #[syntax(eval = value.is_some())]
//!     #[syntax_else(cont = "none")]
//!     value: Option<i32>,
//! }
//!
//! let some = MaybeValue { value: Some(42) };
//! assert_eq!(format!("{}", syntax_fmt(&some)), "42");
//!
//! let none = MaybeValue { value: None };
//! assert_eq!(format!("{}", syntax_fmt(&none)), "none");
//! ```
//!
//! # Stateful Formatting
//!
//! Pass mutable or immutable state through formatting to enable context-aware output like
//! symbol resolution, ID generation, or tracking. Use the `state` or `bound` attribute with
//! the derive macro to specify the state type or trait bound. Both `state` and `bound` can
//! only be applied at type level.
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, SyntaxFormatter, syntax_fmt};
//!
//! trait SymbolResolver {
//!     fn resolve(&self, name: &str) -> String;
//! }
//!
//! struct MyResolver;
//! impl SymbolResolver for MyResolver {
//!     fn resolve(&self, name: &str) -> String {
//!         format!("resolved_{}", name)
//!     }
//! }
//!
//! fn resolve_name<S: SymbolResolver>(name: &str, f: &mut SyntaxFormatter<S>) -> std::fmt::Result {
//!     let resolved = f.state().resolve(name);
//!     write!(f, "{}", resolved)
//! }
//!
//! #[derive(SyntaxFmt)]
//! #[syntax(bound = SymbolResolver)]
//! struct Identifier<'src> {
//!     #[syntax(cont_with = resolve_name)]
//!     name: &'src str,
//! }
//!
//! let resolver = MyResolver;
//! let ident = Identifier { name: "foo" };
//! assert_eq!(format!("{}", syntax_fmt(&ident).state(&resolver)), "resolved_foo");
//! ```
//!
//! ## Additional State Examples
//!
//! - [`SyntaxFormatter::state`]
//! - [`SyntaxFormatter::state_mut`]
//! - [`SyntaxFormatter::map_state`]
//! - [`SyntaxFormatter::map_state_mut`]
//!
//! # Putting it all Together
//!
//! For a comprehensive example demonstrating nested structs, enums, indentation, newlines,
//! and stateful formatting with all attribute args (`eval`, `cont`, `pre`, `suf`, `delim`,
//! `eval_with`, `cont_with`, `state`, `bound`, etc.), see the
//! [examples directory](https://github.com/jngls/syntaxfmt/tree/main/examples).
//!
//! # Reference
//!
//! ## Attribute Summary
//!
//! Attributes can be applied at the type level or field level.
//!
//! | Argument | Description | Field / Type / Else |
//! |----------|-------------|---------------------|
//! | `pre` | Text before content | field/type/else |
//! | `suf` | Text after content | field/type/else |
//! | `delim` | Separator between collection elements | field/type/else |
//! | `cont` | Literal replacement for field value | field/type/else |
//! | `cont_with` | Custom formatter function/closure | field/type/else |
//! | `eval` | Conditional expression | field/type |
//! | `eval_with` | Conditional function/closure | field/type |
//! | `nl` | Newline positions (`beg`, `pre`, `cont`, `suf`) | field/type/else |
//! | `ind` | Increase indent level for field content | field/type/else |
//! | `skip` | Omit field from formatting | field/type |
//! | `state` | Specify state type (type-level only) | type |
//! | `bound` | Add trait bound to state (type-level only) | type |
//!
//! ## Modal Attributes
//!
//! Most attributes accept modal values as arrays: `[normal_value, pretty_value]`
//!
//! Examples:
//! - `pre = ["(", "( "]` - Different prefix for each mode
//! - `delim = [",", ", "]` - Different delimiter for each mode
//!
//! ## Built in Implementations
//!
//! `SyntaxFmt` is implemented for:
//! - Primitives: `i8` to `i128`, `u8` to `u128`, `isize`, `usize`, `f32`, `f64`, `bool`, `char`
//! - Strings: `str`, `String`
//! - Collections: `Vec<T>`, `&[T]`, `[T; N]`
//! - Options: `Option<T>`
//! - References: `&T`, `Box<T>`, `Rc<T>`, `Arc<T>`
//! - Tuples: Up to 8 elements
//! - Unit: `()`
//! 
//! Please [submit a ticket](https://github.com/jngls/syntaxfmt/issues/new) or
//! submit a PR if a core implementation is missing.
//!
//! ## Builder Methods
//!
//! Methods on `SyntaxDisplay` returned by `syntax_fmt()`:
//! - `.pretty()` - Enable pretty printing mode
//! - `.indent(["normal", "pretty"])` - Set indentation strings (default: `["", "    "]`)
//! - `.newline(["normal", "pretty"])` - Set newline strings (default: `["", "\n"]`)
//! - `.state(&state)` - Pass immutable state
//! - `.state_mut(&mut state)` - Pass mutable state

use core::panic;
use std::cell::{Ref, RefCell, RefMut};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;

pub use syntaxfmt_macros::SyntaxFmt;

/// Formatter mode
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Normal = 0,
    Pretty = 1,
    // When adding new modes, ensure they are not numerically inserted between existing modes
}

pub const NUM_MODES: usize = 2;
pub type Strs = [&'static str; NUM_MODES];

type Strings = [String; NUM_MODES];

// Holds state reference
enum StateRef<'s, S> {
    None(&'s S),
    Immutable(&'s S),
    Mutable(&'s mut S),
}

impl<'s, S> StateRef<'s, S> {
    #[must_use]
    #[inline]
    fn new_none(none_val: &'s S) -> Self {
        StateRef::None(none_val)
    }

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
            StateRef::None(r) => r,
            StateRef::Immutable(r) => r,
            StateRef::Mutable(r) => r,
        }
    }

    #[must_use]
    #[inline]
    #[track_caller]
    fn as_mut(&mut self) -> &mut S {
        match self {
            StateRef::None(_) => panic!("StateRef: no state has been set"),
            StateRef::Immutable(_) => panic!("StateRef: state is immutable"),
            StateRef::Mutable(r) => r,
        }
    }
}

/// Context passed to formatting implementations, containing the formatter and formatting state.
pub struct SyntaxFormatter<'sr, 's, 'f, 'w, S> {
    f: &'f mut Formatter<'w>,
    state: &'sr RefCell<StateRef<'s, S>>,
    mode: Mode,
    newline: Strs,
    single_indent: Strs,
    indent: Strings,
    delim_stack: Vec<Strs>,
}

impl<'sr, 's, 'f, 'w, S> SyntaxFormatter<'sr, 's, 'f, 'w, S> {
    #[must_use]
    #[inline]
    fn new(
        f: &'f mut Formatter<'w>,
        state: &'sr RefCell<StateRef<'s, S>>,
        newline: Strs,
        indent: Strs,
        mode: Mode,
    ) -> Self {
        Self {
            f,
            state,
            mode,
            newline,
            single_indent: indent,
            indent: Default::default(),
            delim_stack: Vec::new(),
        }
    }

    #[must_use]
    #[inline(always)]
    fn imode(&self) -> usize {
        self.mode as usize
    }

    /// Returns current mode; `Normal`, `Pretty`
    #[must_use]
    #[inline]
    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// Returns a reference to the user-defined state.
    ///
    /// # Panics
    /// * Panics if state is `()`.
    ///
    /// # Example
    ///
    /// ```
    /// use syntaxfmt::{SyntaxFmt, SyntaxFormatter, syntax_fmt};
    ///
    /// trait SymbolResolver {
    ///     fn resolve(&self, name: &str) -> String;
    /// }
    ///
    /// struct MyResolver;
    /// impl SymbolResolver for MyResolver {
    ///     fn resolve(&self, name: &str) -> String {
    ///         format!("resolved_{}", name)
    ///     }
    /// }
    ///
    /// fn resolve_name<S: SymbolResolver>(name: &str, f: &mut SyntaxFormatter<S>) -> std::fmt::Result {
    ///     let resolved = f.state().resolve(name);
    ///     write!(f, "{}", resolved)
    /// }
    ///
    /// #[derive(SyntaxFmt)]
    /// #[syntax(bound = SymbolResolver)]
    /// struct Identifier<'src> {
    ///     #[syntax(cont_with = resolve_name)]
    ///     name: &'src str,
    /// }
    ///
    /// let resolver = MyResolver;
    /// let ident = Identifier { name: "foo" };
    /// assert_eq!(format!("{}", syntax_fmt(&ident).state(&resolver)), "resolved_foo");
    /// ```
    #[must_use]
    #[inline]
    pub fn state(&self) -> Ref<S> {
        Ref::map(self.state.borrow(), |s| s.as_ref())
    }

    /// Returns a mutable reference to the user-defined state.
    ///
    /// # Panics
    /// * Panics if state is `()`.
    /// * Panics if state is immutable.
    /// 
    /// # Example
    ///
    /// ```
    /// use syntaxfmt::{SyntaxFmt, SyntaxFormatter, syntax_fmt};
    ///
    /// struct IdGenerator {
    ///     next_id: usize,
    /// }
    ///
    /// impl IdGenerator {
    ///     fn next(&mut self) -> usize {
    ///         let id = self.next_id;
    ///         self.next_id += 1;
    ///         id
    ///     }
    /// }
    ///
    /// struct VarDecl<'src> {
    ///     name: &'src str,
    /// }
    ///
    /// impl<'src> SyntaxFmt<IdGenerator> for VarDecl<'src> {
    ///     fn syntax_fmt(&self, f: &mut SyntaxFormatter<IdGenerator>) -> std::fmt::Result {
    ///         let id = f.state_mut().next();
    ///         write!(f, "let {}_{}", self.name, id)
    ///     }
    /// }
    ///
    /// let mut id_gen = IdGenerator { next_id: 0 };
    /// let x = VarDecl { name: "x" };
    /// let y = VarDecl { name: "y" };
    /// assert_eq!(format!("{}", syntax_fmt(&x).state_mut(&mut id_gen)), "let x_0");
    /// assert_eq!(format!("{}", syntax_fmt(&y).state_mut(&mut id_gen)), "let y_1");
    /// ```
    #[must_use]
    #[inline]
    #[track_caller]
    pub fn state_mut<'a>(&'a mut self) -> RefMut<'a, S> {
        RefMut::map(self.state.borrow_mut(), |s| s.as_mut())
    }

    /// Takes a closure that receives this formatter and immutable access to state and returns a value of its choice.
    ///
    /// Useful when you need concurrent access to both the formatter and state within a single expression,
    /// such as within a `write!` macro.
    ///
    /// # Arguments
    ///
    /// * `map` - mapping function with signature `FnOnce(&mut Self, &S) -> R`
    ///
    /// # Panics
    /// * Panics if state is `()`.
    ///
    /// # Example
    ///
    /// ```
    /// use syntaxfmt::{SyntaxFmt, SyntaxFormatter, syntax_fmt};
    ///
    /// trait Counter {
    ///     fn count(&self) -> usize;
    /// }
    ///
    /// struct IdCounter {
    ///     value: usize,
    /// }
    ///
    /// impl Counter for IdCounter {
    ///     fn count(&self) -> usize {
    ///         self.value
    ///     }
    /// }
    ///
    /// fn format_with_count<S: Counter>(name: &str, f: &mut SyntaxFormatter<S>) -> std::fmt::Result {
    ///     f.map_state(|f, state| write!(f, "{}#{}", name, state.count()))
    /// }
    ///
    /// #[derive(SyntaxFmt)]
    /// #[syntax(bound = Counter)]
    /// struct Item<'src> {
    ///     #[syntax(cont_with = format_with_count)]
    ///     name: &'src str,
    /// }
    ///
    /// let counter = IdCounter { value: 42 };
    /// let item = Item { name: "value" };
    /// assert_eq!(format!("{}", syntax_fmt(&item).state(&counter)), "value#42");
    /// ```
    #[must_use]
    #[inline]
    pub fn map_state<F, R>(&mut self, map: F) -> R
    where
        F: FnOnce(&mut Self, &S) -> R,
    {
        map(self, self.state.borrow().as_ref())
    }

    /// Takes a closure that receives this formatter and mutable access to state and returns a value of its choice.
    ///
    /// Useful when you need concurrent access to both the formatter and mutable state within a single expression,
    /// such as within a `write!` macro.
    ///
    /// # Arguments
    ///
    /// * `map` - mapping function with signature `FnOnce(&mut Self, &mut S) -> R`
    ///
    /// # Panics
    /// * Panics if state is `()`.
    /// * Panics if state is immutable.
    ///
    /// # Example
    ///
    /// ```
    /// use syntaxfmt::{SyntaxFmt, SyntaxFormatter, syntax_fmt};
    ///
    /// trait Counter {
    ///     fn increment(&mut self) -> usize;
    /// }
    ///
    /// struct IdCounter {
    ///     count: usize,
    /// }
    ///
    /// impl Counter for IdCounter {
    ///     fn increment(&mut self) -> usize {
    ///         let current = self.count;
    ///         self.count += 1;
    ///         current
    ///     }
    /// }
    ///
    /// fn format_with_id<S: Counter>(name: &str, f: &mut SyntaxFormatter<S>) -> std::fmt::Result {
    ///     f.map_state_mut(|f, state| {
    ///         let id = state.increment();
    ///         write!(f, "{}_{}", name, id)
    ///     })
    /// }
    ///
    /// #[derive(SyntaxFmt)]
    /// #[syntax(bound = Counter)]
    /// struct Node<'src> {
    ///     #[syntax(cont_with = format_with_id)]
    ///     name: &'src str,
    /// }
    ///
    /// let mut counter = IdCounter { count: 0 };
    /// let node = Node { name: "node" };
    /// assert_eq!(format!("{}", syntax_fmt(&node).state_mut(&mut counter)), "node_0");
    /// assert_eq!(format!("{}", syntax_fmt(&node).state_mut(&mut counter)), "node_1");
    /// ```
    #[must_use]
    #[inline]
    pub fn map_state_mut<F, R>(&mut self, map: F) -> R
    where
        F: FnOnce(&mut Self, &mut S) -> R,
    {
        map(self, self.state.borrow_mut().as_mut())
    }

    /// Writes a string set to the formatter based on current mode.
    #[must_use]
    #[inline]
    pub fn write_strs(&mut self, strs: Strs) -> FmtResult {
        write!(self.f, "{}", strs[self.imode()])
    }

    /// Increases the indentation level by one.
    #[inline]
    pub fn push_indent(&mut self) {
        (0..NUM_MODES).for_each(|i| self.indent[i].push_str(self.single_indent[i]));
    }

    /// Decreases the indentation level by one.
    #[inline]
    pub fn pop_indent(&mut self) {
        (0..NUM_MODES).for_each(|i| {
            self.indent[i].truncate(self.indent[i].len() - self.single_indent[i].len())
        });
    }

    /// Writes newline and current indentation based on current mode.
    #[inline]
    pub fn write_newline(&mut self) -> FmtResult {
        let newline = self.newline[self.imode()];
        let indent = self.indent[self.imode()].as_str();
        write!(self.f, "{newline}{indent}")
    }

    /// Pushes a new delimiter set onto the delimiter stack.
    #[inline]
    pub fn push_delim(&mut self, delim: Strs) {
        self.delim_stack.push(delim);
    }

    /// Pops the top delimiter set from the delimiter stack.
    #[inline]
    pub fn pop_delim(&mut self) {
        self.delim_stack.pop();
    }

    /// Writes the current delimiter to the output based on current mode.
    #[inline]
    pub fn write_delim(&mut self) -> FmtResult {
        let delim = self.delim_stack.last().copied();
        let delim = delim.unwrap_or([",", ", "]);
        write!(self.f, "{}", delim[self.imode()])
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
pub struct SyntaxDisplay<'s, 'e, S, E> {
    state: RefCell<StateRef<'s, S>>,
    elem: &'e E,
    newline: Strs,
    indent: Strs,
    mode: Mode,
}

impl<'s, 'e, S, E> SyntaxDisplay<'s, 'e, S, E> {
    /// Set the state to use during formatting (immutable).
    ///
    /// See [`SyntaxFormatter::state`] for usage examples.
    #[must_use]
    #[inline]
    pub fn state<'s2, S2>(self, state: &'s2 S2) -> SyntaxDisplay<'s2, 'e, S2, E> {
        SyntaxDisplay {
            state: RefCell::new(StateRef::new_ref(state)),
            elem: self.elem,
            newline: self.newline,
            indent: self.indent,
            mode: self.mode,
        }
    }

    /// Set the state to use during formatting (mutable).
    ///
    /// See [`SyntaxFormatter::state_mut`] for usage examples.
    #[must_use]
    #[inline]
    pub fn state_mut<'s2, S2>(self, state: &'s2 mut S2) -> SyntaxDisplay<'s2, 'e, S2, E> {
        SyntaxDisplay {
            state: RefCell::new(StateRef::new_mut(state)),
            elem: self.elem,
            newline: self.newline,
            indent: self.indent,
            mode: self.mode,
        }
    }

    /// Enable pretty printing mode.
    #[must_use]
    #[inline]
    pub fn pretty(mut self) -> Self {
        self.mode = Mode::Pretty;
        self
    }

    /// Set the indentation string (default is "" for Normal mode and "    " for Pretty mode).
    #[must_use]
    #[inline]
    pub fn indent(mut self, indent: Strs) -> Self {
        self.indent = indent;
        self
    }

    /// Set the newline strings (default is "" for Normal mode and "\n" for Pretty mode).
    #[must_use]
    #[inline]
    pub fn newline(mut self, newline: Strs) -> Self {
        self.newline = newline;
        self
    }
}

impl<'s, 'e, S, E> Display for SyntaxDisplay<'s, 'e, S, E>
where
    E: SyntaxFmt<S>,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut f = SyntaxFormatter::new(f, &self.state, self.newline, self.indent, self.mode);
        self.elem.syntax_fmt(&mut f)
    }
}

/// Formats a syntax tree.
///
/// Returns a [`SyntaxDisplay`] wrapper that implements `Display`, allowing it to be
/// used with `format!`, `println!`, and other formatting macros.
///
/// By default, uses `()` as the state. Chain with `.state()` or `.state_mut()` to provide
/// custom state. Chain with `.pretty()` to enable pretty printing mode and `.indent()` to
/// customize the indentation string (default is four spaces).
///
/// # Arguments
///
/// * `elem` - The syntax tree to format
///
/// # Examples
///
/// ```
/// use syntaxfmt::{SyntaxFmt, syntax_fmt};
///
/// #[derive(SyntaxFmt)]
/// struct Expr<'src> {
///     #[syntax(pre = ["(", "( "], suf = [")", " )"])]
///     value: &'src str,
/// }
///
/// let expr = Expr { value: "42" };
/// assert_eq!(format!("{}", syntax_fmt(&expr)), "(42)");
/// assert_eq!(format!("{}", syntax_fmt(&expr).pretty()), "( 42 )");
/// ```
///
/// ## With custom state
///
/// ```
/// use syntaxfmt::{Mode, SyntaxFmt, SyntaxFormatter, syntax_fmt};
///
/// struct Counter {
///     count: usize,
/// }
///
/// impl Counter {
///     pub fn next(&mut self) -> usize {
///         let count = self.count;
///         self.count += 1;
///         count
///     }
/// }
///
/// struct Item;
///
/// impl SyntaxFmt<Counter> for Item {
///     fn syntax_fmt(&self, f: &mut SyntaxFormatter<Counter>) -> std::fmt::Result {
///         let count = f.state_mut().next();
///         if f.mode() == Mode::Pretty {
///             write!(f, "pretty_item_{}", count)
///         } else {
///             write!(f, "item_{}", count)
///         }
///     }
/// }
///
/// let mut state = Counter { count: 0 };
/// let item = Item;
/// assert_eq!(format!("{}", syntax_fmt(&item).state_mut(&mut state)), "item_0");
/// assert_eq!(state.count, 1);
/// assert_eq!(format!("{}", syntax_fmt(&item).state_mut(&mut state).pretty()), "pretty_item_1");
/// assert_eq!(state.count, 2);
/// ```
#[must_use]
#[inline]
pub fn syntax_fmt<'e, E>(elem: &'e E) -> SyntaxDisplay<'static, 'e, (), E> {
    const UNIT_STATE: () = ();
    SyntaxDisplay {
        state: RefCell::new(StateRef::new_none(&UNIT_STATE)),
        elem,
        mode: Mode::Normal,
        newline: ["", "\n"],
        indent: ["", "    "],
    }
}

/// Trait for types that can be formatted as syntax.
pub trait SyntaxFmt<S> {
    /// Formats this value using the given context.
    fn syntax_fmt(&self, f: &mut SyntaxFormatter<S>) -> FmtResult;
}

impl<S, T> SyntaxFmt<S> for PhantomData<T> {
    fn syntax_fmt(&self, _ctx: &mut SyntaxFormatter<S>) -> FmtResult {
        Ok(())
    }
}

// Implement SyntaxFmt for common primitive types
macro_rules! impl_syntax_fmt_display {
    ($($ty:ty),*) => {
        $(
            impl<S> SyntaxFmt<S> for $ty {
                fn syntax_fmt(&self, f: &mut SyntaxFormatter<S>) -> FmtResult {
                    write!(f, "{}", self)
                }
            }
        )*
    };
}

impl_syntax_fmt_display!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, char, bool, str,
    String
);

impl<S, T> SyntaxFmt<S> for Option<T>
where
    T: SyntaxFmt<S>,
{
    fn syntax_fmt(&self, f: &mut SyntaxFormatter<S>) -> FmtResult {
        match self {
            Some(inner) => inner.syntax_fmt(f),
            None => Ok(()),
        }
    }
}

// Implement SyntaxFmt for collections
impl<S, T> SyntaxFmt<S> for Vec<T>
where
    T: SyntaxFmt<S>,
{
    fn syntax_fmt(&self, f: &mut SyntaxFormatter<S>) -> FmtResult {
        self.as_slice().syntax_fmt(f)
    }
}

impl<S, T> SyntaxFmt<S> for [T]
where
    T: SyntaxFmt<S>,
{
    fn syntax_fmt(&self, f: &mut SyntaxFormatter<S>) -> FmtResult {
        for (i, elem) in self.iter().enumerate() {
            if i > 0 {
                f.write_delim()?;
            }
            elem.syntax_fmt(f)?;
        }
        Ok(())
    }
}

impl<S, T, const N: usize> SyntaxFmt<S> for [T; N]
where
    T: SyntaxFmt<S>,
{
    fn syntax_fmt(&self, f: &mut SyntaxFormatter<S>) -> FmtResult {
        self.as_slice().syntax_fmt(f)
    }
}

// Implement SyntaxFmt for references and smart pointers
impl<S, T> SyntaxFmt<S> for &T
where
    T: SyntaxFmt<S> + ?Sized,
{
    fn syntax_fmt(&self, f: &mut SyntaxFormatter<S>) -> FmtResult {
        (*self).syntax_fmt(f)
    }
}

impl<S, T> SyntaxFmt<S> for Box<T>
where
    T: SyntaxFmt<S> + ?Sized,
{
    fn syntax_fmt(&self, f: &mut SyntaxFormatter<S>) -> FmtResult {
        (**self).syntax_fmt(f)
    }
}

impl<S, T> SyntaxFmt<S> for std::rc::Rc<T>
where
    T: SyntaxFmt<S> + ?Sized,
{
    fn syntax_fmt(&self, f: &mut SyntaxFormatter<S>) -> FmtResult {
        (**self).syntax_fmt(f)
    }
}

impl<S, T> SyntaxFmt<S> for std::sync::Arc<T>
where
    T: SyntaxFmt<S> + ?Sized,
{
    fn syntax_fmt(&self, f: &mut SyntaxFormatter<S>) -> FmtResult {
        (**self).syntax_fmt(f)
    }
}

impl<S, T> SyntaxFmt<S> for std::borrow::Cow<'_, T>
where
    T: SyntaxFmt<S> + ToOwned + ?Sized,
{
    fn syntax_fmt(&self, f: &mut SyntaxFormatter<S>) -> FmtResult {
        self.as_ref().syntax_fmt(f)
    }
}

// Tuple types
impl<S> SyntaxFmt<S> for () {
    fn syntax_fmt(&self, _ctx: &mut SyntaxFormatter<S>) -> FmtResult {
        Ok(())
    }
}

impl<S, T0> SyntaxFmt<S> for (T0,)
where
    T0: SyntaxFmt<S>,
{
    fn syntax_fmt(&self, f: &mut SyntaxFormatter<S>) -> FmtResult {
        self.0.syntax_fmt(f)
    }
}

macro_rules! impl_syntax_fmt_tuple {
    ($($T:ident : $idx:tt),+) => {
        impl<S, $($T),+> SyntaxFmt<S> for ($($T,)+)
        where
            $($T: SyntaxFmt<S>,)+
        {
            fn syntax_fmt(&self, f: &mut SyntaxFormatter<S>) -> FmtResult {
                $(
                    if $idx > 0 {
                        f.write_delim()?;
                    }
                    self.$idx.syntax_fmt(f)?;
                )+
                Ok(())
            }
        }
    };
}

impl_syntax_fmt_tuple!(T0: 0, T1: 1);
impl_syntax_fmt_tuple!(T0: 0, T1: 1, T2: 2);
impl_syntax_fmt_tuple!(T0: 0, T1: 1, T2: 2, T3: 3);
impl_syntax_fmt_tuple!(T0: 0, T1: 1, T2: 2, T3: 3, T4: 4);
impl_syntax_fmt_tuple!(T0: 0, T1: 1, T2: 2, T3: 3, T4: 4, T5: 5);
impl_syntax_fmt_tuple!(T0: 0, T1: 1, T2: 2, T3: 3, T4: 4, T5: 5, T6: 6);
impl_syntax_fmt_tuple!(T0: 0, T1: 1, T2: 2, T3: 3, T4: 4, T5: 5, T6: 6, T7: 7);
