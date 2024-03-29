use file_ext::FileExt;
use rust_web_server::controller::Controller;
use rust_web_server::mime_type::MimeType;
use rust_web_server::range::Range;
use rust_web_server::request::Request;
use rust_web_server::response::{Response, STATUS_CODE_REASON_PHRASE};
use rust_web_server::server::ConnectionInfo;

pub struct FaviconController;

impl Controller for FaviconController {
    fn is_matching(request: &Request, _connection: &ConnectionInfo) -> bool {
        request.request_uri == "/favicon.svg"
    }

    fn process(_request: &Request, mut response: Response, _connection: &ConnectionInfo) -> Response {
        response.status_code = *STATUS_CODE_REASON_PHRASE.n200_ok.status_code;
        response.reason_phrase = STATUS_CODE_REASON_PHRASE.n200_ok.reason_phrase.to_string();


        if FileExt::does_file_exist(FaviconController::STYLE_FILEPATH) {
            let boxed_content_range =
                Range::get_content_range_of_a_file(FaviconController::STYLE_FILEPATH);

            if boxed_content_range.is_ok() {
                let content_range = boxed_content_range.unwrap();
                let content_range_list = vec![content_range];
                response.content_range_list = content_range_list;
            } else {
                let error = boxed_content_range.err().unwrap();
                let mime_type = MimeType::TEXT_HTML.to_string();
                let content_range = Range::get_content_range(
                    Vec::from(error.as_bytes()),
                    mime_type
                );

                let content_range_list = vec![content_range];
                response.content_range_list = content_range_list;
                response.status_code = *STATUS_CODE_REASON_PHRASE.n500_internal_server_error.status_code;
                response.reason_phrase = STATUS_CODE_REASON_PHRASE.n500_internal_server_error.reason_phrase.to_string();
            }
        } else {
            let style_file = include_bytes!("favicon.svg");

            let content_range =
                Range::get_content_range(style_file.to_vec(), MimeType::IMAGE_SVG.to_string());


            let content_range_list = vec![content_range];
            response.content_range_list = content_range_list;

        }

        response
    }
}

impl FaviconController {
    pub const STYLE_FILEPATH: &'static str = "favicon.svg";
}