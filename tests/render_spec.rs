use rstest::rstest;

use fun_html::{html, prelude::*};

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
fn should_render_html_document() {
    let doc = html(
        [lang("en")],
        [
            head([], [title([], [text("greeting")])]),
            body([], [h1([], [text("Hello world!")])]),
        ],
    );
    assert_eq!(doc.to_string(), "<!DOCTYPE html>\n<html lang=\"en\"><head><title>greeting</title></head><body><h1>Hello world!</h1></body></html>");
}

#[test]
fn should_render_attributes() {
    let node = h1(
        [class(["underlined", "blue"]), attr("foo", "bar")],
        [text("Hello world!")],
    );
    assert_eq!(
        node.to_string(),
        "<h1 class=\"underlined blue\" foo=\"bar\">Hello world!</h1>"
    )
}

#[rstest]
#[case(div([attr("foo", "bar")], [text("hello")]), "<div foo=\"bar\">hello</div>")]
#[case(div([attr("foo", "bar".to_string())], [text("hello".to_string())]), "<div foo=\"bar\">hello</div>")]
#[case(head([id("foo")], [text("hello")]), "<head id=\"foo\">hello</head>")]
#[case(title([attr("foo", "bar")], [text("hello")]), "<title foo=\"bar\">hello</title>")]
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

#[test]
fn text_should_be_escaped() {
    let input = "<script>alert('hello');</script>";
    let string = text(input).to_string();
    assert_eq!(string, "&lt;script&gt;alert('hello');&lt;/script&gt;");
}

#[rstest]
fn attribute_should_be_escaped() {
    let string = div(
        [attr("foo", "<script>\"\" { open: !close }")],
        [text("hello")],
    )
    .to_string();
    assert_eq!(
        string,
        "<div foo=\"&lt;script&gt;&quot;&quot; { open: !close }\">hello</div>"
    );
}
