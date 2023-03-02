use std::{cmp, sync::Arc};

use rand::{distributions::WeightedIndex, prelude::Distribution, thread_rng};

use crate::data_model::model::City;

fn distance(city1: &City, city2: &City) -> f64 {
    (((city1.x - city2.x).pow(2) as f64) + ((city1.y - city2.y).pow(2) as f64)).sqrt()
}

pub(crate) struct Distances {
    distances: Vec<Vec<f64>>,
}
impl Distances {
    pub(crate) fn new(cities: &Vec<City>) -> Self {
        let cities_len = cities.len();
        let mut distances = Vec::new();
        for i in 0..cities_len {
            let mut distances_for_city = Vec::new();
            for j in 0..cities_len {
                let dist = distance(&cities[i], &cities[j]);
                distances_for_city.push(dist);
            }
            distances.push(distances_for_city);
        }

        Distances { distances }
    }

    fn get_distance(&self, city_idx_1: usize, city_idx_2: usize) -> f64 {
        self.distances[city_idx_1][city_idx_2]
    }
}

pub(crate) fn ant_walk(
    cities: &Vec<City>,
    capacity: usize,
    distances: &Distances,
    pheromones: &Vec<Vec<f64>>,
) -> Vec<String> {
    let mut visited = Vec::new();
    for _ in 0..cities.len() {
        visited.push(false);
    }
    let mut current_capacity = capacity;
    let mut output = vec![];
    let mut current_city_idx = 0;
    while visited.contains(&false) {
        visited[current_city_idx] = true;
        let current_city = cities.get(current_city_idx).unwrap();
        output.push(current_city.id.clone());

        let distance_matrix = &distances.distances[current_city_idx];

        let weights: Vec<(usize, f64)> = distance_matrix
            .iter()
            .enumerate()
            .filter(|&(index, _)| !visited[index])
            .map(|(index, dist)| {
                let pheromone: f64 = 'ph: {
                    let one = pheromones.get(current_city_idx);
                    match one {
                        Some(val) => {
                            let two = val.get(index);
                            match two {
                                Some(value) => break 'ph *value,
                                None => break 'ph 1.0,
                            }
                        }
                        None => break 'ph 1.0,
                    }
                };
                (index, (1.0 / (dist)) * pheromone)
            })
            .collect();

        let nearest: Option<(usize, f64)> = 'bar: {
            let res_dist = WeightedIndex::new(weights.iter().map(|item| item.1));
            if let Ok(dist) = res_dist {
                let mut rng = thread_rng();
                break 'bar Some(weights[dist.sample(&mut rng)]);
            }
            None
        };

        if let Some(value) = nearest {
            let other_idx = value.0;
            let other_city = &cities[other_idx];
            if other_city.demand > current_capacity {
                current_city_idx = 0;
                current_capacity = capacity;
            } else {
                current_city_idx = other_idx;
                current_capacity -= other_city.demand;
            }
        }
    }

    let city = &cities[0];
    output.push(city.id.clone());

    output
}

pub(crate) fn solve(cities: Vec<City>, capacity: usize) -> Vec<String> {
    if cities.len() == 0 {
        return Vec::new();
    }
    if cities.len() == 1 {
        return vec![cities[0].id.clone()];
    }
    let distances = Distances::new(&cities);
    let mut pheromones = {
        let cities_len = cities.len();
        let mut ph = Vec::new();
        for _ in 0..cities_len {
            let mut ph_for_city = Vec::new();
            for _ in 0..cities_len {
                let phmns = 1.0;
                ph_for_city.push(phmns);
            }
            ph.push(ph_for_city);
        }
        ph
    };
    let mut route = Vec::new();

    for _ in 0..1000 {
        let mut routes = Vec::new();
        // let mut handles = Vec::new();

        for _ in 0..150 {
            // let h =
            //     std::thread::spawn(move || ant_walk(&cities, capacity, &distances, &pheromones));
            // handles.push(h);
            routes.push(ant_walk(&cities, capacity, &distances, &pheromones));
        }

        // while let Some(h) = handles.pop() {
        //     h.join().unwrap();
        // }
        route = get_shortest_route(&routes, &distances, &cities);
        pheromones = evaporate_pheromones(pheromones, 0.85);
        pheromones = add_pheromones(pheromones, &route, 100.0, &cities);
    }
    return route;
}

pub(crate) fn evaporate_pheromones(mut matrix: Vec<Vec<f64>>, evaporation: f64) -> Vec<Vec<f64>> {
    for row in &mut matrix {
        for index in 0..row.len() {
            row[index] *= evaporation;
        }
    }
    return matrix;
}

pub(crate) fn add_pheromones(
    mut matrix: Vec<Vec<f64>>,
    route: &Vec<String>,
    pheromones_spread: f64,
    cities: &Vec<City>,
) -> Vec<Vec<f64>> {
    if route.len() < 2 {
        return matrix;
    }

    let mut route_matrix: Vec<Vec<usize>> = Vec::new();
    for row in 0..matrix.len() {
        route_matrix.push(Vec::new());
        for _ in 0..matrix[row].len() {
            route_matrix[row].push(0);
        }
    }
    let cities_indexes: Vec<(usize, String)> = cities
        .iter()
        .enumerate()
        .map(|(index, city)| (index, city.id.clone()))
        .collect();
    for idx in 0..route.len() - 1 {
        let city1opt = &cities_indexes.iter().find(|(_, id)| id.eq(&route[idx]));
        let city2opt = &cities_indexes.iter().find(|(_, id)| id.eq(&route[idx + 1]));
        if let (Some(city1), Some(city2)) = (city1opt, city2opt) {
            route_matrix[city1.0][city2.0] += 1;
        }
    }

    let pheromones = pheromones_spread / (route.len() - 1) as f64;

    for row in 0..matrix.len() {
        for index in 0..matrix[row].len() {
            matrix[row][index] += pheromones * route_matrix[row][index] as f64;
        }
    }
    return matrix;
}

pub(crate) fn get_shortest_route(
    routes: &Vec<Vec<String>>,
    distances: &Distances,
    cities: &Vec<City>,
) -> Vec<String> {
    let cities_indexes: Vec<(usize, String)> = cities
        .iter()
        .enumerate()
        .map(|(index, city)| (index, city.id.clone()))
        .collect();
    let mut route_lengths = Vec::new();

    for route in routes {
        let mut sum = 0.0;
        for idx in 0..route.len() - 1 {
            let city1opt = &cities_indexes.iter().find(|(_, id)| id.eq(&route[idx]));
            let city2opt = &cities_indexes.iter().find(|(_, id)| id.eq(&route[idx + 1]));
            if let (Some(city1), Some(city2)) = (city1opt, city2opt) {
                sum += distances.get_distance(city1.0, city2.0);
            }
        }
        route_lengths.push((route, sum));
    }
    if let Some(min_route) = route_lengths
        .iter()
        .min_by(|(_, r1), (_, r2)| r1.total_cmp(r2))
    {
        println!("{}", min_route.1);
        return min_route.0.to_vec();
    }

    return Vec::new();
}
