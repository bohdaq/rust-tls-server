use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use rust_web_server::header::Header;
use rust_web_server::http::VERSION;
use rust_web_server::request::{METHOD, Request};
use crate::app::App;
use crate::log::Log;

#[test]
fn log_request_response() {
    let request = Request {
        method: METHOD.get.to_string(),
        request_uri: "/script.js".to_string(),
        http_version: VERSION.http_1_1.to_string(),
        headers: vec![
            Header { name: Header::_HOST.to_string(), value: "127.0.0.1:7878".to_string() },
            Header { name: Header::_USER_AGENT.to_string(), value: "SOME USER AGENT".to_string() },
            Header { name: Header::_ACCEPT.to_string(), value: "*/*".to_string() },
            Header { name: Header::_ACCEPT_LANGUAGE.to_string(), value: "en-US,en;q=0.5".to_string() },
            Header { name: Header::_ACCEPT_ENCODING.to_string(), value: "gzip, deflate, br".to_string() },
            Header { name: Header::_REFERER.to_string(), value: "https://127.0.0.1:7878/".to_string() },
        ],
    };

    let (response, request) = App::handle_request(request);

    let timestamp = response._get_header(Header::_DATE_UNIX_EPOCH_NANOS.to_string()).unwrap();
    let expected_log = format!("\n\nRequest (peer address is 0.0.0.0:0):\n  HTTP/1.1 GET /script.js  \n  Host: 127.0.0.1:7878\n  User-Agent: SOME USER AGENT\n  Accept: */*\n  Accept-Language: en-US,en;q=0.5\n  Accept-Encoding: gzip, deflate, br\n  Referer: https://127.0.0.1:7878/\nEnd of Request\nResponse:\n  200 OK \n  Accept-CH: Sec-CH-UA-Arch, Sec-CH-UA-Bitness, Sec-CH-UA-Full-Version-List, Sec-CH-UA-Model, Sec-CH-UA-Platform-Version, Downlink, ECT, RTT, Save-Data, Device-Memory\n  Vary: Origin, Sec-CH-UA-Arch, Sec-CH-UA-Bitness, Sec-CH-UA-Full-Version-List, Sec-CH-UA-Model, Sec-CH-UA-Platform-Version, Save-Data, Device-Memory, Upgrade-Insecure-Requests\n  X-Content-Type-Options: nosniff\n  Accept-Ranges: bytes\n  X-Frame-Options: SAMEORIGIN\n  Date-Unix-Epoch-Nanos: {}\n\n  Body: 1 part(s), 271 byte(s) total\nEnd of Response", timestamp.value);

    let peer_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0,0,0,0)), 0);
    let log = Log::request_response(&request, &response, &peer_addr);

    assert_eq!(expected_log, log);
}