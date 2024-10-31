use std::borrow::Cow;

use crate::Attribute;

impl<T: Into<Cow<'static, str>>> From<(&'static str, T)> for Attribute {
    fn from((key, value): (&'static str, T)) -> Self {
        Attribute::new(key, value)
    }
}

pub fn lang(lang: impl Into<Cow<'static, str>>) -> Attribute {
    ("lang", lang).into()
}

pub fn id(id: impl Into<Cow<'static, str>>) -> Attribute {
    ("id", id).into()
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
