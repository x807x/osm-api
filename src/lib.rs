mod apis;
mod elements;
mod error;

pub mod prelude {
    pub use crate::{apis::*, elements::*, error::*};
}

#[cfg(not(debug_assertions))]
pub const API_URL: &str = "https://api.openstreetmap.org";

#[cfg(debug_assertions)]
pub const API_URL: &str = "https://master.apis.dev.openstreetmap.org";

pub struct Bbox {
    pub min_lat: f64,
    pub min_lon: f64,
    pub max_lat: f64,
    pub max_lon: f64,
}
