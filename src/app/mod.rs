pub mod controller;

use rust_web_server::app::controller::static_resource::StaticResourceController;
use rust_web_server::header::Header;
use rust_web_server::request::Request;
use rust_web_server::response::{Response, STATUS_CODE_REASON_PHRASE};
use crate::app::controller::favicon::FaviconController;
use crate::app::controller::tls::TlsController;
use crate::app::controller::index::IndexController;
use crate::app::controller::not_found::NotFoundController;
use crate::app::controller::script::ScriptController;
use crate::app::controller::style::StyleController;

pub struct App {}

impl App {
    pub fn handle_request(request: Request) -> (Response, Request) {
        let header_list = Header::get_header_list(&request);

        let mut response: Response = Response::get_response(
            STATUS_CODE_REASON_PHRASE.n501_not_implemented,
            Some(header_list),
            None
        );


        if TlsController::is_matching_request(&request) {
            response = TlsController::process_request(&request, response);
            return (response, request)
        }

        if IndexController::is_matching_request(&request) {
            response = IndexController::process_request(&request, response);
            return (response, request)
        }

        if StyleController::is_matching_request(&request) {
            response = StyleController::process_request(&request, response);
            return (response, request)
        }

        if ScriptController::is_matching_request(&request) {
            response = ScriptController::process_request(&request, response);
            return (response, request)
        }

        if FaviconController::is_matching_request(&request) {
            response = FaviconController::process_request(&request, response);
            return (response, request)
        }

        if StaticResourceController::is_matching_request(&request) {
            response = StaticResourceController::process_request(&request, response);
            return (response, request)
        }

        if NotFoundController::is_matching_request(&request) {
            response = NotFoundController::process_request(&request, response);
            return (response, request)
        }


        (response, request)
    }

}