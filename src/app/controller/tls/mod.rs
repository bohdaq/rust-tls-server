use rust_web_server::request::Request;
use rust_web_server::response::{Response, STATUS_CODE_REASON_PHRASE};
use rust_web_server::symbol::SYMBOL;

pub struct TlsController;

impl TlsController {
    pub const PRIVATE_KEY: &'static str = "private.key";
    pub const CERTIFICATE: &'static str = "certificate.crt";

    pub fn is_matching_request(request: &Request) -> bool {
        let private_key_path = [SYMBOL.slash, TlsController::PRIVATE_KEY].join("");
        request.request_uri == private_key_path
    }

    pub fn process_request(_request: &Request, mut response: Response) -> Response {
        response.status_code = *STATUS_CODE_REASON_PHRASE.n403_forbidden.status_code;
        response.reason_phrase = STATUS_CODE_REASON_PHRASE.n403_forbidden.reason_phrase.to_string();


        response
    }
}