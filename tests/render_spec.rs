use rstest::rstest;

use fun_html::{
    attr::{self, AnchorTarget},
    elt::{self},
    Attribute, Document, Element,
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
fn should_render_html_document() {
    let doc = fun_html::html(
        [attr::lang("en")],
        [
            elt::head([], [elt::title([], "greeting")]),
            elt::body([], [elt::h1([], [elt::text("Hello world!")])]),
        ],
    );
    assert_eq!(doc.to_string(), "<!DOCTYPE html>\n<html lang=\"en\"><head><title>greeting</title></head><body><h1>Hello world!</h1></body></html>");
}

#[rstest]
#[case(("foo", "bar").into(), "foo=\"bar\"")]
#[case(("x-on:keyup.enter", "doSomething").into(), "x-on:keyup.enter=\"doSomething\"")]
#[case(("@keyup.enter", "doSomething").into(), "@keyup.enter=\"doSomething\"")]
#[case(Attribute::new_unsafe_name("hello".to_string(), "world".to_string()), "hello=\"world\"")]
#[case(attr::id("foo"), "id=\"foo\"")]
#[case(attr::class(["foo"]), "class=\"foo\"")]
#[case(attr::class(["foo", "bar"]), "class=\"foo bar\"")]
#[case(attr::href("foo"), "href=\"foo\"")]
#[case(attr::rel("foo"), "rel=\"foo\"")]
#[case(attr::src("foo"), "src=\"foo\"")]
#[case(attr::type_("foo"), "type=\"foo\"")]
#[case(attr::type_text(), "type=\"text\"")]
#[case(attr::type_password(), "type=\"password\"")]
#[case(attr::type_number(), "type=\"number\"")]
#[case(attr::type_tel(), "type=\"tel\"")]
#[case(attr::type_file(), "type=\"file\"")]
#[case(attr::type_checkbox(), "type=\"checkbox\"")]
#[case(attr::type_radio(), "type=\"radio\"")]
#[case(attr::type_range(), "type=\"range\"")]
#[case(attr::type_email(), "type=\"email\"")]
#[case(attr::type_date(), "type=\"date\"")]
#[case(attr::type_month(), "type=\"month\"")]
#[case(attr::type_hidden(), "type=\"hidden\"")]
#[case(attr::type_reset(), "type=\"reset\"")]
#[case(attr::type_submit(), "type=\"submit\"")]
#[case(attr::integrity("foo"), "integrity=\"foo\"")]
#[case(attr::defer(), "defer")]
#[case(attr::async_(), "async")]
#[case(attr::crossorigin_anonymous(), "crossorigin=\"anonymous\"")]
#[case(attr::crossorigin_use_credentials(), "crossorigin=\"use-credentials\"")]
#[case(attr::download(), "download")]
#[case(attr::download_with_name("myfile.txt"), "download=\"myfile.txt\"")]
#[case(attr::target(AnchorTarget::Blank), "target=\"_blank\"")]
#[case(attr::target_blank(), "target=\"_blank\"")]
#[case(attr::target(AnchorTarget::Self_), "target=\"_self\"")]
#[case(attr::target(AnchorTarget::Top), "target=\"_top\"")]
#[case(attr::target(AnchorTarget::Parent), "target=\"_parent\"")]
#[case(attr::target(AnchorTarget::Frame("myframe".into())), "target=\"myframe\"")]
#[case(attr::charset("foobar"), "charset=\"foobar\"")]
#[case(attr::charset_utf_8(), "charset=\"UTF-8\"")]
#[case(attr::name("hello"), "name=\"hello\"")]
#[case(attr::content("bla"), "content=\"bla\"")]
#[case(attr::alt("bla"), "alt=\"bla\"")]
#[case(attr::width("10"), "width=\"10\"")]
#[case(attr::height("10"), "height=\"10\"")]
#[case(attr::width_int(10), "width=\"10\"")]
#[case(attr::height_int(10), "height=\"10\"")]
#[case(attr::action("something"), "action=\"something\"")]
#[case(attr::method_get(), "method=\"get\"")]
#[case(attr::method_post(), "method=\"post\"")]
#[case(attr::for_("foo"), "for=\"foo\"")]
#[case(attr::value("hello"), "value=\"hello\"")]
#[case(attr::required(), "required")]
#[case(attr::pattern("foobar"), "pattern=\"foobar\"")]
#[case(attr::min("value"), "min=\"value\"")]
#[case(attr::max("value"), "max=\"value\"")]
#[case(attr::minlength("value"), "minlength=\"value\"")]
#[case(attr::maxlength("value"), "maxlength=\"value\"")]
#[case(attr::multiple(), "multiple")]
#[case(attr::placeholder("hello"), "placeholder=\"hello\"")]
#[case(attr::rows(10), "rows=\"10\"")]
#[case(attr::cols(10), "cols=\"10\"")]
fn should_render_attribute(#[case] attr: Attribute, #[case] expected: &str) {
    assert_eq!(attr.to_string(), expected);
}

#[rstest]
#[case(elt::none(), "")]
#[case([elt::div([], []), elt::div([], [])].into(), "<div></div><div></div>")]
#[case(elt::div([], elt::div([], [])), "<div><div></div></div>")]
#[case(elt::text("hello"), "hello")]
#[case(elt::text("hello".to_string()), "hello")]
#[case("hello".into(), "hello")]
#[case("hello".to_string().into(), "hello")]
#[case([elt::div([], ["a".into()]), elt::div([], ["b".into()])].into(), "<div>a</div><div>b</div>")]
#[case(
    elt::raw("<my-component></my-component>"),
    "<my-component></my-component>"
)]
#[case(elt::raw_unsafe("<my-component></my-component>".to_string()), "<my-component></my-component>")]
#[case(elt::link([("foo", "bar").into()]), "<link foo=\"bar\">")]
#[case(
    elt::link_stylesheet("/styles.css"),
    "<link rel=\"stylesheet\" href=\"/styles.css\">"
)]
#[case(elt::script([("foo", "bar").into()], "alert('hello');"), "<script foo=\"bar\">alert('hello');</script>")]
#[case(elt::script_empty([attr::src("/foo.js")]), "<script src=\"/foo.js\"></script>")]
#[case(elt::meta([("foo", "bar").into()]), "<meta foo=\"bar\">")]
#[case(elt::meta_charset_utf_8(), "<meta charset=\"UTF-8\">")]
#[case(
    elt::meta_viewport(),
    "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">"
)]
#[case(
    elt::meta_color_scheme("dark"),
    "<meta name=\"color-scheme\" content=\"dark\">"
)]
#[case(elt::div([("foo", "bar").into()], ["hello".into()]), "<div foo=\"bar\">hello</div>")]
#[case(elt::div([("foo", "bar".to_string()).into()], [elt::text("hello".to_string())]), "<div foo=\"bar\">hello</div>")]
#[case(elt::head([attr::id("foo")], [elt::text("hello")]), "<head id=\"foo\">hello</head>")]
#[case(elt::title([("foo", "bar").into()], "hello"), "<title foo=\"bar\">hello</title>")]
#[case(elt::body([attr::id("foo")], [elt::text("hello")]), "<body id=\"foo\">hello</body>")]
#[case(elt::h1([attr::id("foo")], [elt::text("hello")]), "<h1 id=\"foo\">hello</h1>")]
#[case(elt::h2([attr::id("foo")], [elt::text("hello")]), "<h2 id=\"foo\">hello</h2>")]
#[case(elt::h3([attr::id("foo")], [elt::text("hello")]), "<h3 id=\"foo\">hello</h3>")]
#[case(elt::h4([attr::id("foo")], [elt::text("hello")]), "<h4 id=\"foo\">hello</h4>")]
#[case(elt::h5([attr::id("foo")], [elt::text("hello")]), "<h5 id=\"foo\">hello</h5>")]
#[case(elt::h6([attr::id("foo")], [elt::text("hello")]), "<h6 id=\"foo\">hello</h6>")]
#[case(elt::p([("foo", "bar").into()], ["hello".into()]), "<p foo=\"bar\">hello</p>")]
#[case(elt::br([("foo", "bar").into()]), "<br foo=\"bar\">")]
#[case(elt::small([("foo", "bar").into()], ["hello".into()]), "<small foo=\"bar\">hello</small>")]
#[case(elt::span([("foo", "bar").into()], ["hello".into()]), "<span foo=\"bar\">hello</span>")]
#[case(elt::table([("foo", "bar").into()], ["hello".into()]), "<table foo=\"bar\">hello</table>")]
#[case(elt::tr([("foo", "bar").into()], ["hello".into()]), "<tr foo=\"bar\">hello</tr>")]
#[case(elt::td([("foo", "bar").into()], ["hello".into()]), "<td foo=\"bar\">hello</td>")]
#[case(elt::th([("foo", "bar").into()], ["hello".into()]), "<th foo=\"bar\">hello</th>")]
#[case(elt::thead([("foo", "bar").into()], ["hello".into()]), "<thead foo=\"bar\">hello</thead>")]
#[case(elt::tbody([("foo", "bar").into()], ["hello".into()]), "<tbody foo=\"bar\">hello</tbody>")]
#[case(elt::tfoot([("foo", "bar").into()], ["hello".into()]), "<tfoot foo=\"bar\">hello</tfoot>")]
#[case(elt::a([attr::href("/somepath")], ["visit this cool link!".into()]), "<a href=\"/somepath\">visit this cool link!</a>")]
#[case(elt::img([attr::src("foo"), attr::alt("bar")]), "<img src=\"foo\" alt=\"bar\">")]
#[case(elt::ul([("foo", "bar").into()], [elt::li([("a", "b").into()], ["hello".into()])]), "<ul foo=\"bar\"><li a=\"b\">hello</li></ul>")]
#[case(elt::ol([("foo", "bar").into()], [elt::li([("a", "b").into()], ["hello".into()])]), "<ol foo=\"bar\"><li a=\"b\">hello</li></ol>")]
#[case(elt::section([("foo", "bar").into()], ["hello".into()]), "<section foo=\"bar\">hello</section>")]
#[case(elt::article([("foo", "bar").into()], ["hello".into()]), "<article foo=\"bar\">hello</article>")]
#[case(elt::header([("foo", "bar").into()], ["hello".into()]), "<header foo=\"bar\">hello</header>")]
#[case(elt::main([("foo", "bar").into()], ["hello".into()]), "<main foo=\"bar\">hello</main>")]
#[case(elt::footer([("foo", "bar").into()], ["hello".into()]), "<footer foo=\"bar\">hello</footer>")]
#[case(
    elt::form([attr::action("/do_something"), attr::method_get()], [elt::input([attr::name("foo")])]),
    r#"<form action="/do_something" method="get"><input name="foo"></form>"#
)]
#[case(
    elt::select([attr::name("foo")], [elt::option([attr::value("bar")], ["coucou".into()])]),
    r#"<select name="foo"><option value="bar">coucou</option></select>"#
)]
#[case(elt::input([attr::name("foo"), attr::type_text()]), "<input name=\"foo\" type=\"text\">")]
#[case(elt::textarea([attr::name("foo")], "hello world"), "<textarea name=\"foo\">hello world</textarea>")]
#[case(elt::button([("foo", "bar").into()], ["hello".into()]), "<button foo=\"bar\">hello</button>")]
#[case(elt::label([attr::for_("foo")], ["hello".into()]), "<label for=\"foo\">hello</label>")]
#[case(elt::fieldset([("foo", "bar").into()], ["hello".into()]), "<fieldset foo=\"bar\">hello</fieldset>")]
fn should_render_element(#[case] def: Element, #[case] expected: &str) {
    assert_eq!(def.to_string(), expected);
}

#[test]
fn text_should_be_escaped() {
    let input = "<script>alert('hello');</script>";
    let string = elt::text(input).to_string();
    assert_eq!(string, "&lt;script&gt;alert('hello');&lt;/script&gt;");
}

#[rstest]
fn attribute_should_be_escaped() {
    let string = elt::div(
        [("foo", "<script>\"\" { open: !close }").into()],
        [elt::text("hello")],
    )
    .to_string();
    assert_eq!(
        string,
        "<div foo=\"&lt;script&gt;&quot;&quot; { open: !close }\">hello</div>"
    );
}

#[rstest]
fn script_should_be_escaped() {
    let string = elt::script([], "alert('</script>');").to_string();
    assert_eq!(string, "<script>alert('<\\/script>');</script>");
}
