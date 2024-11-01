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
  * `name`
* Elements
  * Html document (`html`, `head` and `body`)
  * Meta (`meta`, `meta_viewport`, `title`) and related attributes (`charset`, `content`)
  * Text (`h1` to `h6`, and `text`)
  * Container (`div`)
  * Anchor (`a`) and related attributes (`href`, `target` and `download`)
  * Escape hatches (`raw` and `raw_unsafe`)
* implement `From<(&'static str, Cow<'static, str)>` for `Attribute`
* implement `From<&'static str>` and `From<String>` for `Element`
* implement `From<[Element; N]>` and `From<Vec<Element>>` for `Element` (group elements without wrapping them in a `div`)
* Support for `no_std` (by disabling the `std` feature flag)
* Feature flags
  * `std`: Enabled by default. Disable it to compile to `no_std`
  * `rocket_v05` : implement [`rocket::response::Responder`](https://docs.rs/rocket/latest/rocket/response/trait.Responder.html) for `Document` and `Element` (for rocket 0.5)

[Unreleased]: https://github.com/jcornaz/fun-html/compare/...HEAD
