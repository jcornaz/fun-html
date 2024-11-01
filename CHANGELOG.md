# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).


## [Unreleased]

### Added 

* `Element`, `Attribute` and `Document` types
* `html` function to create an html document
* Attribute values and text elements are escaped for HTML
* Attributes
  * `lang`
  * `id`
  * `class`
* Elements
  * Html document (`html`, `head` and `body`)
  * Meta (`meta`, `title`)
  * Text (`h1` to `h6`, and `text`)
  * Container (`div`)
  * Anchor (`a`) and related attributes (`href`, `download`)
  * Escape hatches (`raw` and `raw_unsafe`)
* implement `From<(&'static str, Cow<'static, str)>` for `Attribute`
* implement `From<&'static str>` and `From<String>` for `Element`
* implement `From<[Element; N]>` and `From<Vec<Element>>` for `Element` (group elements without wrapping them in a `div`)

[Unreleased]: https://github.com/jcornaz/fun-html/compare/...HEAD
