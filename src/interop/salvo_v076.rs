use salvo_v076::prelude::*;

impl Scribe for crate::Element {
    fn render(self, res: &mut Response) {
        res.render(Text::Html(self.to_string()));
    }
}

impl Scribe for crate::Document {
    fn render(self, res: &mut Response) {
        res.render(Text::Html(self.to_string()));
    }
}

#[cfg(test)]
mod tests {
    use crate::{elt::div, html};

    use super::*;

    #[test]
    fn document_should_set_content_type() {
        let mut doc_resp = Response::new();
        doc_resp.render(html([], []));
        assert_eq!(
            doc_resp.content_type(),
            Some("text/html; charset=utf-8".parse().unwrap())
        );
    }

    #[test]
    fn element_should_set_content_type() {
        let mut doc_resp = Response::new();
        doc_resp.render(div([], []));
        assert_eq!(
            doc_resp.content_type(),
            Some("text/html; charset=utf-8".parse().unwrap())
        );
    }
}
