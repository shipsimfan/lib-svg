use crate::OpenTag;
use std::fmt::Display;

/// Builds some text
pub struct Text<'a>(OpenTag<'a>);

impl<'a> Text<'a> {
    /// Creates a new text builder
    pub(crate) fn new(output: &'a mut String) -> Self {
        Text(OpenTag::new(output, "text"))
    }

    /// Sets the x position
    pub fn x(mut self, x: f32) -> Self {
        self.0.add_attribute("x", x);
        self
    }

    /// Sets the y position
    pub fn y(mut self, y: f32) -> Self {
        self.0.add_attribute("y", y);
        self
    }

    /// Sets the class
    pub fn class(mut self, class: &str) -> Self {
        self.0.add_attribute("class", class);
        self
    }

    /// Writes the text
    pub fn value<D: Display>(self, text: D) {
        self.0.end().write(text)
    }

    /// Writes the text as seperate lines
    pub fn values<D: Display>(self, text: impl Iterator<Item = D>) {
        let mut tag = self.0.end();

        for value in text {
            tag.write(format_args!("<tspan>{}</tspan>", value));
        }
    }
}
