use std::io::{self, Write};

use termion::{color, style};

use crate::{sources::Sources, span::Span};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Errors {
    errors: Vec<Error>,
    warnings: Vec<Error>,
}

impl Errors {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn err(&mut self, title: impl Into<String>) -> &mut Error {
        self.errors.push(Error::new(title));
        self.errors.last_mut().unwrap()
    }

    pub fn warn(&mut self, title: impl Into<String>) -> &mut Error {
        self.warnings.push(Error::new(title));
        self.warnings.last_mut().unwrap()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Error {
    title: String,
    notes: Vec<ErrorNote>,
    span: Span,
}

impl Error {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            notes: Vec::new(),
            span: Span::null(),
        }
    }

    pub fn set_span(&mut self, span: Span) -> &mut Self {
        self.span = span;
        self
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.set_span(span);
        self
    }

    pub fn add_note(&mut self, note: impl Into<String>) -> &mut Self {
        self.notes.push(ErrorNote::new(note));
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.add_note(note);
        self
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ErrorNote {
    note: String,
}

impl ErrorNote {
    pub fn new(note: impl Into<String>) -> Self {
        Self { note: note.into() }
    }

    pub fn note(&self) -> &str {
        &self.note
    }
}

pub struct ErrorFormatter<'a> {
    sources: &'a Sources,
    writer: &'a mut dyn Write,
}

impl<'a> ErrorFormatter<'a> {
    pub fn new(sources: &'a Sources, writer: &'a mut dyn Write) -> Self {
        Self { sources, writer }
    }

    pub fn format_error(&mut self, error: &Error) -> io::Result<()> {
        writeln!(
            self,
            "{bold}{red}error{color_reset}: {}{style_reset}",
            error.title(),
            red = color::Fg(color::Red),
            color_reset = color::Fg(color::Reset),
            bold = style::Bold,
            style_reset = style::Reset,
        )?;

        if let Some(span) = self.sources.get_span(error.span) {
            writeln!(
                self,
                "{blue}    --> {color_reset}{}:{}:{}",
                span.path().display(),
                span.line(),
                span.column(),
                blue = color::Fg(color::Blue),
                color_reset = color::Fg(color::Reset),
            )?;

            writeln!(
                self,
                "{blue}     |{color_reset}",
                blue = color::Fg(color::Blue),
                color_reset = color::Fg(color::Reset),
            )?;

            for (i, line) in span.source().lines().enumerate() {
                let line_number = span.line() + i;
                let line_number = format!("{: >4}", line_number);

                writeln!(
                    self,
                    "{blue}{} |{color_reset} {}",
                    line_number,
                    line,
                    blue = color::Fg(color::Blue),
                    color_reset = color::Fg(color::Reset),
                )?;
            }
        }

        Ok(())
    }
}

impl<'a> Write for ErrorFormatter<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}
