use alloc::{borrow::Cow, string::String};

use crate::Attribute;

pub enum Target {
    Blank,
    Self_,
    Parent,
    Top,
    Frame(Cow<'static, str>),
}

pub fn target(target: Target) -> Attribute {
    Attribute::new(
        "target",
        match target {
            Target::Blank => "_blank".into(),
            Target::Self_ => "_self".into(),
            Target::Parent => "_parent".into(),
            Target::Top => "_top".into(),
            Target::Frame(name) => name,
        },
    )
}

/// Alias for `target(Target::Blank)`
pub fn target_blank() -> Attribute {
    target(Target::Blank)
}

impl<T: Into<Cow<'static, str>>> From<(&'static str, T)> for Attribute {
    fn from((key, value): (&'static str, T)) -> Self {
        Attribute::new(key, value)
    }
}

pub fn lang(lang: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("lang", lang)
}

pub fn id(id: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("id", id)
}

pub fn href(url: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("href", url)
}

pub fn download() -> Attribute {
    Attribute::new_flag("download")
}

pub fn download_with_name(name: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("download", name)
}

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
