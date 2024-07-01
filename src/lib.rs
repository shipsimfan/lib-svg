//! SVG generation library

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

use open_tag::OpenTag;
use std::ops::{Deref, DerefMut};

mod circle;
mod open_tag;
mod rect;
mod style;
mod tag;
mod text;

pub use circle::Circle;
pub use rect::Rect;
pub use style::Style;
pub use tag::Tag;
pub use text::Text;

/// Utility for writing SVGs
pub struct SVGWriter<'a>(Tag<'a>);

impl<'a> SVGWriter<'a> {
    /// Creates a new [`SVGWriter`]
    pub fn new(output: &'a mut String, width: usize, height: usize) -> Self {
        let mut tag = OpenTag::new(output, "svg");
        tag.add_attribute("version", "1.1");
        tag.add_attribute("baseProfile", "full");
        tag.add_attribute("xmlns", "http://www.w3.org/2000/svg");
        tag.add_attribute("width", width);
        tag.add_attribute("height", height);
        SVGWriter(tag.end())
    }
}

impl<'a> Deref for SVGWriter<'a> {
    type Target = Tag<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for SVGWriter<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
