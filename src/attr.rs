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

/// Do not render any attribute. Useful for conditional rendering.
///
/// # Example
///
/// ```
/// use fun_html::{attr, elt};
///
/// let name: Option<&str> = None;
/// let element = elt::div([
///   // Note, `unwrap_or_default()` would have the same effect here
///   name.map(attr::name).unwrap_or(attr::none()),
/// ], []);
/// ```
pub fn none() -> Attribute {
    Attribute(crate::AttributeInner::None)
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
pub fn href(value: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("href", value)
}

/// `rel` attribute
pub fn rel(value: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("rel", value)
}

/// `src` attribute
pub fn src(value: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("src", value)
}

/// `alt` attribute
pub fn alt(value: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("alt", value)
}

/// `width` attribute
pub fn width(value: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("width", value)
}

/// `height` attribute
pub fn height(value: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("height", value)
}

/// `width` attribute with an `i32` value
pub fn width_int(value: i32) -> Attribute {
    Attribute::new_int("width", value)
}

/// `height` attribute with an `i32` value
pub fn height_int(value: i32) -> Attribute {
    Attribute::new_int("height", value)
}

/// `style` attribute
pub fn style(value: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("style", value)
}

/// `cols` attribute
pub fn cols(value: i32) -> Attribute {
    Attribute::new_int("cols", value)
}

/// `rows` attribute
pub fn rows(value: i32) -> Attribute {
    Attribute::new_int("rows", value)
}

/// `type` attribute
pub fn type_(value: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("type", value)
}

/// `type="text"` (text input)
pub fn type_text() -> Attribute {
    Attribute::new("type", "text")
}

/// `type="password"` (password input)
pub fn type_password() -> Attribute {
    Attribute::new("type", "password")
}

/// `type="number"` (number input)
pub fn type_number() -> Attribute {
    Attribute::new("type", "number")
}

/// `type="tel"` (phone number input)
pub fn type_tel() -> Attribute {
    Attribute::new("type", "tel")
}

/// `type="file"` (file input)
pub fn type_file() -> Attribute {
    Attribute::new("type", "file")
}

/// `type="checkbox"` (checkbox input)
pub fn type_checkbox() -> Attribute {
    Attribute::new("type", "checkbox")
}

/// `type="radio"` (radio input)
pub fn type_radio() -> Attribute {
    Attribute::new("type", "radio")
}

/// `type="range"` (range input)
pub fn type_range() -> Attribute {
    Attribute::new("type", "range")
}

/// `type="email"` (email input)
pub fn type_email() -> Attribute {
    Attribute::new("type", "email")
}

/// `type="date"` (date input)
pub fn type_date() -> Attribute {
    Attribute::new("type", "date")
}

/// `type="month"` (month input)
pub fn type_month() -> Attribute {
    Attribute::new("type", "month")
}

/// `type="hidden"` (hidden input)
pub fn type_hidden() -> Attribute {
    Attribute::new("type", "hidden")
}

/// `type="reset"` (reset button)
pub fn type_reset() -> Attribute {
    Attribute::new("type", "reset")
}

/// `type="submit"` (reset button)
pub fn type_submit() -> Attribute {
    Attribute::new("type", "submit")
}

/// `integrity` attribute
pub fn integrity(value: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("integrity", value)
}

/// `defer` attribute
pub fn defer() -> Attribute {
    Attribute::new_flag("defer")
}

/// `async` attribute
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
#[deprecated(since = "1.5.0", note = "renamed to 'meta_charset_utf8'")]
pub fn charset_utf_8() -> Attribute {
    charset("UTF-8")
}

/// Alias for `charset("UTF-8")`
pub fn charset_utf8() -> Attribute {
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

/// `for` attribute
pub fn for_(value: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("for", value)
}

/// `value` attribute
pub fn value(value: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("value", value)
}

/// `required` attribute
pub fn required() -> Attribute {
    Attribute::new_flag("required")
}
/// `autofocus` attribute
pub fn autofocus() -> Attribute {
    Attribute::new_flag("autofocus")
}

/// `autocomplete` attribute
pub fn autocomplete(type_: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("autocomplete", type_)
}

/// `autocomplete="on"` attribute
pub fn autocomplete_on() -> Attribute {
    Attribute::new("autocomplete", "on")
}

/// `autocomplete="on"` attribute
pub fn autocomplete_off() -> Attribute {
    Attribute::new("autocomplete", "off")
}

/// `disabled` attribute
pub fn disabled() -> Attribute {
    Attribute::new_flag("disabled")
}

/// `pattern` attribute
pub fn pattern(value: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("pattern", value)
}

/// `min` attribute
pub fn min(value: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("min", value)
}

/// `max` attribute
pub fn max(value: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("max", value)
}

/// `minlength` attribute
pub fn minlength(value: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("minlength", value)
}

/// `minlength` attribute using a `u16` value
pub fn minlength_u16(value: u16) -> Attribute {
    Attribute::new_int("minlength", value.into())
}

/// `maxlength` attribute
pub fn maxlength(value: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("maxlength", value)
}

/// `maxlength` attribute using a `u16` value
pub fn maxlength_u16(value: u16) -> Attribute {
    Attribute::new_int("maxlength", value.into())
}

/// `multiple` attribute
pub fn multiple() -> Attribute {
    Attribute::new_flag("multiple")
}

/// `placeholder` attribute
pub fn placeholder(value: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("placeholder", value)
}
