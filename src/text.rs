use crate::OpenTag;
use std::fmt::Display;

/// Builds some text
pub struct Text<'a> {
    tag: OpenTag<'a>,
    x: f32,
}

impl<'a> Text<'a> {
    /// Creates a new text builder
    pub(crate) fn new(output: &'a mut String) -> Self {
        Text {
            tag: OpenTag::new(output, "text"),
            x: 0.0,
        }
    }

    /// Sets the x position
    pub fn x(mut self, x: f32) -> Self {
        self.tag.add_attribute("x", x);
        self.x = x;
        self
    }

    /// Sets the y position
    pub fn y(mut self, y: f32) -> Self {
        self.tag.add_attribute("y", y);
        self
    }

    /// Sets the class
    pub fn class(mut self, class: &str) -> Self {
        self.tag.add_attribute("class", class);
        self
    }

    /// Writes the text
    pub fn value<D: Display>(self, text: D) {
        self.tag.end().write(text)
    }

    /// Writes the text as seperate lines
    pub fn values<D: Display>(self, dy: &str, text: impl IntoIterator<Item = D>) {
        let mut tag = self.tag.end();

        let mut first = true;
        for value in text {
            tag.write(format_args!(
                "<tspan x=\"{}\" dy=\"{}\">{}</tspan>",
                self.x,
                if first {
                    first = false;
                    "0"
                } else {
                    dy
                },
                value
            ));
        }
    }
}
