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
    children: impl IntoIterator<Item = Node>,
) -> Node {
    Node::new("title", attributes, children)
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

pub fn text(value: impl Into<Cow<'static, str>>) -> Node {
    NodeInner::Text(value.into()).into()
}

impl<T: Into<Cow<'static, str>>> From<T> for Node {
    fn from(value: T) -> Self {
        text(value)
    }
}
