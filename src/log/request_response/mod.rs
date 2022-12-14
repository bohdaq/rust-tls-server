#[cfg(test)]
mod tests;

use rust_web_server::request::Request;
use rust_web_server::response::Response;

pub struct Log;

impl Log {
    pub fn request_response(request: &Request, response: &Response) -> String {
        let mut request_headers = "".to_string();
        for header in &request.headers {
            if &header.name.chars().count() > &0 {
                request_headers = [
                    request_headers,
                    "\n  ".to_string(),
                    header.name.to_string(),
                    ": ".to_string(),
                    header.value.to_string()
                ].join("");
            }
        }

        let mut response_headers = "".to_string();
        for header in &response.headers {
            if &header.name.chars().count() > &0 {
                response_headers = [
                    response_headers,
                    "\n  ".to_string(),
                    header.name.to_string(),
                    ": ".to_string(),
                    header.value.to_string()
                ].join("");
            }
        }

        let mut response_body_length = 0;
        let mut response_body_parts_number = 0;
        for content_range in &response.content_range_list {
            let boxed_parse = content_range.size.parse::<i32>();
            if boxed_parse.is_ok() {
                response_body_length += boxed_parse.unwrap();
                response_body_parts_number += 1;
            }
        }

        let log_request_response = format!("\n\nRequest:\n  {} {} {}  {}\nEnd of Request\nResponse:\n  {} {} {}\n\n  Body: {} part(s), {} byte(s) total\nEnd of Response",
                                           &request.http_version,
                                           &request.method,
                                           &request.request_uri,
                                           request_headers,

                                           &response.status_code,
                                           &response.reason_phrase,
                                           response_headers,
                                           response_body_parts_number,
                                           response_body_length);

        log_request_response
    }
}