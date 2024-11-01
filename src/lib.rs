#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod attributes;
pub mod nodes;

use std::{borrow::Cow, fmt::Display};

#[derive(Debug, Clone)]
pub struct Document(Node);

#[derive(Debug, Clone)]
pub struct Node(NodeInner);

#[derive(Debug, Clone)]
enum NodeInner {
    Node {
        tag: &'static str,
        attributes: Vec<Attribute>,
        children: Vec<Node>,
    },
    Text(Cow<'static, str>),
    Raw(Cow<'static, str>),
    Multiple(Vec<Node>),
}

#[derive(Debug, Clone)]
pub struct Attribute {
    name: &'static str,
    value: AttributeValue,
}

#[derive(Debug, Clone)]
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
        debug_assert!(
            !tag.is_empty() && tag.chars().all(|c| !c.is_whitespace()),
            "invalid attribute name: '{tag}'"
        );
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
                        AttributeValue::String(s) => write!(
                            f,
                            " {}=\"{}\"",
                            attribute.name,
                            html_escape::encode_double_quoted_attribute(s)
                        )?,
                    }
                }
                write!(f, ">")?;
                for child in children {
                    write!(f, "{child}")?;
                }
                write!(f, "</{tag}>")?;
            }
            NodeInner::Text(text) => write!(f, "{}", html_escape::encode_text(text))?,
            NodeInner::Raw(raw) => write!(f, "{raw}")?,
            NodeInner::Multiple(nodes) => {
                for node in nodes {
                    write!(f, "{node}")?;
                }
            }
        }
        Ok(())
    }
}

impl Attribute {
    pub fn new(name: &'static str, value: impl Into<Cow<'static, str>>) -> Self {
        debug_assert!(
            !name.is_empty() && name.chars().all(|c| !c.is_whitespace()),
            "invalid attribute name: '{name}'"
        );
        Self {
            name,
            value: AttributeValue::String(value.into()),
        }
    }
}

pub fn html(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Node>,
) -> Document {
    Document(Node::new("html", attributes, children))
}
