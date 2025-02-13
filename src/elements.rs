use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

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
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "@generator")]
    pub generator: String,
    #[serde(rename = "@timestamp")]
    pub time_stamp_osm_base: Option<TimeStamp>,
    #[serde(rename = "@copyright")]
    pub copyright: String,
    #[serde(rename = "@license")]
    pub license: Option<String>,
    #[serde(rename = "@attribution")]
    pub attribution: Option<String>,
    #[serde(rename = "@error")]
    pub error: Option<String>,
    #[serde(rename = "node")]
    pub node: Vec<Node>,
    #[serde(rename = "way")]
    pub way: Vec<Way>,
    #[serde(rename = "relation")]
    pub relation: Vec<Relation>,
}

type TimeStamp = DateTime<Utc>;

fn de_from_str<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}
