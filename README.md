# sear: signed/encrypted archive 📦<a href="https://www.iqlusion.io"><img src="https://storage.googleapis.com/iqlusion-production-web/img/logo/iqlusion-rings-sm.png" alt="iqlusion" width="24" height="24"></a>

[![Crate][crate-img]][crate-link]
[![Docs][docs-img]][docs-link]
[![Apache 2.0 License][license-image]][license-link]
![Rust 1.35+][rustc-image]
[![forbid(unsafe_code)][unsafe-image]][unsafe-link]
[![Build Status][build-image]][build-link]
[![Appveyor Status][appveyor-image]][appveyor-link]
[![Gitter Chat][gitter-image]][gitter-link]

An always-encrypted *tar*-like file archive format with support for Ed25519
digital signatures.

## What is sear?

`sear` is a command-line tool and Rust library for producing tar-like
archives containing multiple files and potentially preserving attributes
including file ownership, modes/permissions, access control lists,
SELinux security contexts, and extended attributes (a.k.a. xattrs).

Additionally, `sear` integrates functionality traditionally provided by
a separate additional encryption tool such as *gpg*. However, where *gpg*
attempts to be a one-size-fits-all encryption which includes a large number
of complicated features (web-of-trust security model, messaging/encrypted email
support), `sear` is laser-focused on encrypting and authenticating (via
digital signatures) archives of files.

## Installation

NOTE: `sear` is presently vaporware, so this won't do a whole lot yet.

1. [Install Rust] (1.35+)
2. Run `cargo install sear`

## File Format

*NOTE: This description is presently expert-oriented. We'll have a simpler
description up later!*

`sear` archives have the following high-level structure:

```
| file 1 | file 2 | file 3 | ... | file N | footer |
```

...where each of the files consist of segmented AEAD-encrypted ciphertexts of
the original file. No additional framing is added to files, although each
segment of a file includes an individual authentication tag (i.e. MAC).

### Encryption

When constructing the archive, all plaintexts are first concatenated, and then
encrypted as a single message stream, under a single key/nonce. This means
individual segments may span multiple files - a separate stream per file
is NOT used. This provides the most space efficient means of storing files,
and can gracefully handle many small files without adding an undue number
of authentication tags.

Segmented AEAD encryption allows for streaming encryption/decryption of
individual files and archives, and also seekability within the archive.
To facilitate such encryption securely, a construction from the new
[Google Tink] cryptography is leveraged, which combines the following:

- [HKDF] key derivation
- [AES-GCM] encryption
- [STREAM] segmented AEAD construction

The STREAM construction has a rigorous and provable security definition:
it provides a Nonce-based Online Authenticated Encryption (nOAE) scheme
and defends against reordering and truncation attacks which are often
possible with naive streaming encryption schemes. However, it also provides
seekability, allowing individual files within the archive to be decrypted,
in addition to seeking within those files.

### Metadata

File metadata is buffered during archive creation, and serialized at the
end of the file as a footer using [Protocol Buffers].

The footer itself is split into an encrypted portion at the beginning followed
by a minimal plaintext portion at the very end of the file. It contains the
following attributes - ones with ℰ next to them are in the encrypted portion
of the footer:

- **UUID:** random identifier for this file, and also the nonce for encryption.
- **Chunk size:** granularity at which streaming encryption/decryption occurs.
  Files are split apart into fixed-sized chunks prior to encryption.
- **Encryption key fingerprint:** (optional) fingerprint of the encryption key
  as a [CryptoURI].
- **Signing key fingerprint:** (optional) fingerprint of the signing key as a
  [CryptoURI].
- **Signature:** (optional) a signature over the contents of the file. See
  below for more information on how this is computed.
- **Creator:** (optional, ℰ) username and hostname where archive was created
- **Date:** (ℰ) timestamp for when the archive was created
- **File attributes:** (ℰ) each entry in the file can have the following
  attributes:
  - **Path:** location of the file
  - **Length:** length of the file in bytes. Offsets within the ciphertext
    are computed as a running total of these values (and offset by the AEAD
    tags on each file segment).
  - **Owner:** username and groupname who own the file (TODO: UID/GID?)
  - **Permissions:** access control attributes consisting of the following:
    - **UNIX mode:** the `chmod`-style mode of the file with user, group, and
      world permission attributes
    - **POSIX ACLs:** expressive ACLs on file ownership
    - **SELinux Labels:** SELinux policy-related metadata
    - **xattr:** extended attributes

### Signatures

Signatures are optional, and computed over a [Merkle tree] of the ciphertexts
of the message segments (note that each message segment is further
authenticated by an AEAD tag). This allows for the signature to authenticate
any individual segment within the archive without the entire file being
present on disk.

Signature keys are generated and stored as [CryptoURI]s. The only signature
algorithm presently supported by this tool is Ed25519. One of the goals for
the tool is to allow signatures to be computed by a [YubiHSM2], allowing the
signature to be hardware-backed.

### Encryption Keys

`sear` supports the following keys, which are all serialized in [CryptoURI]
format:

- **Symmetric:** raw input key material for Tink HKDF-AES-GCM-STREAM
- **Asymmetric** derive IKM from static public key + ephemeral scalar using
  a Noise `NK`-like key exchange pattern
- **Password:** generate and store a random salt, and use it together with
  the password as input to Argon2i to derive a password.

## Code of Conduct

We abide by the [Contributor Covenant][cc] and ask that you do as well.

For more information, please see [CODE_OF_CONDUCT.md].

## License

Copyright © 2019 iqlusion

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you shall be dual licensed as above,
without any additional terms or conditions.

[//]: # (badges)

[crate-img]: https://img.shields.io/crates/v/sear.svg
[crate-link]: https://crates.io/crates/sear
[docs-img]: https://docs.rs/sear/badge.svg
[docs-link]: https://docs.rs/sear/
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusioninc/sear/blob/develop/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-1.35+-blue.svg
[unsafe-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[unsafe-link]: https://internals.rust-lang.org/t/disabling-unsafe-by-default/7988
[build-image]: https://travis-ci.com/iqlusioninc/sear.svg?branch=develop
[build-link]: https://travis-ci.com/iqlusioninc/sear
[appveyor-image]: https://ci.appveyor.com/api/projects/status/k9vd433ks173fqf2?svg=true
[appveyor-link]: https://ci.appveyor.com/project/tony-iqlusion/sear
[gitter-image]: https://badges.gitter.im/iqlusioninc/sear.svg
[gitter-link]: https://gitter.im/iqlusioninc/community

[//]: # (general links)

[Install Rust]: https://www.rust-lang.org/en-US/install.html
[Google Tink]: https://github.com/google/tink
[HKDF]: https://en.wikipedia.org/wiki/HKDF
[AES-GCM]: https://en.wikipedia.org/wiki/Galois/Counter_Mode
[STREAM]: https://web.cs.ucdavis.edu/~rogaway/papers/oae.pdf
[Protocol Buffers]: https://developers.google.com/protocol-buffers/
[CryptoURI]: https://github.com/cryptouri/cryptouri.rs/blob/develop/README.md
[Merkle tree]: https://en.wikipedia.org/wiki/Merkle_tree
[YubiHSM2]: https://developers.yubico.com/YubiHSM2/
[cc]: https://contributor-covenant.org
[CODE_OF_CONDUCT.md]: https://github.com/iqlusioninc/sear/blob/develop/CODE_OF_CONDUCT.md
