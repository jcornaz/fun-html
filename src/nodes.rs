use std::borrow::Cow;

use crate::{Attribute, Node, NodeInner};

pub fn div(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Node>,
) -> Node {
    Node::new("div", attributes, children)
}

pub fn head(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Node>,
) -> Node {
    Node::new("head", attributes, children)
}

pub fn title(
    attributes: impl IntoIterator<Item = Attribute>,
    text: impl Into<Cow<'static, str>>,
) -> Node {
    Node::new("title", attributes, [text.into().into()])
}

pub fn body(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Node>,
) -> Node {
    Node::new("body", attributes, children)
}

pub fn h1(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Node>,
) -> Node {
    Node::new("h1", attributes, children)
}

pub fn h2(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Node>,
) -> Node {
    Node::new("h2", attributes, children)
}

pub fn h3(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Node>,
) -> Node {
    Node::new("h3", attributes, children)
}

pub fn h4(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Node>,
) -> Node {
    Node::new("h4", attributes, children)
}

pub fn h5(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Node>,
) -> Node {
    Node::new("h5", attributes, children)
}

pub fn h6(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Node>,
) -> Node {
    Node::new("h6", attributes, children)
}

/// HTML escaped text
pub fn text(value: impl Into<Cow<'static, str>>) -> Node {
    NodeInner::Text(value.into()).into()
}

/// Inline raw HTML without escaping
///
/// This function is considered safe because the HTML being inlined must be known at compile time
///
/// See [`raw_unsafe`] to inline HTML that is generated at runtime
pub fn raw(html: &'static str) -> Node {
    NodeInner::Raw(html.into()).into()
}

/// Inline raw HTML without escaping
///
/// This function **IS NOT SAFE** and should be avoided unless really necessary.
/// Miss-use can lead to XSS vulnerability.
///
/// See [`raw`] to safely inline HTML that is known at compile time
pub fn raw_unsafe(html: String) -> Node {
    NodeInner::Raw(html.into()).into()
}

impl From<Cow<'static, str>> for Node {
    fn from(value: Cow<'static, str>) -> Self {
        text(value)
    }
}

impl From<&'static str> for Node {
    fn from(value: &'static str) -> Self {
        text(value)
    }
}

impl From<String> for Node {
    fn from(value: String) -> Self {
        text(value)
    }
}

impl<const N: usize> From<[Node; N]> for Node {
    fn from(value: [Node; N]) -> Self {
        Vec::from(value).into()
    }
}

impl From<Vec<Node>> for Node {
    fn from(value: Vec<Node>) -> Self {
        Self(NodeInner::Multiple(value))
    }
}
