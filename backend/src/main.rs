#[macro_use]
extern crate rocket;
use common::Location;
use rocket::fs::NamedFile;
use serde_json::de::from_reader;
use std::fs::File;
use std::io;
use std::io::BufReader;

fn read_json() -> Vec<Location> {
    // TODO error handling
    let json_file_path: String =
        r"C:\Users\micha\Documents\rust_yew\goldilocks\static\locations_flat.json".to_string();
    let location_file = File::open(json_file_path).unwrap();
    let location_reader = BufReader::new(location_file);
    let locations: Vec<Location> = from_reader(location_reader).unwrap();

    return locations;
}

#[get("/all")]
fn loc_all() -> String {
    let locations = read_json();

    let j = serde_json::to_string(&locations).unwrap();

    j
}

#[get("/")]
async fn index() -> io::Result<NamedFile> {
    NamedFile::open("../frontend/dist/index.html").await
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/v1/locations", routes![loc_all])
        .mount("/", routes![index])
}
