[Read Me](README.md) > Configuration

# Configuration Info
The rts can be started without any configuration. The following is the default config - the server will bind to IP 127.0.0.1 and port 7878, will spawn 200 threads, CORS requests are allowed.

The rts will try to read configuration from [system environment variables](https://github.com/bohdaq/rust-tls-server/blob/main/rts.variables) first, then it will override configuration
by reading it from file named [rts.config.toml](https://github.com/bohdaq/rust-tls-server/blob/main/rts.config.toml) placed in the same directory where you execute rts, at last it will
apply config provided via [command-line arguments](https://github.com/bohdaq/rust-tls-server/blob/main/rts.command_line).

I personally prefer to use system environment variables, as once it is set correctly, they are hard to break accidentally by overwriting config, or each time providing command line arguments
during restarts.

There may be a use case when you need to run more than one instance, in such a case config file per instance or command line configuration is an option. 

## Encryption

Server will provide self-signed certificate out of the box.

To do so it will need write permission to create private key and
certificate files.

However, this certificate will cause browser warning.

To resolve the warning you may use [Let's Encrypt](https://letsencrypt.org/) service.

There's [http-to-https-letsencrypt](https://github.com/bohdaq/rust-http-to-https-letsencrypt-acme) HTTP server with default redirect to HTTPS and support for Let'sEncrypt Automatic Certificate Management Environment using HTTP-01 challenge.

## Memory
As any other application, rts will allocate memory required to serve the request.
For example if the client will make an HTTP GET for resource which has size more
than free available memory on the running instance, rts will throw Out Of Memory error.

In such case valid options are:
1. Use range requests on the client for big resources to get a portion at a time.
2. Balance the overall load on instance in case you have heavy load by spinning up
   more rts instances and share traffic between them.