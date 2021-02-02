//! The `Style` type is a simplified view of the various
//! attributes offered by the `term` library. These are
//! enumerated as bits so they can be easily or'd together
//! etc.

use std::default::Default;
use std::io;
use termcolor::{ColorSpec, WriteColor};

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Style {
    bits: u64,
}

macro_rules! declare_styles {
    ($($style:ident,)*) => {
        #[derive(Copy, Clone)]
        #[allow(non_camel_case_types)]
        enum StyleBit {
            $($style,)*
        }

        $(
            pub const $style: Style = Style { bits: 1 << (StyleBit::$style as u64) };
        )*
    }
}

pub const DEFAULT: Style = Style { bits: 0 };

declare_styles! {
    // Foreground colors:
    FG_BLACK,
    FG_BLUE,
    FG_BRIGHT_BLACK,
    FG_BRIGHT_BLUE,
    FG_BRIGHT_CYAN,
    FG_BRIGHT_GREEN,
    FG_BRIGHT_MAGENTA,
    FG_BRIGHT_RED,
    FG_BRIGHT_WHITE,
    FG_BRIGHT_YELLOW,
    FG_CYAN,
    FG_GREEN,
    FG_MAGENTA,
    FG_RED,
    FG_WHITE,
    FG_YELLOW,

    // Background colors:
    BG_BLACK,
    BG_BLUE,
    BG_BRIGHT_BLACK,
    BG_BRIGHT_BLUE,
    BG_BRIGHT_CYAN,
    BG_BRIGHT_GREEN,
    BG_BRIGHT_MAGENTA,
    BG_BRIGHT_RED,
    BG_BRIGHT_WHITE,
    BG_BRIGHT_YELLOW,
    BG_CYAN,
    BG_GREEN,
    BG_MAGENTA,
    BG_RED,
    BG_WHITE,
    BG_YELLOW,

    // Other:
    BOLD,
    DIM,
    ITALIC,
    UNDERLINE,
    BLINK,
    STANDOUT,
    REVERSE,
    SECURE,
}

impl Style {
    pub fn new() -> Style {
        Style::default()
    }

    pub fn with(self, other_style: Style) -> Style {
        Style {
            bits: self.bits | other_style.bits,
        }
    }

    pub fn contains(self, other_style: Style) -> bool {
        self.with(other_style) == self
    }

    /// Attempts to apply the given style to the given terminal. If
    /// the style is not supported, either there is no effect or else
    /// a similar, substitute style may be applied.
    pub fn apply<T: io::Write + WriteColor + ?Sized>(self, term: &mut T) -> io::Result<()> {
        term.reset()?;

        let mut spec = ColorSpec::new();

        macro_rules! fg_color {
            ($color:expr, $term_color:ident, bright) => {
                if self.contains($color) {
                    if term.supports_color() {
                        spec.set_fg(Some(termcolor::Color::$term_color))
                            .set_intense(true);
                    }
                }
            };
            ($color:expr, $term_color:ident) => {
                if self.contains($color) {
                    if term.supports_color() {
                        spec.set_fg(Some(termcolor::Color::$term_color));
                    }
                }
            };
        }

        fg_color!(FG_BLACK, Black);
        fg_color!(FG_BLUE, Blue);
        fg_color!(FG_BRIGHT_BLACK, Black, bright);
        fg_color!(FG_BRIGHT_BLUE, Blue, bright);
        fg_color!(FG_BRIGHT_CYAN, Cyan, bright);
        fg_color!(FG_BRIGHT_GREEN, Green, bright);
        fg_color!(FG_BRIGHT_MAGENTA, Magenta, bright);
        fg_color!(FG_BRIGHT_RED, Red, bright);
        fg_color!(FG_BRIGHT_WHITE, White, bright);
        fg_color!(FG_BRIGHT_YELLOW, Yellow, bright);
        fg_color!(FG_CYAN, Cyan);
        fg_color!(FG_GREEN, Green);
        fg_color!(FG_MAGENTA, Magenta);
        fg_color!(FG_RED, Red);
        fg_color!(FG_WHITE, White);
        fg_color!(FG_YELLOW, Yellow);

        macro_rules! bg_color {
            ($color:expr, $term_color:ident, bright) => {
                if self.contains($color) {
                    if term.supports_color() {
                        spec.set_bg(Some(termcolor::Color::$term_color))
                            .set_intense(true);
                    }
                }
            };
            ($color:expr, $term_color:ident) => {
                if self.contains($color) {
                    if term.supports_color() {
                        spec.set_bg(Some(termcolor::Color::$term_color));
                    }
                }
            };
        }

        bg_color!(BG_BLACK, Black);
        bg_color!(BG_BLUE, Blue);
        bg_color!(BG_BRIGHT_BLACK, Black, bright);
        bg_color!(BG_BRIGHT_BLUE, Blue, bright);
        bg_color!(BG_BRIGHT_CYAN, Cyan, bright);
        bg_color!(BG_BRIGHT_GREEN, Green, bright);
        bg_color!(BG_BRIGHT_MAGENTA, Magenta, bright);
        bg_color!(BG_BRIGHT_RED, Red, bright);
        bg_color!(BG_BRIGHT_WHITE, White, bright);
        bg_color!(BG_BRIGHT_YELLOW, Yellow, bright);
        bg_color!(BG_CYAN, Cyan);
        bg_color!(BG_GREEN, Green);
        bg_color!(BG_MAGENTA, Magenta);
        bg_color!(BG_RED, Red);
        bg_color!(BG_WHITE, White);
        bg_color!(BG_YELLOW, Yellow);

        macro_rules! attr {
            ($attr:expr, $builder_fn:ident) => {
                if self.contains($attr) {
                    spec.$builder_fn(true);
                }
            };
        }

        attr!(BOLD, set_bold);
        attr!(DIM, set_dimmed);
        attr!(ITALIC, set_italic);
        attr!(UNDERLINE, set_underline);

        // TODO FIXME in order to fully migrate to termcolor without breaking
        // support for existing code, we'll need to implement these attributes
        //
        // attr!(REVERSE);
        // attr!(SECURE);
        // attr!(STANDOUT);
        // attr!(BLINK);

        term.set_color(&spec)
    }
}

///////////////////////////////////////////////////////////////////////////

pub struct StyleCursor<'term, T: ?Sized + io::Write + WriteColor> {
    current_style: Style,
    term: &'term mut T,
}

impl<'term, T: ?Sized + io::Write + WriteColor> StyleCursor<'term, T> {
    pub fn new(term: &'term mut T) -> io::Result<StyleCursor<'term, T>> {
        let current_style = Style::default();
        current_style.apply(term)?;
        Ok(StyleCursor {
            current_style: current_style,
            term: term,
        })
    }

    pub fn term(&mut self) -> &mut T {
        self.term
    }

    pub fn set_style(&mut self, style: Style) -> io::Result<()> {
        if style != self.current_style {
            style.apply(self.term)?;
            self.current_style = style;
        }
        Ok(())
    }
}
