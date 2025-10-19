use std::cell::Cell;
use std::fmt::{Arguments, Display, Formatter, Result as FmtResult};

pub use syntaxfmt_macros::SyntaxFmt;

pub struct SyntaxFmtContext<'a, 'fmt, State> {
    f: &'a mut Formatter<'fmt>,
    state: Cell<&'a State>,
    ind: usize,
    pretty: bool,
}

impl<'a, 'fmt, State> SyntaxFmtContext<'a, 'fmt, State> {
    #[must_use]
    #[inline]
    fn new(f: &'a mut Formatter<'fmt>, state: &'a State) -> Self {
        Self {
            f,
            state: Cell::new(state),
            ind: 0,
            pretty: false,
        }
    }

    #[must_use]
    #[inline]
    fn new_pretty(f: &'a mut Formatter<'fmt>, state: &'a State) -> Self {
        Self {
            f,
            state: Cell::new(state),
            ind: 0,
            pretty: true,
        }
    }

    #[must_use]
    #[inline]
    pub fn is_pretty(&self) -> bool {
        self.pretty
    }

    #[must_use]
    #[inline]
    pub fn state(&self) -> &'a State {
        self.state.get()
    }

    #[inline]
    pub fn write_fmt(&mut self, args: Arguments<'_>) -> FmtResult {
        self.f.write_fmt(args)
    }

    #[inline]
    pub fn write_str(&mut self, s: &str) -> FmtResult {
        self.f.write_str(s)
    }

    #[inline]
    pub fn indent(&mut self, indent: &str) -> FmtResult {
        write!(self.f, "{}", indent.repeat(self.ind))
    }

    #[inline]
    pub fn inc_indent(&mut self) {
        self.ind += 1;
    }

    #[inline]
    pub fn dec_indent(&mut self) {
        self.ind = self.ind.saturating_sub(1);
    }
}

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

pub trait SyntaxFmt<State> {
    const DELIM: &'static str = ",";
    const PRETTY_DELIM: &'static str = ", ";
    const INDENT: &'static str = "    ";

    fn syntax_fmt(&self, ctx: &mut SyntaxFmtContext<State>) -> FmtResult;
}

impl<State, T> SyntaxFmt<State> for T
where
    T: Display,
{
    fn syntax_fmt(&self, ctx: &mut SyntaxFmtContext<State>) -> FmtResult {
        write!(ctx, "{}", *self)
    }
}
