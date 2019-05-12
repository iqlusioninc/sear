# sear: signed/encrypted archive 📦<a href="https://www.iqlusion.io"><img src="https://storage.googleapis.com/iqlusion-prod-web-assets/img/logo/iqlusion-rings-sm.png" alt="iqlusion" width="24" height="24"></a>

An always-encrypted *tar*-like file archive format with support for Ed25519
digital signatures.

## What is sear?

**sear** is a command-line tool and Rust library for producing tar-like
archives containing multiple files and potentially preserving attributes
including file ownership, modes/permissions, access control lists,
SELinux security contexts, and extended attributes (a.k.a. xattrs).

Additionally, **sear** integrates functionality traditionally provided by
a separate additional encryption tool such as *gpg*. However, where *gpg*
attempts to be a one-size-fits-all encryption which includes a large number
of complicated features (web-of-trust security model, messaging/encrypted email
support), **sear** is laser-focused on encrypting and authenticating (via
digital signatures) archives of files.

## Installation

1. [Install Rust] 
2. Run `cargo install sear`

[Install Rust]: https://www.rust-lang.org/en-US/install.html
