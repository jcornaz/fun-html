#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! This crate provides a simple and efficient way to generate HTML using Rust functions,
//! with an intuitive and composable API to create HTML elements.
//!
//! The [`elements`] module contains functions to create common HTML elements. Most of them take 2 arguments:
//! The list of attributes, and the list of children.
//! For example, `div([id("mydiv")], ["hello".into()])` creates a `<div>` with an ID of "mydiv" containing the text "hello".
//!
//! The [`attributes`] module contains functions to create common attributes.
//!
//! ```
//! use fun_html::{attributes::class, elements::h1};
//!
//! let greeting = h1(
//!   [class(["bold"])], // <-- First argument is the attributes
//!   ["Hello world!".into()], // <-- Second argument is the children
//! );
//! assert_eq!(greeting.to_string(), "<h1 class=\"bold\">Hello world!</h1>");
//! ```
//!
//! ## Safety
//!
//! Text can be inserted by using the `text` function or by using one of the `Into<Element>` implementations.
//! All text and attribute values are automatically escaped to ensure safe and valid HTML output.
//!
//! ## Escape hatches
//!
//! If necessary, it is possible to define custom elements and attributes with respectively [`Element::new`] and [`Attribute::new`].
//!
//! It is also possible to inline raw html with:
//! * [`elements::raw`]: inline HTML that is known at compile time, and is therefore considered safe.
//! * [`elements::raw_unsafe`]: can inline HTML built at runtime, and is therefore unsafe.
//!
//! ## Feature flags
//!
//! * `std`: enabled by default. must be disabled to compile to `no_std`
//! * `rocket_v05`: implements the `Responder` from [rocket](https://rocket.rs) v0.5 for [`Document`] and [`Element`]

pub mod attributes;
pub mod elements;

mod interop {
    #[cfg(feature = "rocket_v05")]
    mod rocket_v05;
}

extern crate alloc;

use alloc::{borrow::Cow, fmt::Display, vec::Vec};

/// An HTML document (`<!DOCTYPE html>`)
///
/// ```
/// use fun_html::{Document, html, elements::{head, body}};
/// let doc: Document = html([], [head([], []), body([], [])]);
///
/// assert_eq!(doc.to_string(), "<!DOCTYPE html>\n<html><head></head><body></body></html>");
/// ```
#[derive(Debug, Clone)]
pub struct Document(Element);

/// An HTML element
///
/// ```
/// use fun_html::{Element, elements::div};
/// let element: Element = div([], []);
///
/// assert_eq!(element.to_string(), "<div></div>");
/// ```    
#[derive(Debug, Clone)]
pub struct Element(ElementInner);

#[derive(Debug, Clone)]
enum ElementInner {
    Parent {
        tag: &'static str,
        attributes: Vec<Attribute>,
        children: Vec<Element>,
    },
    Void {
        tag: &'static str,
        attributes: Vec<Attribute>,
    },
    Text(Cow<'static, str>),
    Script(Cow<'static, str>),
    Raw(Cow<'static, str>),
    Multiple(Vec<Element>),
}

#[derive(Debug, Clone)]
pub struct Attribute {
    name: &'static str,
    value: AttributeValue,
}

#[derive(Debug, Clone)]
enum AttributeValue {
    String(Cow<'static, str>),
    Flag,
}

impl Default for Document {
    fn default() -> Self {
        Self(Element::new(
            "html",
            [],
            [Element::new("head", [], []), Element::new("body", [], [])],
        ))
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
        write!(f, "<!DOCTYPE html>\n{}", self.0)
    }
}

impl Element {
    /// Create a new HTML element from its tag, attributes, and children
    pub fn new(
        tag: &'static str,
        attributes: impl IntoIterator<Item = Attribute>,
        children: impl IntoIterator<Item = Element>,
    ) -> Self {
        assert_valid_tag_name(tag);
        Self(ElementInner::Parent {
            tag,
            attributes: attributes.into_iter().map(Into::into).collect(),
            children: children.into_iter().collect(),
        })
    }

    /// Create a new [void] HTML element from its tag and attributes
    ///
    /// [void]: https://developer.mozilla.org/en-US/docs/Glossary/Void_element
    pub fn new_void(tag: &'static str, attributes: impl IntoIterator<Item = Attribute>) -> Self {
        assert_valid_tag_name(tag);
        Self(ElementInner::Void {
            tag,
            attributes: attributes.into_iter().collect(),
        })
    }
}

fn assert_valid_tag_name(tag: &str) {
    debug_assert!(
        !tag.is_empty() && tag.chars().all(|c| !c.is_whitespace()),
        "invalid tag name: '{tag}'"
    );
}

impl From<ElementInner> for Element {
    fn from(value: ElementInner) -> Self {
        Self(value)
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
        match &self.0 {
            ElementInner::Parent {
                tag,
                attributes,
                children,
            } => {
                write!(f, "<{tag}")?;
                write_attributes(f, attributes)?;
                write!(f, ">")?;
                for child in children {
                    write!(f, "{child}")?;
                }
                write!(f, "</{tag}>")?;
            }
            ElementInner::Void { tag, attributes } => {
                write!(f, "<{tag}")?;
                write_attributes(f, attributes)?;
                write!(f, ">")?;
            }
            ElementInner::Text(text) => write!(f, "{}", html_escape::encode_text(text))?,
            ElementInner::Script(text) => write!(f, "{}", html_escape::encode_script(text))?,
            ElementInner::Raw(raw) => write!(f, "{raw}")?,
            ElementInner::Multiple(elems) => {
                for elt in elems {
                    write!(f, "{elt}")?;
                }
            }
        }
        Ok(())
    }
}

fn write_attributes(
    f: &mut alloc::fmt::Formatter<'_>,
    attributes: &Vec<Attribute>,
) -> Result<(), alloc::fmt::Error> {
    for attribute in attributes {
        match &attribute.value {
            AttributeValue::String(s) => write!(
                f,
                " {}=\"{}\"",
                attribute.name,
                html_escape::encode_double_quoted_attribute(s)
            )?,
            AttributeValue::Flag => write!(f, " {}", attribute.name,)?,
        }
    }
    Ok(())
}

impl Attribute {
    pub fn new(name: &'static str, value: impl Into<Cow<'static, str>>) -> Self {
        assert_valid_attribute_name(name);
        Self {
            name,
            value: AttributeValue::String(value.into()),
        }
    }

    pub fn new_flag(name: &'static str) -> Self {
        assert_valid_attribute_name(name);
        Self {
            name,
            value: AttributeValue::Flag,
        }
    }
}

fn assert_valid_attribute_name(name: &str) {
    debug_assert!(
        !name.is_empty() && name.chars().all(|c| !c.is_whitespace()),
        "invalid attribute name: '{name}'"
    );
}

pub fn html(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Document {
    Document(Element::new("html", attributes, children))
}
