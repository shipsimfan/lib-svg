use crate::{OpenTag, Tag};

/// Builds some CSS
pub struct Style<'a, 'b> {
    tag: Tag<'a>,
    css: &'b str,
}

impl<'a, 'b> Style<'a, 'b> {
    /// Creates a new css builder
    pub(crate) fn new(output: &'a mut String, css: &'b str) -> Self {
        Style {
            tag: OpenTag::new(output, "style").end(),
            css,
        }
    }
}

impl<'a, 'b> Drop for Style<'a, 'b> {
    fn drop(&mut self) {
        self.tag.write(self.css);
    }
}
