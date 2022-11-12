use openssl::ssl::{SslMethod, SslAcceptor, SslStream, SslFiletype};
use std::net::{TcpStream, TcpListener};
use std::sync::Arc;
use std::thread;
use rust_web_server::server::Server;

fn main() {
    println!("Rust TLS Server | Draft | Work in Progress");

    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    // DO NOT PUT KEY AND CERTIFICATE IN THE FOLDER WHERE YOU ARE RUNNING SERVER!!!
    acceptor.set_private_key_file("/Users/bogdantsap/git/key/selfsigned.key", SslFiletype::PEM).unwrap();
    acceptor.set_certificate_file("/Users/bogdantsap/git/key/selfsigned.crt", SslFiletype::PEM).unwrap();
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

fn handle_client(stream: SslStream<TcpStream>) {
    Server::process_request(stream);
}
