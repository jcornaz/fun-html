# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).


## [Unreleased]

### Added 

* Escape HTML strings
* `id` and `class` attributes
* Nodes
  * Html document (`html`, `head` and `body`)
  * Meta (`title`)
  * Text (`h1`, `h2`, `h3`, `h4`, `h5`, `h6`, and `text`)
  * Container (`div`)
  * Escape hatches (`raw and `raw_unsafe`)
* `Node`, `Attribute` and `Document` types
* implement `From<(&'static str, Cow<'static, str)>` for `Attribute`
* implement `From<&'static str>` and `From<String>` for `Node`
* implement `From<[Node; N]>` and `From<Vec<Node>>` for `Node` (group nodes without wrapping them in a `div`)

[Unreleased]: https://github.com/jcornaz/fun-html/compare/...HEAD
