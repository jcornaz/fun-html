use std::borrow::Cow;

use crate::Attribute;

pub fn attr(key: &'static str, value: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new(key, value)
}

pub fn id(id: impl Into<Cow<'static, str>>) -> Attribute {
    attr("id", id)
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
