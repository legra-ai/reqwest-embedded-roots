#![doc = include_str!("../README.md")]

use std::sync::Arc;

/// Returns a [`reqwest::ClientBuilder`] pre-configured with Mozilla's root CA
/// certificates from [`webpki_roots`], the OS-native trust store layered on
/// top, and the `ring` crypto provider.
///
/// Callers chain additional settings (timeouts, headers, proxies, …) before
/// calling `.build()`.
///
/// # Panics
///
/// Panics if the default TLS protocol versions are rejected by the
/// provider — this cannot happen with `ring`'s built-in defaults.
pub fn client_builder() -> reqwest::ClientBuilder {
    let root_store = Arc::new(root_cert_store());
    let tls = rustls::ClientConfig::builder_with_provider(Arc::new(
        rustls::crypto::ring::default_provider(),
    ))
    .with_safe_default_protocol_versions()
    .expect("ring supports all safe default protocol versions")
    .with_root_certificates(root_store)
    .with_no_client_auth();
    reqwest::Client::builder().tls_backend_preconfigured(tls)
}

/// Returns a [`reqwest::Client`] with embedded Mozilla root CA certificates
/// and the OS-native trust store layered on top.
///
/// Shorthand for `client_builder().build()`.
///
/// # Errors
///
/// Returns an error if the underlying TLS or HTTP configuration is invalid.
pub fn client() -> reqwest::Result<reqwest::Client> {
    client_builder().build()
}

/// Builds a [`rustls::RootCertStore`] with Mozilla's `webpki-roots` bundle
/// and the OS-native trust store on top.
///
/// Traffic to public hosts is signed by commercial CAs in the Mozilla
/// bundle, which ships compiled into the binary — so a container image needs
/// no `ca-certificates` package. Loading the OS store on top means the
/// client also respects whatever the operator has decided to trust
/// system-wide (an internal CA, a local development root), without the
/// application needing to know any on-disk certificate path.
fn root_cert_store() -> rustls::RootCertStore {
    let mut store = rustls::RootCertStore::empty();
    store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let result = rustls_native_certs::load_native_certs();
    for err in &result.errors {
        tracing::debug!(error = %err, "failed to load a native CA cert");
    }
    let (added, ignored) = store.add_parsable_certificates(result.certs);
    tracing::debug!(
        added,
        ignored,
        "loaded native CA certs into the rustls root store"
    );

    store
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_store_carries_at_least_the_embedded_bundle() {
        let store = root_cert_store();
        assert!(
            store.len() >= webpki_roots::TLS_SERVER_ROOTS.len(),
            "store must contain every embedded root (got {}, embedded {})",
            store.len(),
            webpki_roots::TLS_SERVER_ROOTS.len(),
        );
    }

    #[test]
    fn the_client_builds_without_a_runtime() {
        client().expect("client must build from the embedded configuration");
    }

    #[test]
    fn the_builder_accepts_further_configuration() {
        client_builder()
            .user_agent("reqwest-embedded-roots-test")
            .build()
            .expect("configured client must build");
    }
}
