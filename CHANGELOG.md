# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).


## [Unreleased]

### Deprecated

* `attr::charset_utf_8()` and `elt::meta_charset_utf_8()`
  (renamed to `attr::charset_utf8()` and `elt::meta_charset_utf8()`)

### Added

* `attr::none()` for conditional rendering of an attribute
* implement `Default` for `Element` and `Attribute`
* `attr::disabled()` and `attr::autofocus()`
* `attr::charset_utf8()` and `elt::meta_charset_utf8()`
* `attr::autocomplete(type_: impl Into<Cow<'static, str>>)`, `attr::autocomplete_on()`, `attr::autocomplete_off()`


## [1.4.0] - 2024-12-13

### Added

* `salvo_v074` feature flag providing implementation of `salvo::Scribe` for `Document` and `Element`


## [1.3.0] - 2024-11-05

### Added

* `meta_charset_utf_8`, `meta_color_scheme` and `link_stylesheet`


## [1.2.1] - 2024-11-05

### Documentation

Minor fix in root crate documentation


## [1.2.0] - 2024-11-03

### Added

* `Attribute::new_unsafe_name` escape hatch to allow using an attribute name generated at runtime.


## [1.1.0] - 2024-11-03

### Added 

* `textarea` and related attributes (`cols` and `rows`)
* `width_int` and `height_int` which takes an `i32` value instead of a string


## [1.0.0] - 2024-11-03

### Added 

* `Element`, `Attribute` and `Document` types
* `html` function to create an html document
* Attribute values and text elements are escaped for HTML
* Attributes
  * `lang`
  * `id`
  * `class`
  * `name`
  * `src`
* Elements
  * Html document (`html`, `head` and `body`)
  * Meta (`meta`, `meta_viewport`, `link`, `script`, `title`) and related attributes
    (`charset`, `content`, `rel`, `async`, `defer`, `integrity`, `type`, `src`, and `crossorigin`)
  * Text (`h1` to `h6`, `p`, `span`, `small`, `br` and `text`)
  * Container (`div`, `article`, `section`, `header`, `main`, `footer`)
  * Table (`table`, `thead`, `tbody`, `tfoot`, `tr`, `th`, `td`)
  * Anchor (`a`) and related attributes (`href`, `target` and `download`)
  * Image (`img`) and related attributes (`src`, `alt`, `width` and `height`)
  * Lists (`ul`, `ol` and `li`)
  * Forms (`form`, `fieldset`, `input`, `label`, `select`, `option`) and related attributes
    (`action`, `method`, `placeholder`, `for`, `value`, `min`, `max`, `minlength`, `maxlength`, `multiple`, `type_*`)
  * Escape hatches (`raw` and `raw_unsafe`)
* implement `From<(&'static str, Cow<'static, str)>` for `Attribute`
* implement `From<&'static str>` and `From<String>` for `Element`
* implement `From<[Element; N]>` and `From<Vec<Element>>` for `Element` (group elements without wrapping them in a `div`)
* Feature flags
  * `std`: Enabled by default. Disable it to compile to `no_std`
  * `rocket_v05` : implement [`rocket::response::Responder`](https://docs.rs/rocket/latest/rocket/response/trait.Responder.html) for `Document` and `Element` (for rocket 0.5)

[Unreleased]: https://github.com/jcornaz/fun-html/compare/v1.4.0...HEAD
[1.4.0]: https://github.com/jcornaz/fun-html/compare/v1.3.0...v1.4.0
[1.3.0]: https://github.com/jcornaz/fun-html/compare/v1.2.1...v1.3.0
[1.2.1]: https://github.com/jcornaz/fun-html/compare/v1.2.0...v1.2.1
[1.2.0]: https://github.com/jcornaz/fun-html/compare/v1.1.0...v1.2.0
[1.1.0]: https://github.com/jcornaz/fun-html/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/jcornaz/fun-html/compare/...v1.0.0
