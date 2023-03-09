use yew::prelude::*;

use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Location {
    pub id: usize,
    pub lat: f64,
    pub long: f64,
    pub name: String,
    pub url: String,
}

#[derive(Properties, PartialEq)]
pub struct LocationListProps {
    pub locations: Vec<Location>,
    pub on_click: Callback<Location>,
}

#[derive(Properties, PartialEq)]
pub struct LocationDetailsProps {
    pub location: Location,
}
