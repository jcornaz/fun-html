//! Common HTML elements
//!
//! Note that you may create your own element with [`Element::new`] or [`Element::new_void`]
//!
//! It is also possible to inline raw HTML with [`raw`] and [`raw_unsafe`]

use alloc::{borrow::Cow, string::String, vec::Vec};

use crate::{Attribute, Element, ElementInner};

/// `<div>`
pub fn div(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("div", attributes, children)
}

/// `<head>`
pub fn head(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("head", attributes, children)
}

/// `<meta>`
pub fn meta(attributes: impl IntoIterator<Item = Attribute>) -> Element {
    Element::new_void("meta", attributes)
}

/// `<meta name="viewport" content="width=device-width, initial-scale=1.0">`
pub fn meta_viewport() -> Element {
    raw("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">")
}

/// `<link>`
pub fn link(attributes: impl IntoIterator<Item = Attribute>) -> Element {
    Element::new_void("link", attributes)
}

/// `<script>`
pub fn script(attributes: impl IntoIterator<Item = Attribute>, content: &'static str) -> Element {
    Element::new(
        "script",
        attributes,
        [Element(ElementInner::Script(content.into()))],
    )
}

/// `<script>`
pub fn script_empty(attributes: impl IntoIterator<Item = Attribute>) -> Element {
    Element::new("script", attributes, [])
}

/// `<title>`
pub fn title(
    attributes: impl IntoIterator<Item = Attribute>,
    text: impl Into<Cow<'static, str>>,
) -> Element {
    Element::new("title", attributes, [text.into().into()])
}

/// `<body>`
pub fn body(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("body", attributes, children)
}

/// `<h1>`
pub fn h1(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("h1", attributes, children)
}

/// `<h2>`
pub fn h2(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("h2", attributes, children)
}

/// `<h3>`
pub fn h3(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("h3", attributes, children)
}

/// `<h4>`
pub fn h4(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("h4", attributes, children)
}

/// `<h5>`
pub fn h5(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("h5", attributes, children)
}

/// `<h6>`
pub fn h6(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("h6", attributes, children)
}

/// `<p>`
pub fn p(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("p", attributes, children)
}

/// `<br>`
pub fn br(attributes: impl IntoIterator<Item = Attribute>) -> Element {
    Element::new_void("br", attributes)
}

/// `<small>`
pub fn small(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("small", attributes, children)
}

/// `<span>`
pub fn span(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("span", attributes, children)
}

/// `<table>`
pub fn table(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("table", attributes, children)
}

/// `<tr>`
pub fn tr(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("tr", attributes, children)
}

/// `<td>`
pub fn td(
    attdibutes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("td", attdibutes, children)
}

/// `<th>`
pub fn th(
    attdibutes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("th", attdibutes, children)
}

/// `<thead>`
pub fn thead(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("thead", attributes, children)
}

/// `<tbody>`
pub fn tbody(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("tbody", attributes, children)
}

/// `<tfoot>`
pub fn tfoot(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("tfoot", attributes, children)
}

/// `<section>`
pub fn section(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("section", attributes, children)
}

/// `<article>`
pub fn article(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("article", attributes, children)
}

/// `<header>`
pub fn header(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("header", attributes, children)
}

/// `<main>`
pub fn main(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("main", attributes, children)
}

/// `<footer>`
pub fn footer(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("footer", attributes, children)
}

/// `<a>`
pub fn a(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("a", attributes, children)
}

/// `<img>`
pub fn img(attributes: impl IntoIterator<Item = Attribute>) -> Element {
    Element::new_void("img", attributes)
}

/// `<ul>`
pub fn ul(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("ul", attributes, children)
}

/// `<ol>`
pub fn ol(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("ol", attributes, children)
}

/// `<li>`
pub fn li(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("li", attributes, children)
}

/// HTML escaped text
pub fn text(value: impl Into<Cow<'static, str>>) -> Element {
    ElementInner::Text(value.into()).into()
}

/// Inline raw HTML without escaping
///
/// This function is considered safe because the HTML being inlined must be known at compile time
///
/// See [`raw_unsafe`] to inline HTML that is generated at runtime
pub fn raw(html: &'static str) -> Element {
    ElementInner::Raw(html.into()).into()
}

/// Inline raw HTML without escaping
///
/// This function **IS NOT SAFE** and should be avoided unless really necessary.
/// Miss-use can lead to XSS vulnerability.
///
/// See [`raw`] to safely inline HTML that is known at compile time
pub fn raw_unsafe(html: String) -> Element {
    ElementInner::Raw(html.into()).into()
}

impl From<Cow<'static, str>> for Element {
    fn from(value: Cow<'static, str>) -> Self {
        text(value)
    }
}

impl From<&'static str> for Element {
    fn from(value: &'static str) -> Self {
        text(value)
    }
}

impl From<String> for Element {
    fn from(value: String) -> Self {
        text(value)
    }
}

impl<const N: usize> From<[Element; N]> for Element {
    fn from(value: [Element; N]) -> Self {
        Vec::from(value).into()
    }
}

impl From<Vec<Element>> for Element {
    fn from(value: Vec<Element>) -> Self {
        Self(ElementInner::Multiple(value))
    }
}