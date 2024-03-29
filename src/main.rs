extern crate core;

mod app;

use std::borrow::Borrow;
use std::io::{Read, Write};
use openssl::ssl::{SslMethod, SslAcceptor, SslStream, SslFiletype};
use rcgen::generate_simple_self_signed;
use std::net::{TcpStream, TcpListener, SocketAddr, IpAddr, Ipv4Addr};
use std::sync::Arc;
use rust_web_server::entry_point::{bootstrap, get_ip_port_thread_count, get_request_allocation_size, set_default_values};
use file_ext::FileExt;
use rust_web_server::application::Application;
use rust_web_server::core::New;
use rust_web_server::header::Header;
use rust_web_server::request::Request;
use rust_web_server::response::Response;
use rust_web_server::server::{Address, ConnectionInfo, Server};
use rust_web_server::symbol::SYMBOL;
use rust_web_server::log::Log;
use rust_web_server::thread_pool::ThreadPool;
use crate::app::App;
use crate::app::controller::tls::TlsController;

fn main() {
    let info = Log::info("Rust TLS Server");
    println!("{}", info);

    let usage_info = Log::usage_information();
    println!("{}", usage_info);
    println!("RWS Configuration Start: \n");
    set_default_values();
    bootstrap();

    let boxed_acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls());
    if boxed_acceptor.is_err() {
        eprintln!("{}", boxed_acceptor.as_ref().err().unwrap().to_string());
        return;
    }
    let mut acceptor = boxed_acceptor.unwrap();

    let subject_name = ["localhost".to_string()];

    let boxed_certificate = generate_simple_self_signed(subject_name);
    if boxed_certificate.is_err() {
        eprintln!("{}", boxed_certificate.as_ref().err().unwrap().to_string());
        return;
    }
    let certificate = boxed_certificate.unwrap();

    let slash_private_key = [SYMBOL.slash, TlsController::PRIVATE_KEY].join("");

    let boxed_private_key_path = FileExt::get_static_filepath(&slash_private_key);
    if boxed_private_key_path.is_err() {
        eprintln!("{}", boxed_private_key_path.as_ref().err().unwrap().to_string());
        return;
    }
    let private_key_path = boxed_private_key_path.as_ref().unwrap();
    let boxed_read_or_write = FileExt::read_or_create_and_write(&private_key_path, certificate.serialize_private_key_pem().as_bytes());
    if boxed_read_or_write.is_err() {
        eprintln!("{}", boxed_read_or_write.as_ref().err().unwrap().to_string());
        return;
    }

    let slash_certificate = [SYMBOL.slash, TlsController::CERTIFICATE].join("");
    let boxed_certificate_path = FileExt::get_static_filepath(&slash_certificate);
    if boxed_certificate_path.is_err() {
        eprintln!("{}", boxed_certificate_path.as_ref().err().unwrap().to_string());
        return;
    }
    let certificate_path = boxed_certificate_path.unwrap();
    let boxed_serialized_certificate = certificate.serialize_pem();
    if boxed_serialized_certificate.is_err() {
        eprintln!("{}", boxed_serialized_certificate.err().unwrap().to_string());
        return;
    }

    let serialized_certificate = boxed_serialized_certificate.unwrap();
    let boxed_read_or_write = FileExt::read_or_create_and_write(&certificate_path, serialized_certificate.as_bytes());
    if boxed_read_or_write.is_err() {
        eprintln!("{}", boxed_read_or_write.err().unwrap().to_string());
        return;
    }

    let boxed_private_key_set = acceptor.set_private_key_file(private_key_path, SslFiletype::PEM);
    if boxed_private_key_set.is_err() {
        eprintln!("{}", boxed_private_key_set.err().unwrap().to_string());
        return;
    }
    boxed_private_key_set.unwrap();

    let boxed_certificate_set = acceptor.set_certificate_file(certificate_path, SslFiletype::PEM);
    if boxed_certificate_set.is_err() {
        eprintln!("{}", boxed_certificate_set.err().unwrap().to_string());
        return;
    }
    boxed_certificate_set.unwrap();

    let boxed_check = acceptor.check_private_key();
    if boxed_check.is_err() {
        eprintln!("{}", boxed_check.err().unwrap().to_string());
        return;
    }
    boxed_check.unwrap();

    let acceptor = Arc::new(acceptor.build());


    let (ip, port, thread_count) = get_ip_port_thread_count();
    let mut ip_readable = ip.to_string();
    if ip_readable.contains(":") {
        ip_readable = [SYMBOL.opening_square_bracket, &ip, SYMBOL.closing_square_bracket].join("");
    }

    let bind_addr = [ip_readable, SYMBOL.colon.to_string(), port.to_string()].join(SYMBOL.empty_string);

    let boxed_listener = TcpListener::bind(&bind_addr);
    if boxed_listener.is_err() {
        eprintln!("{}", boxed_listener.err().unwrap().to_string());
        return;
    }
    let listener = boxed_listener.unwrap();
    let pool = ThreadPool::new(thread_count as usize);

    let server_url_thread_count = Log::server_url_thread_count("https", &bind_addr, thread_count);
    println!("{}", server_url_thread_count);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                pool.execute(move || {
                    let boxed_accept = acceptor.accept(stream);
                    if boxed_accept.is_err() {
                        eprintln!("{}", boxed_accept.err().unwrap().to_string());
                        return;
                    }
                    let stream = boxed_accept.unwrap();
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed, {}", e.to_string());
            }
        }
    }

}

fn handle_client(mut stream: SslStream<TcpStream>) {
    let mut buffer :[u8; 1024] = [0; 1024];
    let boxed_read = stream.read(&mut buffer);
    if boxed_read.is_err() {
        let message = boxed_read.as_ref().err().unwrap().to_string();
        eprintln!("unable to read TCP stream {}", &message);

        let raw_response = Server::bad_request_response(message);
        let boxed_stream = stream.write(raw_response.borrow());
        if boxed_stream.is_ok() {
            stream.flush().unwrap();
        };
    }

    boxed_read.unwrap();
    let request : &[u8] = &buffer;


    let boxed_request = Request::parse_request(request);
    if boxed_request.is_err() {
        let message = boxed_request.as_ref().err().unwrap().to_string();
        eprintln!("unable to parse request: {}", &message);

        let raw_response = Server::bad_request_response(message);
        let boxed_stream = stream.write(raw_response.borrow());
        if boxed_stream.is_ok() {
            stream.flush().unwrap();
        };
    }


    let request: Request = boxed_request.unwrap();

    let boxed_peer_addr = stream.get_ref().peer_addr();
    if boxed_peer_addr.is_err() {
        eprintln!("\nunable to read peer addr");
        return;
    }
    let peer_addr = boxed_peer_addr.unwrap();

    let (server_ip, server_port, _thread_count) = get_ip_port_thread_count();
    let client_ip = peer_addr.ip().to_string();
    let client_port = peer_addr.port() as i32;
    let request_allocation_size = get_request_allocation_size();

    let connection = ConnectionInfo {
        client: Address {
            ip: client_ip.to_string(),
            port: client_port
        },
        server: Address {
            ip: server_ip,
            port: server_port
        },
        request_size: request_allocation_size,
    };

    let app = App::new();
    let mut response = app.execute(&request, &connection).unwrap();

    response.headers.push(Header {
        name: Header::_REFERRER_POLICY.to_string(),
        value: "no-referrer, strict-origin-when-cross-origin".to_string()
    });


    response.headers.push(Header {
        name: Header::_CONTENT_SECURITY_POLICY.to_string(),
        value: "default-src https:; style-src https: 'unsafe-inline'".to_string()
    });

    response.headers.push(Header {
        name: Header::_STRICT_TRANSPORT_SECURITY.to_string(),
        value: "max-age=15768000".to_string()
    });


    let tcp_stream = stream.get_ref();
    let mut peer_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0,0,0,0)), 0);
    let boxed_peer_addr = tcp_stream.peer_addr();
    if boxed_peer_addr.is_ok() {
        peer_addr = boxed_peer_addr.unwrap()
    } else {
        eprintln!("\nunable to read peer addr");
    }

    let log_request_response = Log::request_response(&request, &response, &peer_addr);
    println!("{}", log_request_response);

    let raw_response = Response::generate_response(response, request);


    let boxed_stream = stream.write(raw_response.borrow());
    if boxed_stream.is_ok() {
        let boxed_flush = stream.flush();
        if boxed_flush.is_err() {
            eprintln!("{}", boxed_flush.err().unwrap().to_string());
            return;
        }
        boxed_flush.unwrap();
    };

}
