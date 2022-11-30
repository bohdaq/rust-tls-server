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
1. All other features from [rust-web-server](https://github.com/bohdaq/rust-web-server/blob/main/README.md#features) (excluding #6)

## Download
[Download binary](https://github.com/bohdaq/rust-tls-server/releases) from releases page.  There is a mirror for downloads on [Google Drive](https://drive.google.com/drive/folders/1m0GyfvSaKROutjWeVLg23VBCbqZn7OkW?usp=sharing).

## Installation
> $ sudo cp rts /usr/local/bin
>
> $ sudo chmod +x /usr/local/bin/rts

## Run
Simply run the following from command line:

> $ rts --ip=127.0.0.1 --port=8888 --threads=100

## Configuration

The rws can be started without any configuration. The following is the default config - the server will bind to IP 127.0.0.1 and port 7887, will spawn 200 threads, CORS requests are allowed.

The rws will try to read configuration from [system environment variables](https://github.com/bohdaq/rust-web-server/blob/main/rws.variables) first, then it will override configuration by reading it from file named [rws.config.toml](https://github.com/bohdaq/rust-web-server/blob/main/rws.config.toml) placed in the same directory where you execute rws, at last it will apply config provided via [command-line arguments](https://github.com/bohdaq/rust-web-server/blob/main/rws.command_line).

I personally prefer to use system environment variables, as once it is set correctly, they are hard to break accidentally by overwriting config, or each time providing command line arguments during restarts.

There may be a use case when you need to run more than one instance, in such a case config file per instance or command line configuration is an option. 
