use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Cities {
    pub header: Header,
    pub cities: Vec<City>,
    pub list: Vec<String>,
}
#[derive(Serialize, Deserialize)]
pub(crate) struct Header {
    pub name: String,
    pub comment: String,
    pub dimension: usize,
    pub edge_weight_type: String,
    pub capacity: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct City {
    pub id: String,
    pub x: isize,
    pub y: isize,
    pub demand: usize,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Output {
    pub coords: Vec<City>,
    pub route: Vec<String>,
}
