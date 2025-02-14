use log::debug;
use quick_xml::de::from_str as from_xml_str;
use reqwest::{get, StatusCode};
use serde_json::from_str as from_json_str;

use crate::{
    error::{Error, Result},
    prelude::OSM,
    API_URL,
};

pub enum Format {
    XML,
    JSON,
}
pub async fn get_with_url(url: &str, format: Format) -> Result<OSM> {
    debug!("GET {}", url);
    let response = get(url).await?;
    match response.status() {
        StatusCode::OK => {
            let text = response.text().await?;
            debug!("response\n{}", text);
            let osm: OSM = match format {
                Format::XML => from_xml_str(&text)?,
                Format::JSON => from_json_str(&text)?,
            };

            Ok(osm)
        }
        StatusCode::NOT_FOUND => Err(Error::Http(StatusCode::NOT_FOUND)),
        _ => {
            todo!("Handle other status codes")
        }
    }
}

pub async fn get_versions() -> Result<OSM> {
    get_with_url(format!("{}/api/versions", API_URL).as_str(), Format::XML).await
}

#[cfg(feature = "json-api")]
pub async fn get_versions_json() -> Result<OSM> {
    get_with_url(
        format!("{}/api/versions.json", API_URL).as_str(),
        Format::JSON,
    )
    .await
}

pub async fn get_capabilities() -> Result<OSM> {
    get_with_url(
        format!("{}/api/capabilities", API_URL).as_str(),
        Format::XML,
    )
    .await
}

pub async fn get_capabilities_json() -> Result<OSM> {
    get_with_url(
        format!("{}/api/capabilities.json", API_URL).as_str(),
        Format::JSON,
    )
    .await
}
