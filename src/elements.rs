use log::debug;
use std::{default, str::FromStr};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

/// Node is a point in space.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    #[serde(rename = "@id")]
    pub id: i64,
    #[serde(rename = "@lat")]
    pub lat: f64,
    #[serde(rename = "@lon")]
    pub lon: f64,
    #[serde(rename = "tag", default)]
    pub tags: Vec<Tag>,
    #[serde(flatten)]
    pub attributes: Attributes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeRef {
    #[serde(rename = "@ref")]
    pub reference: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Way {
    #[serde(rename = "@id")]
    pub id: i64,
    #[serde(rename = "nd", default)]
    pub nodes: Vec<NodeRef>,
    #[serde(rename = "tag", default)]
    pub tags: Vec<Tag>,
    #[serde(flatten)]
    pub attributes: Attributes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementRef {
    #[serde(rename = "@type")]
    element_type: ElementType,
    #[serde(rename = "@ref")]
    reference: i64,
    #[serde(rename = "@role")]
    role: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ElementType {
    Node,
    Way,
    Relation,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    #[serde(rename = "member", default)]
    pub members: Vec<ElementRef>,
    #[serde(rename = "tag", default)]
    pub tags: Vec<Tag>,
    #[serde(flatten)]
    pub attributes: Attributes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "@k")]
    pub key: String,
    #[serde(rename = "@v")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attributes {
    #[serde(rename = "@user")]
    pub user: String,
    #[serde(rename = "@uid", deserialize_with = "de_from_str")]
    pub uid: i64,
    #[serde(rename = "@timestamp")]
    pub timestamp: TimeStamp,
    #[serde(rename = "@visible", deserialize_with = "de_from_str")]
    pub visible: bool,
    #[serde(rename = "@version", deserialize_with = "de_from_str")]
    pub version: u8,
    #[serde(rename = "@changeset", deserialize_with = "de_from_str")]
    pub changeset: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "osm")]
pub struct OSM {
    #[serde(alias = "@version", default)]
    pub version: String,
    #[serde(alias = "@generator")]
    pub generator: String,
    #[serde(rename = "@timestamp")]
    pub time_stamp_osm_base: Option<TimeStamp>,
    #[serde(alias = "@copyright")]
    pub copyright: String,
    #[serde(alias = "@license")]
    pub license: String,
    #[serde(alias = "@attribution")]
    pub attribution: String,
    #[serde(rename = "@error")]
    pub error: Option<String>,
    #[serde(rename = "node", default)]
    pub node: Vec<Node>,
    #[serde(rename = "way", default)]
    pub way: Vec<Way>,
    #[serde(rename = "relation", default)]
    pub relation: Vec<Relation>,
    pub api: Option<Api>,
}

type TimeStamp = DateTime<Utc>;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Api {
    #[serde(alias = "versions", deserialize_with = "de_inner_version")]
    pub version: ApiVersion,
    #[serde(alias = "area", deserialize_with = "de_inner_maximum", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_areas: Option<f64>,
    #[serde(alias = "note_area", deserialize_with = "de_inner_maximum", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_note_areas: Option<i64>,
    #[serde(alias = "tracepoints", deserialize_with = "de_inner_per_page", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trackpoints_per_page: Option<i64>,
    #[serde(alias = "waynodes", deserialize_with = "de_inner_maximum", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_way_nodes: Option<i64>,
    #[serde(
        alias = "relationmembers",
        deserialize_with = "de_inner_maximum",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_relation_members: Option<i64>,
    #[serde(alias = "changesets", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changesets: Option<QueryLimit>,
    #[serde(alias = "notes", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<QueryLimit>,
    #[serde(alias = "timeout", deserialize_with = "de_inner_seconds", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    #[serde(alias = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ApiStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionRange {
    #[serde(alias = "@minimum")]
    pub min: String,
    #[serde(alias = "@maximum")]
    pub max: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Online,
    #[serde(alias = "readonly")]
    ReadOnly,
    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiStatus {
    #[serde(alias = "@database")]
    pub database: Status,
    #[serde(alias = "@api")]
    pub api: Status,
    #[serde(alias = "@gpx")]
    pub gpx: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryLimit {
    #[serde(alias = "@default_query_limit", alias = "default_query_limit")]
    pub default: i64,
    #[serde(alias = "@maximum_query_limit", alias = "maximum_query_limit")]
    pub max: i64,
    #[serde(alias = "@maximum_elements", alias = "maximum_elements")]
    pub max_elements: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ApiVersion {
    Versions(Vec<String>),
    VersionRange(String, String),
    #[default]
    None,
}

fn de_inner_version<'de, D>(deserializer: D) -> Result<ApiVersion, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    debug!("value: {:?}", value);
    match value {
        Value::Array(versions) => Ok(ApiVersion::Versions(
            versions.iter().map(|v| v.as_str().expect("Not a valid string").to_string()).collect(),
        )),
        Value::Object(obj) => {
            if let Some(version) = obj.get("$text") {
                return Ok(ApiVersion::Versions(vec![version.to_string()]));
            }
            let max = obj.get("@maximum")
                .or_else(|| obj.get("maximum"))
                .expect("Missing maximum")
                .as_str()
                .expect("Not a valid string")
                .to_string();
            let min = obj.get("@minimum")
                .or_else(|| obj.get("minimum"))
                .expect("Missing minimum")
                .as_str()
                .expect("Not a valid string")
                .to_string();
            Ok(ApiVersion::VersionRange(max, min))
        },
        _ => Err(serde::de::Error::custom("Invalid version")),
    }
}

fn de_inner_maximum<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    #[derive(Deserialize)]
    struct Helper<T> {
        #[serde(alias = "@maximum")]
        maximum: Option<T>,
    }
    Ok(Helper::<T>::deserialize(deserializer)?.maximum)
}

fn de_inner_seconds<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    #[derive(Deserialize)]
    struct Helper<T> {
        #[serde(alias = "@seconds")]
        seconds: Option<T>,
    }
    Ok(Helper::<T>::deserialize(deserializer)?.seconds)
}

fn de_inner_per_page<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    #[derive(Deserialize)]
    struct Helper<T> {
        #[serde(alias = "@per_page")]
        per_page: Option<T>,
    }
    Ok(Helper::<T>::deserialize(deserializer)?.per_page)
}

fn de_min_max<'de, D, T>(deserializer: D) -> Result<Option<(T, T)>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    #[derive(Deserialize)]
    struct Helper<T> {
        #[serde(alias = "@minimum")]
        minimum: T,
        #[serde(alias = "@maximum")]
        maximum: T,
    }
    match Helper::<T>::deserialize(deserializer) {
        Ok(helper) => Ok(Some((helper.minimum, helper.maximum))),
        Err(_) => Ok(None),
    }
}

fn de_from_str<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}
