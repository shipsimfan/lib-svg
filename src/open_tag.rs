use crate::Tag;
use std::fmt::{Display, Write};

/// An SVG opening tag
pub(crate) struct OpenTag<'a> {
    output: &'a mut String,
    name: &'a str,

    print_self_close: bool,
}

impl<'a> OpenTag<'a> {
    /// Creates a new [`OpenTag`]
    pub(crate) fn new(output: &'a mut String, name: &'a str) -> Self {
        write!(output, "<{}", name).unwrap();

        OpenTag {
            output,
            name,
            print_self_close: true,
        }
    }

    /// Adds an attribute to this tag
    pub(crate) fn add_attribute<D: Display>(&mut self, name: &str, value: D) {
        write!(self.output, " {}=\"{}\"", name, value).unwrap()
    }

    /// Ends the opening tag allowing children to be added
    pub(crate) fn end(mut self) -> Tag<'a> {
        // Extract variables from this before dropping
        let name = self.name;
        let output = self.output as *mut String;

        self.print_self_close = false;
        drop(self);

        // Get the mutable reference to the string back.
        //
        // This is safe because the string that `self.output` references isn't actually dropped
        // until the lifetime finishes, and the return value of the function guarantees that
        // lifetime is greater than this function. That lifetime also guarantees we still hold the
        // mutable reference.
        let output = unsafe { &mut *output };

        // Write the standard closing tag
        output.push_str(">");
        Tag::new(output, name)
    }
}

impl<'a> Drop for OpenTag<'a> {
    fn drop(&mut self) {
        if !self.print_self_close {
            return;
        }

        self.output.push_str(" />")
    }
}
