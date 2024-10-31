use std::borrow::Cow;

use crate::{Attribute, Node, NodeInner};

pub fn h1(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Node>,
) -> Node {
    Node::new("h1", attributes, children)
}

pub fn text(value: impl Into<Cow<'static, str>>) -> Node {
    NodeInner::Text(value.into()).into()
}
