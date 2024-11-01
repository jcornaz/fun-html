use rstest::rstest;

use fun_html::{attributes::*, html, nodes::*, Attribute, Document, Element};

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
            head([], [title([], "greeting")]),
            body([], [h1([], [text("Hello world!")])]),
        ],
    );
    assert_eq!(doc.to_string(), "<!DOCTYPE html>\n<html lang=\"en\"><head><title>greeting</title></head><body><h1>Hello world!</h1></body></html>");
}

#[rstest]
#[case(("foo", "bar").into(), "foo=\"bar\"")]
#[case(("x-on:keyup.enter", "doSomething").into(), "x-on:keyup.enter=\"doSomething\"")]
#[case(("@keyup.enter", "doSomething").into(), "@keyup.enter=\"doSomething\"")]
#[case(id("foo"), "id=\"foo\"")]
#[case(class(["foo"]), "class=\"foo\"")]
#[case(class(["foo", "bar"]), "class=\"foo bar\"")]
fn should_render_attribute(#[case] attr: Attribute, #[case] expected: &str) {
    let string = div([attr], []).to_string();
    assert_eq!(string, format!("<div {expected}></div>"));
}

#[rstest]
#[case(text("hello"), "hello")]
#[case(text("hello".to_string()), "hello")]
#[case("hello".into(), "hello")]
#[case("hello".to_string().into(), "hello")]
#[case([div([], ["a".into()]), div([], ["b".into()])].into(), "<div>a</div><div>b</div>")]
#[case(raw("<my-component></my-component>"), "<my-component></my-component>")]
#[case(raw_unsafe("<my-component></my-component>".to_string()), "<my-component></my-component>")]
#[case(meta([("foo", "bar").into()]), "<meta foo=\"bar\">")]
#[case(div([("foo", "bar").into()], ["hello".into()]), "<div foo=\"bar\">hello</div>")]
#[case(div([("foo", "bar".to_string()).into()], [text("hello".to_string())]), "<div foo=\"bar\">hello</div>")]
#[case(head([id("foo")], [text("hello")]), "<head id=\"foo\">hello</head>")]
#[case(title([("foo", "bar").into()], "hello"), "<title foo=\"bar\">hello</title>")]
#[case(body([id("foo")], [text("hello")]), "<body id=\"foo\">hello</body>")]
#[case(h1([id("foo")], [text("hello")]), "<h1 id=\"foo\">hello</h1>")]
#[case(h2([id("foo")], [text("hello")]), "<h2 id=\"foo\">hello</h2>")]
#[case(h3([id("foo")], [text("hello")]), "<h3 id=\"foo\">hello</h3>")]
#[case(h4([id("foo")], [text("hello")]), "<h4 id=\"foo\">hello</h4>")]
#[case(h5([id("foo")], [text("hello")]), "<h5 id=\"foo\">hello</h5>")]
#[case(h6([id("foo")], [text("hello")]), "<h6 id=\"foo\">hello</h6>")]
fn should_render_node(#[case] def: Element, #[case] expected: &str) {
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
        [("foo", "<script>\"\" { open: !close }").into()],
        [text("hello")],
    )
    .to_string();
    assert_eq!(
        string,
        "<div foo=\"&lt;script&gt;&quot;&quot; { open: !close }\">hello</div>"
    );
}

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
