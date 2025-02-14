use osm_api::prelude::*;

#[tokio::test]
async fn test_get_versions_json() -> Result<()> {
    let result = get_versions_json().await;
    assert!(result.is_ok());

    Ok(())
}

#[tokio::test]
async fn test_get_capabilities_json() -> Result<()> {
    let result = get_capabilities_json().await;
    assert!(result.is_ok());

    Ok(())
}
