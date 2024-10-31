#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod attributes;
pub mod nodes;

use std::{borrow::Cow, fmt::Display};

#[derive(Debug)]
pub struct Document(Node);

#[derive(Debug)]
pub struct Node(NodeInner);

#[derive(Debug)]
enum NodeInner {
    Node {
        tag: &'static str,
        attributes: Vec<Attribute>,
        children: Vec<Node>,
    },
    Text(Cow<'static, str>),
}

#[derive(Debug)]
pub struct Attribute {
    key: &'static str,
    value: AttributeValue,
}

#[derive(Debug)]
enum AttributeValue {
    String(Cow<'static, str>),
}

impl Default for Document {
    fn default() -> Self {
        Self(Node::new(
            "html",
            [],
            [Node::new("head", [], []), Node::new("body", [], [])],
        ))
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<!DOCTYPE html>\n{}", self.0)
    }
}

impl Node {
    pub fn new(
        tag: &'static str,
        attributes: impl IntoIterator<Item = Attribute>,
        children: impl IntoIterator<Item = Node>,
    ) -> Self {
        Self(NodeInner::Node {
            tag,
            attributes: attributes.into_iter().map(Into::into).collect(),
            children: children.into_iter().collect(),
        })
    }
}

impl From<NodeInner> for Node {
    fn from(value: NodeInner) -> Self {
        Self(value)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            NodeInner::Node {
                tag,
                attributes,
                children,
            } => {
                write!(f, "<{tag}")?;
                for attribute in attributes {
                    match &attribute.value {
                        AttributeValue::String(s) => write!(f, " {}=\"{s}\"", attribute.key)?,
                    }
                }
                write!(f, ">")?;
                for child in children {
                    write!(f, "{child}")?;
                }
                write!(f, "</{tag}>")?;
            }
            NodeInner::Text(text) => write!(f, "{text}")?,
        }
        Ok(())
    }
}

impl Attribute {
    pub fn new(key: &'static str, value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            key,
            value: AttributeValue::String(value.into()),
        }
    }
}
