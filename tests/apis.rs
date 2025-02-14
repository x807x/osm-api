use osm_api::prelude::*;
#[tokio::test]
async fn test_get_versions() -> Result<()> {
    let result = get_versions().await;
    assert!(result.is_ok());

    Ok(())
}

#[tokio::test]
async fn test_get_capabilities() -> Result<()> {
    let result = get_capabilities().await;
    assert!(result.is_ok());

    Ok(())
}
