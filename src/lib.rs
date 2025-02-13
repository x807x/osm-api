mod elements;

pub mod prelude {
    pub use crate::elements::{Attributes, ElementRef, ElementType, Node, Relation, Tag, Way, OSM};
}

pub const API_URL: &str = "https://api.openstreetmap.org/api/0.6/";

pub struct Bbox {
    pub min_lat: f64,
    pub min_lon: f64,
    pub max_lat: f64,
    pub max_lon: f64,
}
