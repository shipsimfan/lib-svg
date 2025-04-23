use crate::OpenTag;

/// Builds a line
pub struct Line<'a>(OpenTag<'a>);

impl<'a> Line<'a> {
    /// Creates a new line builder
    pub(crate) fn new(output: &'a mut String) -> Self {
        Line(OpenTag::new(output, "line"))
    }

    /// Sets the starting x position
    pub fn x1(mut self, x: f32) -> Self {
        self.0.add_attribute("x1", x);
        self
    }

    /// Sets the starting y position
    pub fn y1(mut self, y: f32) -> Self {
        self.0.add_attribute("y1", y);
        self
    }

    /// Sets the ending x position
    pub fn x2(mut self, x: f32) -> Self {
        self.0.add_attribute("x2", x);
        self
    }

    /// Sets the ending y position
    pub fn y2(mut self, y: f32) -> Self {
        self.0.add_attribute("y2", y);
        self
    }

    /// Sets the stroke width
    pub fn stroke_width(mut self, width: &str) -> Self {
        self.0.add_attribute("stroke-width", width);
        self
    }

    /// Sets the stroke
    pub fn stroke(mut self, stroke: &str) -> Self {
        self.0.add_attribute("stroke", stroke);
        self
    }

    /// Sets the class
    pub fn class(mut self, class: &str) -> Self {
        self.0.add_attribute("class", class);
        self
    }
}
