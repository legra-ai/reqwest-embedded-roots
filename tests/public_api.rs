//! Public-API integration test: clients build offline with the embedded
//! root store, from the shipped crate.

#[test]
fn ready_client_and_configured_builder_both_construct() -> reqwest::Result<()> {
    let client = reqwest_embedded_roots::client()?;
    let configured = reqwest_embedded_roots::client_builder()
        .user_agent("public-api-test/1.0")
        .build()?;
    let _ = (client, configured);
    Ok(())
}
