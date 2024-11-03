//! Common attributes
//!
//! Note that you may create your own attribute by using [`Attribute::new`] or [`Attribute::new_flag`]
//! Or by leveraging on of the `From` implementation on [`Attribute`]

use alloc::{borrow::Cow, string::String};

use crate::Attribute;

impl<T: Into<Cow<'static, str>>> From<(&'static str, T)> for Attribute {
    fn from((key, value): (&'static str, T)) -> Self {
        Attribute::new(key, value)
    }
}

/// `id` attribute
pub fn id(id: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("id", id)
}

/// `class` attribute
///
/// It takes a list of clases and join them together
///
/// ## Example
///
/// ```
/// # use fun_html::attr::class;
///
/// assert_eq!(
///   class(["foo", "bar", "baz"]).to_string(),
///   r#"class="foo bar baz""#,
/// );
/// ```
pub fn class<'a>(classes: impl IntoIterator<Item = &'a str>) -> Attribute {
    let mut values = String::new();
    let mut iter = classes.into_iter();
    if let Some(value) = iter.next() {
        values.push_str(value);
    }
    for value in iter {
        values.push(' ');
        values.push_str(value);
    }
    Attribute::new("class", values)
}

/// `lang` attribute (usually on `html` element)
pub fn lang(lang: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("lang", lang)
}

/// Represent an anchor target
#[derive(Debug, Clone)]
pub enum AnchorTarget {
    /// `_blank`
    Blank,
    /// `_self`
    Self_,
    /// `_parent`
    Parent,
    /// `_top`
    Top,
    /// Frame name
    Frame(Cow<'static, str>),
}

/// `target` attribute for `<a>`
pub fn target(target: AnchorTarget) -> Attribute {
    Attribute::new(
        "target",
        match target {
            AnchorTarget::Blank => "_blank".into(),
            AnchorTarget::Self_ => "_self".into(),
            AnchorTarget::Parent => "_parent".into(),
            AnchorTarget::Top => "_top".into(),
            AnchorTarget::Frame(name) => name,
        },
    )
}

/// Alias for `target(Target::Blank)`
pub fn target_blank() -> Attribute {
    target(AnchorTarget::Blank)
}

/// `href` attribute
pub fn href(url: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("href", url)
}

/// `rel` atribute
pub fn rel(url: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("rel", url)
}

/// `src` atribute
pub fn src(url: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("src", url)
}

/// `alt` atribute
pub fn alt(url: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("alt", url)
}

/// `width` atribute
pub fn width(url: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("width", url)
}

/// `height` atribute
pub fn height(url: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("height", url)
}

/// `type` atribute
pub fn type_(url: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("type", url)
}

/// `integrity` atribute
pub fn integrity(url: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("integrity", url)
}

/// `defer` atribute
pub fn defer() -> Attribute {
    Attribute::new_flag("defer")
}

/// `async` atribute
pub fn async_() -> Attribute {
    Attribute::new_flag("async")
}

/// `crossorigin="anonymous"`
pub fn crossorigin_anonymous() -> Attribute {
    Attribute::new("crossorigin", "anonymous")
}

/// `crossorigin="use-credentials"`
pub fn crossorigin_use_credentials() -> Attribute {
    Attribute::new("crossorigin", "use-credentials")
}

/// `download` flag attribute
pub fn download() -> Attribute {
    Attribute::new_flag("download")
}

/// `download` attribute with a file name argument
pub fn download_with_name(name: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("download", name)
}

/// `charset` attribute
pub fn charset(charset: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("charset", charset)
}

/// Alias for `charset("UTF-8")`
pub fn charset_utf_8() -> Attribute {
    charset("UTF-8")
}

/// `name` attribute
pub fn name(name: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("name", name)
}

/// `content` attribute
pub fn content(content: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("content", content)
}

/// `action` attribute
pub fn action(action: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("action", action)
}

/// `method_get` attribute
pub fn method_get() -> Attribute {
    Attribute::new("method", "get")
}

/// `method_get` attribute
pub fn method_post() -> Attribute {
    Attribute::new("method", "post")
}
