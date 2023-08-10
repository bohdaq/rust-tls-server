[Read Me](README.md) > Install

## Install

Make sure you removed previous executable:

> sudo rm -f /usr/local/bin/rts #old path

> sudo rm -f /usr/bin/rts

[Download binary](https://github.com/bohdaq/rust-tls-server/releases) for you platform from releases page.
There is a mirror for downloads on [Google Drive](https://drive.google.com/drive/folders/1m0GyfvSaKROutjWeVLg23VBCbqZn7OkW?usp=share_link).

## x86 64-bit

### Apple macOS
> sudo cp rts /usr/local/bin

> sudo chmod 777 /usr/local/bin/rts


#### Homebrew macOS
Open **[Rust TLS Server Homebrew formula](https://github.com/bohdaq/homebrew-rust-tls-server)** for details.


### Linux
> sudo cp rts /usr/bin

> sudo chmod 777 /usr/bin/rts


#### Debian
> sudo dpkg -i --force-overwrite rts.deb


#### RPM
Replace _YOUR_VERSION_ with version you downloaded.
> sudo rpm -i --force rts-_YOUR_VERSION_.rpm


#### Portage ebuild
Open **[Rust TLS Server Portage ebuild](https://github.com/bohdaq/rts-portage-ebuild)** for details.


#### Pacman package
Open **[Rust TLS Server Pacman package](https://github.com/bohdaq/rts-arch-package)** for details.

###  Windows
Copy executable to _C:\WINDOWS\system32_ folder.

## ARM 64-bit

### Linux
> sudo cp rts /usr/bin

> sudo chmod 777 /usr/bin/rts


#### Debian
> sudo dpkg -i --force-overwrite rts.deb


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