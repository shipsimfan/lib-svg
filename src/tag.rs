use crate::Circle;
use std::fmt::Write;

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
    pub fn style(&mut self, css: &str) {
        write!(self.output, "<style>{}</style>", css).unwrap();
    }
}

impl<'a> Drop for Tag<'a> {
    fn drop(&mut self) {
        write!(self.output, "</{}>", self.name).unwrap();
    }
}
