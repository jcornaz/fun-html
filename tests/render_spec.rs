use fun_html::{
    attributes::class,
    nodes::{h1, text},
    Document,
};

#[test]
fn should_render_empty_document() {
    let doc = Document::default();
    let string = format!("{doc}");
    assert_eq!(
        string,
        "<!DOCTYPE html>\n<html><head></head><body></body></html>"
    )
}

#[test]
fn should_render_attributes() {
    let node = h1([class(["underlined", "blue"])], [text("Hello world!")]);
    let string = format!("{node}");
    assert_eq!(string, "<h1 class=\"underlined blue\">Hello world!</h1>")
}
