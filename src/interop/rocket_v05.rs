use rocket_v05::response::{content::RawHtml, Responder};

impl<'r> Responder<'r, 'static> for crate::Element {
    fn respond_to(self, req: &'r rocket_v05::Request<'_>) -> rocket_v05::response::Result<'static> {
        RawHtml(self.to_string()).respond_to(req)
    }
}

impl<'r> Responder<'r, 'static> for crate::Document {
    fn respond_to(self, req: &'r rocket_v05::Request<'_>) -> rocket_v05::response::Result<'static> {
        RawHtml(self.to_string()).respond_to(req)
    }
}
