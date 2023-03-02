#[cfg(test)]
mod ant_colony {
    use crate::{
        data_model::model::City,
        solvers::ant_colony::{self, Distances},
    };

    fn city(id: &str, x: isize, y: isize, demand: usize) -> City {
        City {
            id: String::from(id),
            x,
            y,
            demand,
        }
    }

    #[test]
    fn given_no_cities_returns_empty_path() {
        let result = ant_colony::solve(vec![], 1);
        assert_eq!(result.len(), 0);
    }
    #[test]
    fn given_1_returns_1() {
        let cities = vec![city("1", 0, 0, 1)];
        let result = ant_colony::solve(cities, 1);
        assert_eq!(result.len(), 1);
        assert_eq!(result, vec!["1"]);
    }
    #[test]
    fn given_1_2_returns_1_2_1() {
        let cities = vec![city("1", 0, 0, 1), city("2", 1, 1, 1)];
        let result = ant_colony::solve(cities, 10);
        assert_eq!(result, vec!["1", "2", "1"]);
    }

    #[test]
    fn given_1_2_3_returns_1_3_2_1() {
        let cities = vec![city("1", 0, 0, 1), city("2", 0, 10, 1), city("3", 0, 1, 1)];
        let result = ant_colony::solve(cities, 10);
        assert_eq!(result, vec!["1", "3", "2", "1"]);
    }

    #[test]
    fn given_1_2_3_and_1_capacity_returns_1_3_1_2_1() {
        let cities = vec![city("1", 0, 0, 1), city("2", 10, 0, 1), city("3", 1, 0, 1)];
        let result = ant_colony::solve(cities, 1);
        assert_eq!(result, vec!["1", "3", "1", "2", "1"]);
    }

    #[test]
    fn given_vec_of_routes_and_distance_matrix_returns_shortest_route() {
        let cities = vec![city("1", 0, 0, 1), city("2", 10, 0, 1), city("3", 1, 0, 1)];
        let distances = Distances::new(&cities);

        fn t(s: &str) -> String {
            s.to_string()
        }
        let routes = vec![
            vec![t("1"), t("3"), t("1"), t("2"), t("1")],
            vec![t("1"), t("2"), t("1"), t("3"), t("1")],
        ];
        let result = ant_colony::get_shortest_route(&routes, &distances, &cities);
        assert_eq!(result, vec!["1", "3", "1", "2", "1"]);
    }

    mod ant {
        use crate::solvers::{
            ant_colony::{self, Distances},
            test_ant_colony::ant_colony::city,
        };

        #[test]
        fn given_1_3_2_and_pheromones_1_2_3_returns_1_3_2_1() {
            let cities = vec![city("1", 0, 0, 1), city("2", 2, 2, 1), city("3", 1, 1, 1)];
            let distances = Distances::new(&cities);
            let pheromones: Vec<Vec<f64>> = vec![
                vec![0.0, 0.0, 1.0],
                vec![1.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0],
            ];
            let result = ant_colony::ant_walk(&cities, 10, &distances, &pheromones);
            assert_eq!(result, vec!["1", "3", "2", "1"]);
        }

        #[test]
        fn given_1_3_2_and_pheromones_1_2_3_and_capacity_1_returns_1_3_1_2_1() {
            let cities = vec![city("1", 0, 0, 1), city("2", 2, 2, 1), city("3", 1, 1, 1)];
            let distances = Distances::new(&cities);
            let pheromones: Vec<Vec<f64>> = vec![
                vec![0.0, 0.0001, 1.0],
                vec![1.0, 0.0, 0.0],
                vec![1.0, 1.0, 0.0],
            ];
            let result = ant_colony::ant_walk(&cities, 1, &distances, &pheromones);
            assert_eq!(result, vec!["1", "3", "1", "2", "1"]);
        }
    }

    mod update_pheromones {
        use std::vec;

        use crate::solvers::{ant_colony, test_ant_colony::ant_colony::city};

        #[test]
        fn given_empty_matrix_returns_matrix() {
            let matrix = vec![vec![]];
            let expected: Vec<Vec<f64>> = vec![vec![]];

            let result = ant_colony::evaporate_pheromones(matrix, 1.0);

            assert_eq!(result, expected);
        }

        #[test]
        fn given_matrix_and_evaporation_constant_returns_with_evaporated_values() {
            let matrix = vec![vec![1.0]];
            let evaporation_constant = 0.5;
            let result = ant_colony::evaporate_pheromones(matrix, evaporation_constant);

            assert_eq!(result, vec![vec![0.5]]);
        }

        #[test]
        fn given_matrix_and_empty_route_returns_matrix() {
            let matrix = vec![vec![1.0]];
            let route = vec![];
            let result = ant_colony::add_pheromones(matrix, &route, 0.0, &vec![]);

            assert_eq!(result, vec![vec![1.0]]);
        }

        #[test]
        fn given_matrix_and_1_route_returns_matrix() {
            let matrix = vec![vec![1.0]];
            let route = vec![String::from("1")];
            let result = ant_colony::add_pheromones(matrix, &route, 1.0, &vec![city("1", 0, 0, 1)]);

            assert_eq!(result, vec![vec![1.0]]);
        }

        #[test]
        fn given_matrix_and_1_2_1_route_returns_added_value_only_on_route() {
            let matrix = vec![vec![1.0, 1.0], vec![1.0, 1.0]];
            let route = vec![String::from("1"), String::from("2"), String::from("1")];
            let cities = vec![city("1", 0, 0, 1), city("2", 2, 2, 1)];
            let result = ant_colony::add_pheromones(matrix, &route, 2.0, &cities);

            assert_eq!(result, vec![vec![1.0, 2.0], vec![2.0, 1.0]]);
        }
    }
}
