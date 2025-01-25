use core::fmt::Write;

use maud_v026::{Markup, PreEscaped, Render};

impl Render for crate::Element {
    fn render_to(&self, buffer: &mut String) {
        write!(buffer, "{self}").unwrap();
    }
}

impl Render for crate::Document {
    fn render_to(&self, buffer: &mut String) {
        write!(buffer, "{self}").unwrap()
    }
}

impl From<Markup> for crate::Element {
    fn from(PreEscaped(s): Markup) -> Self {
        crate::elt::raw_unsafe(s)
    }
}

impl From<crate::Element> for Markup {
    fn from(value: crate::Element) -> Self {
        PreEscaped(value.to_string())
    }
}

impl From<crate::Document> for Markup {
    fn from(value: crate::Document) -> Self {
        PreEscaped(value.to_string())
    }
}
