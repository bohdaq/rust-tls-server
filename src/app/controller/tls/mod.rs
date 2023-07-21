use rust_web_server::controller::Controller;
use rust_web_server::request::Request;
use rust_web_server::response::{Response, STATUS_CODE_REASON_PHRASE};
use rust_web_server::server::ConnectionInfo;
use rust_web_server::symbol::SYMBOL;

pub struct TlsController;

impl Controller for TlsController {
    fn is_matching(request: &Request, _connection: &ConnectionInfo) -> bool {
        let private_key_path = [SYMBOL.slash, TlsController::PRIVATE_KEY].join("");
        request.request_uri == private_key_path
    }

    fn process(_request: &Request, mut response: Response, _connection: &ConnectionInfo) -> Response {
        response.status_code = *STATUS_CODE_REASON_PHRASE.n403_forbidden.status_code;
        response.reason_phrase = STATUS_CODE_REASON_PHRASE.n403_forbidden.reason_phrase.to_string();


        response
    }
}

impl TlsController {
    pub const PRIVATE_KEY: &'static str = "private.key";
    pub const CERTIFICATE: &'static str = "certificate.crt";
}