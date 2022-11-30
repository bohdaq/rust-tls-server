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
