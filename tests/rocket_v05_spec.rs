#![cfg(feature = "rocket_v05")]

#[macro_use]
extern crate rocket_v05 as rocket;

use rstest::{fixture, rstest};

use fun_html::{
    attr::id,
    elt::{body, div, head},
    html, Document, Element,
};
use rocket::{
    get,
    http::{ContentType, Status},
    local::blocking::Client,
};

#[get("/document")]
fn get_document() -> Document {
    html([], [head([], []), body([], ["hello".into()])])
}

#[get("/fragment")]
fn get_fragment() -> Element {
    div([id("foo")], ["hello".into()])
}

#[fixture]
fn client() -> Client {
    let rocket = rocket_v05::build().mount("/", routes![get_document, get_fragment]);
    Client::tracked(rocket).unwrap()
}

#[rstest]
fn document_should_implement_response(client: Client) {
    let response = client.get("/document").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    assert_eq!(
        response.into_string().expect("should have a body"),
        "<!DOCTYPE html>\n<html><head></head><body>hello</body></html>"
    );
}

#[rstest]
fn element_should_implement_response(client: Client) {
    let response = client.get("/fragment").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    assert_eq!(
        response.into_string().expect("should have a body"),
        "<div id=\"foo\">hello</div>"
    );
}
