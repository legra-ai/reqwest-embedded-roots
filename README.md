# reqwest-embedded-roots

[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![CI][ci-badge]][ci-url]
[![License][license-badge]][license-url]
[![Downloads][downloads-badge]][downloads-url]

A [`reqwest`] client factory with Mozilla's CA roots compiled in and the
OS trust store layered on top.

`FROM scratch` and distroless container images have no `ca-certificates`
package, so a stock TLS client can't verify anything. This crate embeds
Mozilla's root bundle ([`webpki-roots`]) directly in the binary — updated
whenever you rebuild against a newer release — and then loads the
OS-native trust store *on top*, so the client also honours whatever the
operator trusts system-wide: a corporate CA, a local development root.
TLS runs on rustls with the `ring` provider; no OpenSSL anywhere.

```rust
# fn main() -> reqwest::Result<()> {
// A ready client:
let client = reqwest_embedded_roots::client()?;

// Or chain your own settings first:
let configured = reqwest_embedded_roots::client_builder()
    .user_agent("my-service/1.0")
    .build()?;
# let _ = (client, configured);
# Ok(())
# }
```

The embedded bundle always loads; native-store certificates that fail to
parse are skipped with a `tracing` debug event rather than failing the
build, so one malformed cert in a system store cannot take the client
down.

[`reqwest`]: https://docs.rs/reqwest
[`webpki-roots`]: https://docs.rs/webpki-roots

## License

Licensed under either of:

- Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE));
- MIT License ([`LICENSE-MIT`](LICENSE-MIT)).

## Links

[crates-badge]: https://img.shields.io/crates/v/reqwest-embedded-roots.svg
[crates-url]: https://crates.io/crates/reqwest-embedded-roots
[docs-badge]: https://docs.rs/reqwest-embedded-roots/badge.svg
[docs-url]: https://docs.rs/reqwest-embedded-roots
[ci-badge]: https://github.com/legra-ai/reqwest-embedded-roots/actions/workflows/ci.yml/badge.svg
[ci-url]: https://github.com/legra-ai/reqwest-embedded-roots/actions/workflows/ci.yml
[license-badge]: https://img.shields.io/crates/l/reqwest-embedded-roots.svg
[license-url]: https://github.com/legra-ai/reqwest-embedded-roots/blob/main/LICENSE-APACHE
[downloads-badge]: https://img.shields.io/crates/d/reqwest-embedded-roots.svg
[downloads-url]: https://crates.io/crates/reqwest-embedded-roots
