use crate::OpenTag;

/// Builds an image
pub struct Image<'a>(OpenTag<'a>);

impl<'a> Image<'a> {
    /// Creates a new image builder
    pub(crate) fn new(output: &'a mut String) -> Self {
        Image(OpenTag::new(output, "image"))
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

    /// Sets the source
    pub fn href(mut self, href: &str) -> Self {
        self.0.add_attribute("href", href);
        self
    }

    /// Sets an external source
    pub fn xlink_href(mut self, href: &str) -> Self {
        self.0.add_attribute("xlink:href", href);
        self
    }

    /// Sets the class
    pub fn class(mut self, class: &str) -> Self {
        self.0.add_attribute("class", class);
        self
    }
}
