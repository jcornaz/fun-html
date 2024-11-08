use rstest::rstest;

use fun_html::{attr::*, elt::*, html, Attribute, Document, Element};

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
#[case(Attribute::new_unsafe_name("hello".to_string(), "world".to_string()), "hello=\"world\"")]
#[case(id("foo"), "id=\"foo\"")]
#[case(class(["foo"]), "class=\"foo\"")]
#[case(class(["foo", "bar"]), "class=\"foo bar\"")]
#[case(href("foo"), "href=\"foo\"")]
#[case(rel("foo"), "rel=\"foo\"")]
#[case(src("foo"), "src=\"foo\"")]
#[case(type_("foo"), "type=\"foo\"")]
#[case(type_text(), "type=\"text\"")]
#[case(type_password(), "type=\"password\"")]
#[case(type_number(), "type=\"number\"")]
#[case(type_tel(), "type=\"tel\"")]
#[case(type_file(), "type=\"file\"")]
#[case(type_checkbox(), "type=\"checkbox\"")]
#[case(type_radio(), "type=\"radio\"")]
#[case(type_range(), "type=\"range\"")]
#[case(type_email(), "type=\"email\"")]
#[case(type_date(), "type=\"date\"")]
#[case(type_month(), "type=\"month\"")]
#[case(type_hidden(), "type=\"hidden\"")]
#[case(type_reset(), "type=\"reset\"")]
#[case(type_submit(), "type=\"submit\"")]
#[case(integrity("foo"), "integrity=\"foo\"")]
#[case(defer(), "defer")]
#[case(async_(), "async")]
#[case(crossorigin_anonymous(), "crossorigin=\"anonymous\"")]
#[case(crossorigin_use_credentials(), "crossorigin=\"use-credentials\"")]
#[case(download(), "download")]
#[case(download_with_name("myfile.txt"), "download=\"myfile.txt\"")]
#[case(target(AnchorTarget::Blank), "target=\"_blank\"")]
#[case(target_blank(), "target=\"_blank\"")]
#[case(target(AnchorTarget::Self_), "target=\"_self\"")]
#[case(target(AnchorTarget::Top), "target=\"_top\"")]
#[case(target(AnchorTarget::Parent), "target=\"_parent\"")]
#[case(target(AnchorTarget::Frame("myframe".into())), "target=\"myframe\"")]
#[case(charset("foobar"), "charset=\"foobar\"")]
#[case(charset_utf_8(), "charset=\"UTF-8\"")]
#[case(name("hello"), "name=\"hello\"")]
#[case(content("bla"), "content=\"bla\"")]
#[case(alt("bla"), "alt=\"bla\"")]
#[case(width("10"), "width=\"10\"")]
#[case(height("10"), "height=\"10\"")]
#[case(width_int(10), "width=\"10\"")]
#[case(height_int(10), "height=\"10\"")]
#[case(action("something"), "action=\"something\"")]
#[case(method_get(), "method=\"get\"")]
#[case(method_post(), "method=\"post\"")]
#[case(for_("foo"), "for=\"foo\"")]
#[case(value("hello"), "value=\"hello\"")]
#[case(required(), "required")]
#[case(pattern("foobar"), "pattern=\"foobar\"")]
#[case(min("value"), "min=\"value\"")]
#[case(max("value"), "max=\"value\"")]
#[case(minlength("value"), "minlength=\"value\"")]
#[case(maxlength("value"), "maxlength=\"value\"")]
#[case(multiple(), "multiple")]
#[case(placeholder("hello"), "placeholder=\"hello\"")]
#[case(rows(10), "rows=\"10\"")]
#[case(cols(10), "cols=\"10\"")]
fn should_render_attribute(#[case] attr: Attribute, #[case] expected: &str) {
    assert_eq!(attr.to_string(), expected);
}

#[rstest]
#[case(none(), "")]
#[case([div([], []), div([], [])].into(), "<div></div><div></div>")]
#[case(div([], div([], [])), "<div><div></div></div>")]
#[case(text("hello"), "hello")]
#[case(text("hello".to_string()), "hello")]
#[case("hello".into(), "hello")]
#[case("hello".to_string().into(), "hello")]
#[case([div([], ["a".into()]), div([], ["b".into()])].into(), "<div>a</div><div>b</div>")]
#[case(raw("<my-component></my-component>"), "<my-component></my-component>")]
#[case(raw_unsafe("<my-component></my-component>".to_string()), "<my-component></my-component>")]
#[case(link([("foo", "bar").into()]), "<link foo=\"bar\">")]
#[case(
    link_stylesheet("/styles.css"),
    "<link rel=\"stylesheet\" href=\"/styles.css\">"
)]
#[case(script([("foo", "bar").into()], "alert('hello');"), "<script foo=\"bar\">alert('hello');</script>")]
#[case(script_empty([src("/foo.js")]), "<script src=\"/foo.js\"></script>")]
#[case(meta([("foo", "bar").into()]), "<meta foo=\"bar\">")]
#[case(meta_charset_utf_8(), "<meta charset=\"UTF-8\">")]
#[case(
    meta_viewport(),
    "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">"
)]
#[case(
    meta_color_scheme("dark"),
    "<meta name=\"color-scheme\" content=\"dark\">"
)]
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
#[case(p([("foo", "bar").into()], ["hello".into()]), "<p foo=\"bar\">hello</p>")]
#[case(br([("foo", "bar").into()]), "<br foo=\"bar\">")]
#[case(small([("foo", "bar").into()], ["hello".into()]), "<small foo=\"bar\">hello</small>")]
#[case(span([("foo", "bar").into()], ["hello".into()]), "<span foo=\"bar\">hello</span>")]
#[case(table([("foo", "bar").into()], ["hello".into()]), "<table foo=\"bar\">hello</table>")]
#[case(tr([("foo", "bar").into()], ["hello".into()]), "<tr foo=\"bar\">hello</tr>")]
#[case(td([("foo", "bar").into()], ["hello".into()]), "<td foo=\"bar\">hello</td>")]
#[case(th([("foo", "bar").into()], ["hello".into()]), "<th foo=\"bar\">hello</th>")]
#[case(thead([("foo", "bar").into()], ["hello".into()]), "<thead foo=\"bar\">hello</thead>")]
#[case(tbody([("foo", "bar").into()], ["hello".into()]), "<tbody foo=\"bar\">hello</tbody>")]
#[case(tfoot([("foo", "bar").into()], ["hello".into()]), "<tfoot foo=\"bar\">hello</tfoot>")]
#[case(a([href("/somepath")], ["visit this cool link!".into()]), "<a href=\"/somepath\">visit this cool link!</a>")]
#[case(img([src("foo"), alt("bar")]), "<img src=\"foo\" alt=\"bar\">")]
#[case(ul([("foo", "bar").into()], [li([("a", "b").into()], ["hello".into()])]), "<ul foo=\"bar\"><li a=\"b\">hello</li></ul>")]
#[case(ol([("foo", "bar").into()], [li([("a", "b").into()], ["hello".into()])]), "<ol foo=\"bar\"><li a=\"b\">hello</li></ol>")]
#[case(section([("foo", "bar").into()], ["hello".into()]), "<section foo=\"bar\">hello</section>")]
#[case(article([("foo", "bar").into()], ["hello".into()]), "<article foo=\"bar\">hello</article>")]
#[case(header([("foo", "bar").into()], ["hello".into()]), "<header foo=\"bar\">hello</header>")]
#[case(main([("foo", "bar").into()], ["hello".into()]), "<main foo=\"bar\">hello</main>")]
#[case(footer([("foo", "bar").into()], ["hello".into()]), "<footer foo=\"bar\">hello</footer>")]
#[case(
    form([action("/do_something"), method_get()], [input([name("foo")])]),
    r#"<form action="/do_something" method="get"><input name="foo"></form>"#
)]
#[case(
    select([name("foo")], [option([value("bar")], ["coucou".into()])]),
    r#"<select name="foo"><option value="bar">coucou</option></select>"#
)]
#[case(input([name("foo"), type_text()]), "<input name=\"foo\" type=\"text\">")]
#[case(textarea([name("foo")], "hello world"), "<textarea name=\"foo\">hello world</textarea>")]
#[case(button([("foo", "bar").into()], ["hello".into()]), "<button foo=\"bar\">hello</button>")]
#[case(label([for_("foo")], ["hello".into()]), "<label for=\"foo\">hello</label>")]
#[case(fieldset([("foo", "bar").into()], ["hello".into()]), "<fieldset foo=\"bar\">hello</fieldset>")]
fn should_render_element(#[case] def: Element, #[case] expected: &str) {
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
fn script_should_be_escaped() {
    let string = script([], "alert('</script>');").to_string();
    assert_eq!(string, "<script>alert('<\\/script>');</script>");
}
