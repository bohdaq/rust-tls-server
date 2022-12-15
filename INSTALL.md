[Read Me](README.md) > Install

## Install
[Download binary](https://github.com/bohdaq/rust-tls-server/releases) for you platform from releases page.
There is a mirror for downloads on [Google Drive](https://drive.google.com/drive/folders/1m0GyfvSaKROutjWeVLg23VBCbqZn7OkW?usp=share_link).
### x86 64-bit Apple macOS
> sudo cp rts /usr/local/bin
>
> sudo chmod +x /usr/local/bin/rts
#### x86 64-bit Homebrew macOS
> brew tap bohdaq/rust-web-server
>
> brew install rts

### x86 64-bit Linux
> sudo cp rts /usr/local/bin
>
> sudo chmod +x /usr/local/bin/rts
#### x86 64-bit Debian
> sudo dpkg -i --force-overwrite rts.deb
#### x86 64-bit RPM
Replace _YOUR_VERSION_ with version you downloaded.
> sudo rpm -i --force rts-_YOUR_VERSION_.rpm

### ARM 64-bit Linux
> sudo cp rts /usr/local/bin
>
> sudo chmod +x /usr/local/bin/rts
#### ARM 64-bit Debian
> sudo dpkg -i --force-overwrite rts.deb

### x86 64-bit Windows
Copy executable to _C:\WINDOWS\system32_ folder.


### Testing installation
To check installation execute the following code in the terminal:

> $ rts

You will see similar output:

> Rust TLS Server
>
> Version:       YOUR_VERSION
>
> Authors:       Bohdan Tsap <bohdan.tsap@tutanota.com>
>
> Repository:    https://github.com/bohdaq/rust-web-server
>
> Desciption:    rust-tls-server (rts) is a static content web-server written in Rust
>
> Rust Version:  RUST_VERSION
> 
> ...
> Hello, rust-web-server is up and running: https://127.0.0.1:7878


Open browser, go to https://127.0.0.1:7878, you'll see default page.

Go back to terminal, press Ctrl + C (or CMD + C) to stop server.