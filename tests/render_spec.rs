use rstest::rstest;

use fun_html::prelude::*;

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

#[rstest]
#[case(head([id("foo")], [text("hello")]), "<head id=\"foo\">hello</head>")]
#[case(body([id("foo")], [text("hello")]), "<body id=\"foo\">hello</body>")]
#[case(h1([id("foo")], [text("hello")]), "<h1 id=\"foo\">hello</h1>")]
#[case(h2([id("foo")], [text("hello")]), "<h2 id=\"foo\">hello</h2>")]
#[case(h3([id("foo")], [text("hello")]), "<h3 id=\"foo\">hello</h3>")]
#[case(h4([id("foo")], [text("hello")]), "<h4 id=\"foo\">hello</h4>")]
#[case(h5([id("foo")], [text("hello")]), "<h5 id=\"foo\">hello</h5>")]
#[case(h6([id("foo")], [text("hello")]), "<h6 id=\"foo\">hello</h6>")]
fn should_render_node(#[case] def: Node, #[case] expected: &str) {
    assert_eq!(def.to_string(), expected);
}
