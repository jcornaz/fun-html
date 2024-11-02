# fun-html

![rustc](https://img.shields.io/badge/rustc-1.60+-blue?logo=rust)

This rust library provides a simple and efficient way to generate HTML using Rust functions,
with an intuitive and composable API to create HTML elements.

```rust
use fun_html::{attr::class, elt::h1};

let greeting = h1(
  [class(["bold"])], // <-- attributes
  ["Hello world!".into()], // <-- children
);
assert_eq!(greeting.to_string(), r#"<h1 class="bold">Hello world!</h1>"#);
```

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
