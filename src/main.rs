mod app;

use std::borrow::Borrow;
use std::io::{Read, Write};
use openssl::ssl::{SslMethod, SslAcceptor, SslStream, SslFiletype};
use rcgen::generate_simple_self_signed;
use std::net::{TcpStream, TcpListener};
use std::sync::Arc;
use std::thread;
use file_ext::FileExt;
use rust_web_server::header::Header;
use rust_web_server::request::Request;
use rust_web_server::response::Response;
use rust_web_server::server::Server;
use rust_web_server::symbol::SYMBOL;
use crate::app::App;
use crate::app::controller::tls::TlsController;

fn main() {
    println!("Rust TLS Server | Draft | Work in Progress");

    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    let subject_name = ["localhost".to_string()];
    let certificate = generate_simple_self_signed(subject_name).unwrap();

    let slash_private_key = [SYMBOL.slash, TlsController::PRIVATE_KEY].join("");
    let private_key_path = FileExt::get_static_filepath(&slash_private_key).unwrap();
    FileExt::read_or_create_and_write(&private_key_path, certificate.serialize_private_key_pem().as_bytes()).unwrap();


    let slash_certificate = [SYMBOL.slash, TlsController::CERTIFICATE].join("");
    let certificate_path = FileExt::get_static_filepath(&slash_certificate).unwrap();
    FileExt::read_or_create_and_write(&certificate_path, certificate.serialize_pem().unwrap().as_bytes()).unwrap();


    acceptor.set_private_key_file(private_key_path, SslFiletype::PEM).unwrap();
    acceptor.set_certificate_file(certificate_path, SslFiletype::PEM).unwrap();
    acceptor.check_private_key().unwrap();

    let acceptor = Arc::new(acceptor.build());

    let listener = TcpListener::bind("127.0.0.1:4438").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                thread::spawn(move || {
                   let stream = acceptor.accept(stream).unwrap();
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Connection failed, {}", e.to_string());
            }
        }
    }

}

fn handle_client(mut stream: SslStream<TcpStream>) {
    let mut buffer :[u8; 1024] = [0; 1024];
    let boxed_read = stream.read(&mut buffer);
    if boxed_read.is_err() {
        eprintln!("unable to read TCP stream {}", boxed_read.as_ref().err().unwrap());

        let raw_response = Server::bad_request_response();
        let boxed_stream = stream.write(raw_response.borrow());
        if boxed_stream.is_ok() {
            stream.flush().unwrap();
        };
    }

    boxed_read.unwrap();
    let request : &[u8] = &buffer;


    let boxed_request = Request::parse_request(request);
    if boxed_request.is_err() {
        eprintln!("unable to parse request: {}", boxed_request.as_ref().err().unwrap());

        let raw_response = Server::bad_request_response();
        let boxed_stream = stream.write(raw_response.borrow());
        if boxed_stream.is_ok() {
            stream.flush().unwrap();
        };
    }


    let request: Request = boxed_request.unwrap();
    let (mut response, request) = App::handle_request(request);

    response.headers.push(Header {
        name: "Referrer-Policy".to_string(),
        value: "no-referrer, strict-origin-when-cross-origin".to_string()
    });


    response.headers.push(Header {
        name: "Content-Security-Policy".to_string(),
        value: "default-src https:".to_string()
    });

    response.headers.push(Header {
        name: "Strict-Transport-Security".to_string(),
        value: "max-age=15768000".to_string()
    });

    let raw_response = Response::generate_response(response, request);

    let boxed_stream = stream.write(raw_response.borrow());
    if boxed_stream.is_ok() {
        stream.flush().unwrap();
    };

}
