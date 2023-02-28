
use crate::data_model::model::City;

fn distance(city1: &City, city2: &City) -> f64 {
    (((city1.x - city2.x).pow(2) as f64) + ((city1.y - city2.y).pow(2) as f64)).sqrt()
}

pub(crate) fn solve(cities: Vec<City>, capacity: usize) -> Vec<String> {
    if cities.len() == 0 {
        return vec![];
    }
    if cities.len() == 1 {
        return vec![cities[0].id.clone()];
    }
    let mut current_capacity = capacity;
    let mut visited = Vec::new();
    for _ in 0..cities.len() {
        visited.push(false);
    }

    let mut output = vec![];
    let mut current_city_idx = 0;
    while visited.contains(&false) {
        visited[current_city_idx] = true;
        let current_city = cities.get(current_city_idx).unwrap();
        output.push(current_city.id.clone());

        let distance_matrix: Vec<(usize, f64)> = cities
            .iter()
            .enumerate()
            .filter(|&(index, _)| !visited[index])
            .map(|(index, city)| (index, distance(city, current_city)))
            .collect();
        let nearest = distance_matrix
            .iter()
            .min_by(|&(_, dist1), &(_, dist2)| dist1.total_cmp(dist2));
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

    return output;
}
