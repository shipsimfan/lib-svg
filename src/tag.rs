use crate::{prefix, Circle};
use std::fmt::Write;

/// An SVG tag
pub struct Tag<'a> {
    output: &'a mut String,
    name: &'a str,
    depth: usize,
}

impl<'a> Tag<'a> {
    /// Creates a new [`Tag`]
    pub(crate) fn new(output: &'a mut String, name: &'a str, depth: usize) -> Self {
        Tag {
            output,
            name,
            depth,
        }
    }

    /// Begins writing a circle
    pub fn circle(&mut self) -> Circle {
        Circle::new(self.output, self.depth)
    }
}

impl<'a> Drop for Tag<'a> {
    fn drop(&mut self) {
        prefix(self.output, self.depth - 1);
        writeln!(self.output, "</{}>", self.name).unwrap();
    }
}
