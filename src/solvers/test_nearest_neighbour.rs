#[cfg(test)]
mod nearest_neighbours {
    use crate::{data_model::model::City, solvers::nearest_neighbours};

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
        let result = nearest_neighbours::solve(vec![], 1);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn given_one_city_returns_that_city() {
        let cities = vec![city("1", 0, 0, 1)];
        let result = nearest_neighbours::solve(cities, 1);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "1");
    }

    #[test]
    fn two_cities_returns_path_1_2_1() {
        let cities = vec![city("1", 0, 0, 1), city("2", 1, 1, 1)];
        let result = nearest_neighbours::solve(cities, 10);

        assert_eq!(result, vec!["1", "2", "1"])
    }

    #[test]
    fn three_cities_returns_path_1_3_2_1() {
        let cities = vec![city("1", 0, 0, 1), city("2", 2, 2, 1), city("3", 1, 1, 1)];
        let result = nearest_neighbours::solve(cities, 10);

        assert_eq!(result, vec!["1", "3", "2", "1"])
    }
    #[test]
    fn three_cities_with_capacity_1_3_1_2_1() {
        let cities = vec![city("1", 0, 0, 0), city("2", 2, 2, 1), city("3", 1, 1, 1)];
        let result = nearest_neighbours::solve(cities, 1);

        assert_eq!(result, vec!["1", "3", "1", "2", "1"])
    }
}
