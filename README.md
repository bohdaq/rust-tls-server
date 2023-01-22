# rust-tls-server

**rust-tls-server** is a web server capable of serving static content over https.


## Features
1. HTTPS via [Mozilla Intermediate TLS](https://wiki.mozilla.org/Security/Server_Side_TLS)
1. [Self-signed certificate](https://en.wikipedia.org/wiki/Self-signed_certificate) out of the box
1. [Strict Transport Security](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Strict-Transport-Security)
1. [Content Security Policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Security-Policy) is set to serve 
assets only from https sources. Inline javascript is forbidden
1. [Referer Policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Referrer-Policy) is set
to not send any referer information
1. [Cross-Origin Resource Sharing (CORS)](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS)
1. [HTTP Range Requests](https://developer.mozilla.org/en-US/docs/Web/HTTP/Range_requests)
1. [HTTP Client Hints](https://developer.mozilla.org/en-US/docs/Web/HTTP/Client_hints)
1. [X-Content-Type-Options](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Content-Type-Options)
1. [X-Frame-Options](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Frame-Options)
1. Incoming request and outgoing response logging
1. [Symlinks](https://en.wikipedia.org/wiki/Symbolic_link)

## Download
[Download binary](https://github.com/bohdaq/rust-tls-server/releases) from releases page.  There is a mirror for downloads on [Google Drive](https://drive.google.com/drive/folders/1m0GyfvSaKROutjWeVLg23VBCbqZn7OkW?usp=sharing).

## Installation
Open [INSTALL](INSTALL.md) for details.

## Development
Open [DEVELOPER](DEVELOPER.md) for details.

## Configuration
Open [CONFIGURE](CONFIGURE.md) for details.

## Frequently Asked Questions
Open [FAQ](FAQ.md) for details.

## Community
Use GitHub discussions, issues and pull requests.

There is Rust Web Server [Discord](https://discord.gg/zaErjtr5Dm) where you can ask questions and share ideas. 

Follow the [Rust code of conduct](https://www.rust-lang.org/policies/code-of-conduct).

## Donations
If you appreciate my work and want to support it, feel free to do it via [PayPal](https://www.paypal.com/donate/?hosted_button_id=7J69SYZWSP6HJ).

## Links
1. [Rust Web Server](https://github.com/bohdaq/rust-web-server)
1. [http-to-https-letsencrypt](https://github.com/bohdaq/http-to-https-letsencrypt)
1. [Rust Web Framework](https://github.com/bohdaq/rust-web-framework/)
1. [Create Debian Package](https://github.com/bohdaq/rws-create-deb)
1. [Create RPM Package](https://github.com/bohdaq/rws-rpm-builder)
1. [Homebrew Formula](https://github.com/bohdaq/homebrew-rust-tls-server)
1. [crypto-ext](https://github.com/bohdaq/crypto-ext/)
1. [file-ext](https://github.com/bohdaq/file-ext/)