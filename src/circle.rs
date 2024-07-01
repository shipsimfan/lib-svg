use crate::OpenTag;

/// Builds a circle
pub struct Circle<'a>(OpenTag<'a>);

impl<'a> Circle<'a> {
    /// Creates a new circle builder
    pub(crate) fn new(output: &'a mut String) -> Self {
        Circle(OpenTag::new(output, "circle"))
    }

    /// Sets the center x position
    pub fn cx(mut self, cx: f32) -> Self {
        self.0.add_attribute("cx", cx);
        self
    }

    /// Sets the center y position
    pub fn cy(mut self, cy: f32) -> Self {
        self.0.add_attribute("cy", cy);
        self
    }

    /// Sets the radius
    pub fn r(mut self, r: f32) -> Self {
        self.0.add_attribute("r", r);
        self
    }

    /// Sets the fill
    pub fn fill(mut self, fill: &str) -> Self {
        self.0.add_attribute("fill", fill);
        self
    }

    /// Sets the class
    pub fn class(mut self, class: &str) -> Self {
        self.0.add_attribute("class", class);
        self
    }
}
