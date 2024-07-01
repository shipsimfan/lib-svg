use crate::{Circle, Style, Text};
use std::fmt::{Display, Write};

/// An SVG tag
pub struct Tag<'a> {
    output: &'a mut String,
    name: &'a str,
}

impl<'a> Tag<'a> {
    /// Creates a new [`Tag`]
    pub(crate) fn new(output: &'a mut String, name: &'a str) -> Self {
        Tag { output, name }
    }

    /// Begins writing a circle
    pub fn circle(&mut self) -> Circle {
        Circle::new(self.output)
    }

    /// Adds a block of CSS to style elements
    pub fn style<'b, 'c>(&'b mut self, css: &'c str) -> Style<'b, 'c> {
        Style::new(self.output, css)
    }

    /// Adds a block of text
    pub fn text(&mut self) -> Text {
        Text::new(self.output)
    }

    /// Writes `d` directly to the output
    pub(crate) fn write<D: Display>(&mut self, d: D) {
        write!(self.output, "{}", d).unwrap();
    }
}

impl<'a> Drop for Tag<'a> {
    fn drop(&mut self) {
        write!(self.output, "</{}>", self.name).unwrap();
    }
}
