#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

//! This crate provides a simple and efficient way to generate HTML using Rust functions,
//! with an intuitive and composable API to create HTML elements.
//!
//! The [`elt`] module contains functions to create common HTML elements. Most of them take 2 arguments:
//! The list of attributes, and the list of children.
//! For example, `div([id("mydiv")], ["hello".into()])` creates a `<div>` with an ID of "mydiv" containing the text "hello".
//!
//! The [`attr`] module contains functions to create common attributes.
//!
//! Text can be inserted by using the [`elt::text`] function or by using one of the `Into<Element>` implementations.//! ```
//! All text and attribute values are automatically escaped to ensure safe and valid HTML output.
//!
//! ```
//! use fun_html::{attr::class, elt::h1};
//!
//! let greeting = h1(
//!   [class(["bold"])], // <-- attributes
//!   ["Hello world!".into()], // <-- children
//! );
//! assert_eq!(greeting.to_string(), r#"<h1 class="bold">Hello world!</h1>"#);
//! ```
//!
//! Because those are simple rust functions, it is easy to leverage rust features like conditions, loops and iterators:
//!
//! ```
//! # use fun_html::{elt::{li,ul,text}};
//!
//! let list_values = true;
//! let element = if list_values {
//!   ul([], (1..=3).map(|n| li([], [n.to_string().into()])))
//! } else {
//!   text("no value")
//! };
//!
//! assert_eq!(element.to_string(), "<ul><li>1</li><li>2</li><li>3</li></ul>")
//! ```
//!
//! ## Escape hatches
//!
//! If necessary, it is possible to define custom elements and attributes with respectively [`Element::new`] and [`Attribute::new`].
//!
//! It is also possible to declare custom attributes on the fly like this: `div([("hx-get", "/values").into()], [])`)
//!
//! It is also possible to inline raw html with:
//! * [`elt::raw`]: inline HTML that is known at compile time, and is therefore considered safe.
//! * [`elt::raw_unsafe`]: can inline HTML built at runtime, and is therefore unsafe.
//!
//!
//! ## Feature flags
//!
//! * `std`: enabled by default. must be disabled to compile to `no_std`
//! * `rocket_v05`: implements the `Responder` from [rocket](https://rocket.rs) v0.5 for [`Document`] and [`Element`]

pub mod attr;
pub mod elt;

mod interop {
    #[cfg(feature = "rocket_v05")]
    mod rocket_v05;
}

extern crate alloc;

use alloc::{borrow::Cow, fmt::Display, vec::Vec};

/// An HTML document (`<!DOCTYPE html>`)
///
/// ## Example
///
/// ```
/// # use fun_html::{Document, html, elt::{head, body}};
/// let doc: Document = html([], [head([], []), body([], [])]);
///
/// assert_eq!(doc.to_string(), "<!DOCTYPE html>\n<html><head></head><body></body></html>");
/// ```
#[derive(Debug, Clone)]
pub struct Document(Element);

/// An HTML element
///
/// It can be created via [`Self::new`], [`Self::new_void`]
///
/// ## Example
///
/// ```
/// # use fun_html::{Element, elt::div};
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
    None,
}

/// An attribute
///
/// It can be created via [`Self::new`], [`Self::new_flag`]
/// or by converting from ither `(&'static str, &'static str)` or `(&'static str, String)`.
///
/// See [`attr`] for a collection of common attributes
///
/// ## Example
///
/// ```
/// # use fun_html::attr::id;
/// assert_eq!(
///   id("foo").to_string(),
///   r#"id="foo""#,
/// )
/// ```    
#[derive(Debug, Clone)]
pub struct Attribute {
    name: &'static str,
    value: Option<Cow<'static, str>>,
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
    /// ("void" element cannot have children and do not need a closing tag)
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
            ElementInner::None => (),
        }
        Ok(())
    }
}

fn write_attributes(
    f: &mut alloc::fmt::Formatter<'_>,
    attributes: &Vec<Attribute>,
) -> Result<(), alloc::fmt::Error> {
    for attribute in attributes {
        write!(f, " {attribute}")?;
    }
    Ok(())
}

impl Display for Attribute {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(value) = self.value.as_ref() {
            write!(
                f,
                "=\"{}\"",
                html_escape::encode_double_quoted_attribute(value)
            )?;
        }
        Ok(())
    }
}

impl Attribute {
    /// Create a new attribute
    pub fn new(name: &'static str, value: impl Into<Cow<'static, str>>) -> Self {
        assert_valid_attribute_name(name);
        Self {
            name,
            value: Some(value.into()),
        }
    }

    /// Create a new flag attribute (that doesn't take a value)
    pub fn new_flag(name: &'static str) -> Self {
        assert_valid_attribute_name(name);
        Self { name, value: None }
    }
}

fn assert_valid_attribute_name(name: &str) {
    debug_assert!(
        !name.is_empty() && name.chars().all(|c| !c.is_whitespace()),
        "invalid attribute name: '{name}'"
    );
}

/// Create an HTML [`Document`]
///
/// You must pass the [`elt::head`] and [`elt::body`] element as you would with any other element.
///
/// ## Example
///
/// ```
/// # use fun_html::{html, elt::{head, body, title, h1}, attr::{lang}};
/// let document = html([lang("en")], [
///     head([], [title([], "Greetings")]),
///     body([], [h1([], ["Hello world!".into()])]),
/// ]);
/// ```
pub fn html(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Document {
    Document(Element::new("html", attributes, children))
}
