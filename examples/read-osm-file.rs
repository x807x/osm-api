use osm_api::prelude::*;
use std::fs;

use anyhow::Result;
use quick_xml::de::from_reader;

fn main() -> Result<()> {
    let file = fs::File::open("test-datas/map.osm")?;
    let reader = std::io::BufReader::new(file);
    //    let reader = Reader::from_reader(reader);
    let osm: OSM = from_reader(reader)?;
    println!("{:#?}", osm);

    Ok(())
}
