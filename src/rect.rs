use crate::OpenTag;

/// Builds a rectangle
pub struct Rect<'a>(OpenTag<'a>);

impl<'a> Rect<'a> {
    /// Creates a new rectangle builder
    pub(crate) fn new(output: &'a mut String) -> Self {
        Rect(OpenTag::new(output, "rect"))
    }

    /// Sets the x position
    pub fn x(mut self, cx: f32) -> Self {
        self.0.add_attribute("cx", cx);
        self
    }

    /// Sets the y position
    pub fn y(mut self, cy: f32) -> Self {
        self.0.add_attribute("cy", cy);
        self
    }

    /// Sets the width
    pub fn width(mut self, r: f32) -> Self {
        self.0.add_attribute("r", r);
        self
    }

    /// Sets the height
    pub fn height(mut self, fill: &str) -> Self {
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
