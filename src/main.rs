mod app;

use std::borrow::Borrow;
use std::io::{Read, Write};
use openssl::ssl::{SslMethod, SslAcceptor, SslStream, SslFiletype};
use rcgen::generate_simple_self_signed;
use std::net::{TcpStream, TcpListener};
use std::sync::Arc;
use file_ext::FileExt;
use rust_web_server::entry_point::{bootstrap, get_ip_port_thread_count};
use rust_web_server::header::Header;
use rust_web_server::request::Request;
use rust_web_server::response::Response;
use rust_web_server::server::Server;
use rust_web_server::symbol::SYMBOL;
use rust_web_server::thread_pool::ThreadPool;
use crate::app::App;
use crate::app::controller::tls::TlsController;

fn main() {
    println!("Rust TLS Server | Draft | Work in Progress");
    bootstrap();

    let boxed_acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls());
    if boxed_acceptor.is_err() {
        println!("{}", boxed_acceptor.as_ref().err().unwrap().to_string());
    }
    let mut acceptor = boxed_acceptor.unwrap();

    let subject_name = ["localhost".to_string()];

    let boxed_certificate = generate_simple_self_signed(subject_name);
    if boxed_certificate.is_err() {
        println!("{}", boxed_certificate.as_ref().err().unwrap().to_string());
    }
    let certificate = boxed_certificate.unwrap();

    let slash_private_key = [SYMBOL.slash, TlsController::PRIVATE_KEY].join("");

    let boxed_private_key_path = FileExt::get_static_filepath(&slash_private_key);
    if boxed_private_key_path.is_err() {
        println!("{}", boxed_private_key_path.as_ref().err().unwrap().to_string());
    }
    let private_key_path = boxed_private_key_path.unwrap();
    FileExt::read_or_create_and_write(&private_key_path, certificate.serialize_private_key_pem().as_bytes()).unwrap();


    let slash_certificate = [SYMBOL.slash, TlsController::CERTIFICATE].join("");
    let certificate_path = FileExt::get_static_filepath(&slash_certificate).unwrap();
    FileExt::read_or_create_and_write(&certificate_path, certificate.serialize_pem().unwrap().as_bytes()).unwrap();


    acceptor.set_private_key_file(private_key_path, SslFiletype::PEM).unwrap();
    acceptor.set_certificate_file(certificate_path, SslFiletype::PEM).unwrap();
    acceptor.check_private_key().unwrap();

    let acceptor = Arc::new(acceptor.build());

    let (ip, port, thread_count) = get_ip_port_thread_count();
    let bind_addr = [ip, SYMBOL.colon.to_string(), port.to_string()].join(SYMBOL.empty_string);
    let listener = TcpListener::bind(bind_addr).unwrap();
    let pool = ThreadPool::new(thread_count as usize);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                pool.execute(move || {
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
        name: Header::_REFERRER_POLICY.to_string(),
        value: "no-referrer, strict-origin-when-cross-origin".to_string()
    });


    response.headers.push(Header {
        name: Header::_CONTENT_SECURITY_POLICY.to_string(),
        value: "default-src https:".to_string()
    });

    response.headers.push(Header {
        name: Header::_STRICT_TRANSPORT_SECURITY.to_string(),
        value: "max-age=15768000".to_string()
    });

    let raw_response = Response::generate_response(response, request);

    let boxed_stream = stream.write(raw_response.borrow());
    if boxed_stream.is_ok() {
        stream.flush().unwrap();
    };

}
