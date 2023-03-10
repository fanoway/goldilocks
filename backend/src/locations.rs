use common::Location;
use serde_json::de::from_reader;
use std::fs::File;
use std::io::BufReader;

pub fn read_json() -> Vec<Location> {
    // TODO error handling
    // TODO transition from lpocal file to KV worker
    let json_file_path: String =
        r"C:\Users\micha\Documents\rust_yew\goldilocks\static\locations_flat.json".to_string();
    let location_file = File::open(json_file_path).unwrap();
    let location_reader = BufReader::new(location_file);
    let locations: Vec<Location> = from_reader(location_reader).unwrap();

    return locations;
}
