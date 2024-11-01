use rstest::rstest;

use fun_html::{Attribute, Element};

#[rstest]
#[cfg(debug_assertions)]
#[should_panic]
fn should_panic_for_invalid_attribute_name(
    #[values("hello world", "hello\tworld", "hello\nworld", "")] name: &'static str,
) {
    Attribute::new(name, "value");
}

#[rstest]
#[cfg(debug_assertions)]
#[should_panic]
fn should_panic_for_invalid_tag_name(
    #[values("hello world", "hello\tworld", "hello\nworld", "")] name: &'static str,
) {
    Element::new(name, [], []);
}

#[rstest]
#[cfg(debug_assertions)]
#[should_panic]
fn should_panic_for_invalid_void_element_name(
    #[values("hello world", "hello\tworld", "hello\nworld", "")] name: &'static str,
) {
    Element::new_void(name, []);
}
