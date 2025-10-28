//! A derive macro based library for flexible syntax tree formatting with pretty printing support.
//!
//! `syntaxfmt` provides a trait based approach to formatting syntax trees with both compact
//! and pretty-printed output modes. It's designed for compiler frontends, code generators,
//! and any application that needs to format structured data as text with optional formatting.
//!
//! # Features
//!
//! - **Derive macro** - Automatic implementation of formatting logic
//! - **Dual formatting modes** - Compact and pretty-printed output
//! - **Collection support** - Automatic formatting for `Vec<T>`, `&[T]`, and `[T; N]` with custom delimiters
//! - **Conditional formatting** - `eval` attribute for `bool` and `Option<T>` types, with `syntax_else` for else branches
//! - **Newline control** - Fine-grained positioning with `nl` attribute (`beg`, `pre`, `cont`, `suf`)
//! - **Indentation** - `ind` attribute for automatic indentation regions in pretty mode
//! - **Stateful formatting** - Pass mutable state through formatting with trait bounds
//! - **Custom formatters** - Override field formatting with custom functions via `cont` attribute
//! - **Minimal syntax** - Simple, composable attributes: `pre`, `suf`, `delim`, `eval`, `nl`, `ind`, `cont`, `skip`
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
//!     name: &'src str,
//!     #[syntax(format = "({*})", pretty_format = "( {*} )")]
//!     args: &'src str,
//! }
//!
//! let call = FunctionCall {
//!     name: "println",
//!     args: "\"Hello, world!\"",
//! };
//!
//! // Compact formatting
//! assert_eq!(format!("{}", syntax_fmt(&call)), "println(\"Hello, world!\")");
//!
//! // Pretty formatting with .pretty()
//! assert_eq!(format!("{}", syntax_fmt(&call).pretty()), "println( \"Hello, world!\" )");
//! ```
//!
//! # Derive Macro Attributes
//!
//! ## Type-level attributes
//!
//! - `#[syntax(delim = ", ")]` - Delimiter between items of this type, used by Vec and slice implementations (default: `","`)
//! - `#[syntax(pretty_delim = ",\n")]` - Delimiter in pretty mode (default: `", "`)
//! - `#[syntax(format = "prefix{*}suffix")]` - For prefixes and suffixes around the whole type (default: `"{*}"`)
//! - `#[syntax(pretty_format = "prefix{*}suffix")]` - For pretty prefixes and suffixes around the whole type (default: `"{*}"`)
//! - `#[syntax(state_bound = "MyTrait")]` - Add trait bound for exposing functionality to custom formatter functions
//!
//! ## Field-level attributes
//!
//! - `#[syntax(format = "prefix{*}suffix")]` - For prefixes and suffixes around the field (default: `"{*}"`)
//! - `#[syntax(pretty_format = "prefix{*}suffix")]` - For pretty prefixes and suffixes around the field (default: `"{*}"`)
//! - `#[syntax(content = my_formatter)]` - Custom content formatter function
//! - `#[syntax(empty_suffix = ";")]` - Early out with this string when field is empty (for types which implement `is_empty()` function)
//! - `#[syntax(indent)]` - Write indentation before this field (pretty mode only)
//! - `#[syntax(indent_region)]` - Increase indent level for this field's content
//! - `#[syntax(skip)]` - Skip this field during formatting
//!
//! # Examples
//!
//! ## Basic struct formatting
//!
//! Note that syntax attribute may be used at the type level or at the field level.
//! 
//! ```
//! use syntaxfmt::{SyntaxFmt, syntax_fmt};
//!
//! #[derive(SyntaxFmt)]
//! #[syntax(format = "let {*};")]
//! struct LetStatement<'src> {
//!     name: &'src str,
//!     #[syntax(format = " = {*}")]
//!     value: &'src str,
//! }
//!
//! let stmt = LetStatement {
//!     name: "x",
//!     value: "42",
//! };
//!
//! assert_eq!(format!("{}", syntax_fmt(&stmt)), "let x = 42;");
//! ```
//!
//! ## Optional and boolean fields
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, syntax_fmt};
//!
//! #[derive(SyntaxFmt)]
//! #[syntax(format = "{*};")]
//! struct ConstStatement<'src> {
//!     #[syntax(format = "pub ")]
//!     is_pub: bool,
//!     #[syntax(format = "const {*}: i32")]
//!     name: &'src str,
//!     #[syntax(format = " = {*}")]
//!     value: Option<i32>,
//! }
//!
//! let stmt = ConstStatement {
//!     is_pub: false,
//!     name: "X",
//!     value: None,
//! };
//!
//! assert_eq!(format!("{}", syntax_fmt(&stmt)), "const X: i32;");
//!
//! let pub_stmt = ConstStatement {
//!     is_pub: true,
//!     name: "X",
//!     value: None,
//! };
//!
//! assert_eq!(format!("{}", syntax_fmt(&pub_stmt)), "pub const X: i32;");
//!
//! let value_stmt = ConstStatement {
//!     is_pub: true,
//!     name: "X",
//!     value: Some(42),
//! };
//!
//! assert_eq!(format!("{}", syntax_fmt(&value_stmt)), "pub const X: i32 = 42;");
//! ```
//!
//! ## Pretty printing with indentation
//!
//! The `indent_region` attribute increases the indent level for a field's content, and `indent`
//! writes indentation before formatting a field in pretty mode. The default indentation is four
//! spaces, which can be customized using the `.indent()` builder method.
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, syntax_fmt};
//!
//! #[derive(SyntaxFmt)]
//! struct Statement<'src> {
//!     #[syntax(format = "{*};", indent)]
//!     code: &'src str,
//! }
//!
//! #[derive(SyntaxFmt)]
//! struct Block<'src> {
//!     #[syntax(
//!         format = "{{{*}}}",
//!         pretty_format = "{{\n{*}\n}}",
//!         indent_region
//!     )]
//!     body: Statement<'src>,
//! }
//!
//! let block = Block { body: Statement { code: "return 42" } };
//!
//! assert_eq!(format!("{}", syntax_fmt(&block)), "{return 42;}");
//! assert_eq!(format!("{}", syntax_fmt(&block).pretty()), "{\n    return 42;\n}");
//!
//! assert_eq!(format!("{}", syntax_fmt(&block).pretty().indent("\t")), "{\n\treturn 42;\n}");
//! ```
//!
//! ## Using `empty_suffix` for empty collections
//!
//! The `empty_suffix` attribute provides early-out formatting for empty collection fields.
//! When the field's `is_empty()` function returns true, only the suffix is output instead
//! of the remainder of the struct fields. This is useful for syntax like `mod name;` vs
//! `mod name { items }`.
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, syntax_fmt};
//!
//! #[derive(SyntaxFmt)]
//! #[syntax(delim = " ", pretty_delim = " ")]
//! struct Statement<'src>(&'src str);
//!
//! #[derive(SyntaxFmt)]
//! #[syntax(format = "mod {*}")]
//! struct Module<'src> {
//!     name: &'src str,
//!     #[syntax(format = " {{{*}}}", empty_suffix = ";")]
//!     items: Vec<Statement<'src>>,
//! }
//!
//! let empty = Module {
//!     name: "empty",
//!     items: vec![],
//! };
//! assert_eq!(format!("{}", syntax_fmt(&empty)), "mod empty;");
//!
//! let with_items = Module {
//!     name: "lib",
//!     items: vec![Statement("fn main()")],
//! };
//! assert_eq!(format!("{}", syntax_fmt(&with_items)), "mod lib {fn main()}");
//! ```
//!
//! ## Collection formatting
//!
//! Collections (`Vec<T>`, `&[T]`, `[T; N]`) are automatically formatted by iterating over their
//! elements and using the delimiter specified by the nearest outer delimiter attribute.
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, syntax_fmt};
//!
//! #[derive(SyntaxFmt)]
//! struct Segment<'src>(&'src str);
//!
//! #[derive(SyntaxFmt)]
//! struct Path<'src> {
//!     #[syntax(delim = "::", pretty_delim = " :: ")]
//!     segments: Vec<Segment<'src>>,
//! }
//!
//! let path = Path {
//!     segments: vec![Segment("std"), Segment("collections"), Segment("HashMap")],
//! };
//!
//! assert_eq!(
//!     format!("{}", syntax_fmt(&path)),
//!     "std::collections::HashMap"
//! );
//! assert_eq!(
//!     format!("{}", syntax_fmt(&path).pretty()),
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
//! fn quote_formatter<S>(value: &str, f: &mut SyntaxFormatter<S>) -> std::fmt::Result {
//!     write!(f, "\"{}\"", value)
//! }
//!
//! #[derive(SyntaxFmt)]
//! struct StringLiteral<'src> {
//!     #[syntax(content = quote_formatter)]
//!     value: &'src str,
//! }
//!
//! let lit = StringLiteral { value: "hello" };
//! assert_eq!(format!("{}", syntax_fmt(&lit)), "\"hello\"");
//! ```
//!
//! ## Stateful formatting with mutable state
//!
//! You can manually implement `SyntaxFmt` or use custom formatter functions to access and
//! modify user-provided state during formatting.
//!
//! ```
//! use syntaxfmt::{SyntaxFmt, SyntaxFormatter, syntax_fmt};
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
//!     fn syntax_fmt(&self, f: &mut SyntaxFormatter<VarTracker>) -> std::fmt::Result {
//!         let id = f.state_mut().allocate();
//!         write!(f, "let {}_{} = ", self.name, id)
//!     }
//! }
//!
//! let mut tracker = VarTracker { next_id: 0 };
//! let decl_0 = VarDecl { name: "x" };
//! let decl_1 = VarDecl { name: "x" };
//! assert_eq!(format!("{}", syntax_fmt(&decl_0).state_mut(&mut tracker)), "let x_0 = ");
//! assert_eq!(format!("{}", syntax_fmt(&decl_1).state_mut(&mut tracker)), "let x_1 = ");
//! assert_eq!(tracker.next_id, 2);
//! ```

use core::panic;
use std::cell::{Ref, RefCell, RefMut};
// use std::ffi::{OsStr, OsString};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Deref, DerefMut};

pub use syntaxfmt_macros::SyntaxFmt;

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
    fn new(f: &'f mut Formatter<'w>, state: &'sr RefCell<StateRef<'s, S>>, newline: Strs, indent: Strs, mode: Mode) -> Self {
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

    /// Takes a closure that receives this formatter and immutable access to state and returns a value of its choice.
    #[must_use]
    #[inline]
    pub fn map_state<F, R>(&mut self, map: F) -> R where F: FnOnce(&mut Self, &S) -> R {
        map(self, self.state.borrow().as_ref())
    }

    /// Takes a closure that receives this formatter and mutable access to state and returns a value of its choice.
    #[must_use]
    #[inline]
    pub fn map_state_mut<F, R>(&mut self, map: F) -> R where F: FnOnce(&mut Self, &mut S) -> R {
        map(self, self.state.borrow_mut().as_mut())
    }

    /// Writes a dual string to the formatter based on prettiness.
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
        (0..NUM_MODES).for_each(|i| self.indent[i].truncate(self.indent[i].len() - self.single_indent[i].len()));
    }
    
    /// Writes newline and current indentation if in pretty mode.
    #[inline]
    pub fn write_newline(&mut self) -> FmtResult {
        let newline = self.newline[self.imode()];
        let indent = self.indent[self.imode()].as_str();
        write!(self.f, "{newline}{indent}")
    }

    /// Pushes a new delimiter pair onto the delimiter stack.
    #[inline]
    pub fn push_delim(&mut self, delim: Strs) {
        self.delim_stack.push(delim);
    }

    /// Pops the top delimiter pair from the delimiter stack.
    #[inline]
    pub fn pop_delim(&mut self) {
        self.delim_stack.pop();
    }

    /// Writes the current delimiter to the output based on pretty mode.
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
///     #[syntax(format = "({*})", pretty_format = "( {*} )")]
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
/// use syntaxfmt::{SyntaxFmt, SyntaxFormatter, syntax_fmt};
///
/// struct Counter {
///     count: usize,
/// }
///
/// struct Item;
///
/// impl SyntaxFmt<Counter> for Item {
///     fn syntax_fmt(&self, f: &mut SyntaxFormatter<Counter>) -> std::fmt::Result {
///         let count = f.state().count;
///         f.state_mut().count += 1;
///         if f.is_pretty() {
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
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64,
    char, bool,
    str, String
);

impl<S, T> SyntaxFmt<S> for Option<T> where T: SyntaxFmt<S> {
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
