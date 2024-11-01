# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).


## [Unreleased]

### Added 

* `Node`, `Attribute` and `Document` types
* `html` function to create an html document
* Attribute values and text nodes are escaped for HTML
* `lang`, `id` and `class` attributes
* Nodes
  * Html document (`html`, `head` and `body`)
  * Meta (`title`)
  * Text (`h1` to `h6`, and `text`)
  * Container (`div`)
  * Escape hatches (`raw` and `raw_unsafe`)
* implement `From<(&'static str, Cow<'static, str)>` for `Attribute`
* implement `From<&'static str>` and `From<String>` for `Node`
* implement `From<[Node; N]>` and `From<Vec<Node>>` for `Node` (group nodes without wrapping them in a `div`)

[Unreleased]: https://github.com/jcornaz/fun-html/compare/...HEAD
