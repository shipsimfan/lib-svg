use crate::OpenTag;

/// Builds a rectangle
pub struct Rect<'a>(OpenTag<'a>);

impl<'a> Rect<'a> {
    /// Creates a new rectangle builder
    pub(crate) fn new(output: &'a mut String) -> Self {
        Rect(OpenTag::new(output, "rect"))
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

    /// Sets the width
    pub fn width(mut self, width: f32) -> Self {
        self.0.add_attribute("width", width);
        self
    }

    /// Sets the height
    pub fn height(mut self, height: f32) -> Self {
        self.0.add_attribute("height", height);
        self
    }

    /// Sets the fill
    pub fn fill(mut self, fill: &str) -> Self {
        self.0.add_attribute("fill", fill);
        self
    }

    /// Sets the corner radius
    pub fn rx(mut self, rx: f32) -> Self {
        self.0.add_attribute("rx", rx);
        self
    }

    /// Sets the class
    pub fn class(mut self, class: &str) -> Self {
        self.0.add_attribute("class", class);
        self
    }
}
