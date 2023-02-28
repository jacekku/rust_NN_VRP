mod data_model;
mod solvers;
use serde_json::Result;
use std::fs;

use crate::data_model::model::{Cities, Output};

fn main() {
    println!("Hello, world!");
    solve().unwrap();
}

fn solve() -> Result<()> {
    let file = fs::read_to_string("./resources/E-n101-k8.vrp.json").unwrap();
    let data = file.as_str();
    let cities: Cities = serde_json::from_str(data)?;

    println!("Data is from {}", cities.header.name);

    let coords = cities.cities.clone();

    let route = solvers::nearest_neighbours::solve(cities.cities, cities.header.capacity);

    let output = Output {
        coords: coords.to_vec(),
        route: route.to_vec(),
    };
    fs::write("results.json", serde_json::to_string(&output).unwrap()).unwrap();

    Ok(())
}
