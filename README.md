# fun-html

[![Crates.io Version](https://img.shields.io/crates/v/fun-html)](https://crates.io/crates/fun-html)
[![License](https://img.shields.io/github/license/jcornaz/fun-html)](./LICENSE)
![MSRV](https://img.shields.io/crates/msrv/fun-html)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jcornaz/fun-html/.github%2Fworkflows%2Fcheck.yml?branch=main)](https://github.com/jcornaz/fun-html/actions/workflows/check.yml?query=branch%3Amain)
[![docs.rs](https://img.shields.io/docsrs/fun-html)](https://docs.rs/fun-html)


This rust library provides a simple and efficient way to generate HTML using Rust functions,
with an intuitive and composable API to create HTML elements.

```rust
use fun_html::{attr, elt};

let greeting = elt::h1(
  [attr::class(["bold"])], // <-- attributes
  ["Hello world!".into()], // <-- children
);
assert_eq!(greeting.to_string(), r#"<h1 class="bold">Hello world!</h1>"#);
```

Because those are simple rust functions, it is easy to leverage rust features like conditions, loops and iterators:

```rust
use fun_html::{elt, Element};

fn list_view(items: Vec<i32>) -> Element {
  if !items.is_empty(){
    elt::ul([], items.into_iter().map(|item| elt::li([], [item.to_string().into()])))
  } else {
    elt::text("no item")
  }
};

assert_eq!(
  list_view((1..=3).collect()).to_string(),
  "<ul><li>1</li><li>2</li><li>3</li></ul>"
);
```

> [!NOTE]
>
> The crate [fun-htmx](https://github.com/jcornaz/fun-htmx) provide additional attributes
> for use with [HTMX](https://htmx.org)

## Feature flags

* `std`: enabled by default. must be disabled to compile to `no_std`
* `rocket_v05`: implements the [`Responder` trait from rocket 0.5](https://docs.rs/rocket/0.5/rocket/response/trait.Responder.html) for `Document` and `Element`
* `salvo_v074`: implements the [`Scribe` trait from salvo 0.74](https://docs.rs/salvo/0.74/salvo/trait.Scribe.html) for `Document` and `Element`


## MSRV

The minimum supported rust version is currently `1.60`.

It will be updated when required, and that will not be considered a breaking change (it can happen in a minor version).


## MIT License

Copyright (c) 2024 Jonathan Cornaz

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
