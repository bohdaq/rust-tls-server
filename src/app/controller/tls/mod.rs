use rust_web_server::mime_type::MimeType;
use rust_web_server::range::Range;
use rust_web_server::request::Request;
use rust_web_server::response::{Response, STATUS_CODE_REASON_PHRASE};
use rust_web_server::symbol::SYMBOL;

pub struct TlsController;

impl TlsController {
    pub const PRIVATE_KEY_FILEPATH: &'static str = "private_key";
    pub const CERTIFICATE_FILEPATH: &'static str = "certificate";

    pub fn is_matching_request(request: &Request) -> bool {
        let private_key_path = [SYMBOL.slash, TlsController::PRIVATE_KEY_FILEPATH].join("");
        let certificate_path = [SYMBOL.slash, TlsController::CERTIFICATE_FILEPATH].join("");

        request.request_uri == private_key_path || request.request_uri == certificate_path
    }

    pub fn process_request(_request: &Request, mut response: Response) -> Response {
        response.status_code = *STATUS_CODE_REASON_PHRASE.n403_forbidden.status_code;
        response.reason_phrase = STATUS_CODE_REASON_PHRASE.n403_forbidden.reason_phrase.to_string();


        response
    }
}