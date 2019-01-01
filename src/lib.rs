pub mod geokdbush;
pub mod kdbush;

#[cfg(test)]
mod tests {
    use crate::geokdbush::{around, distance};
    use crate::kdbush::{KDBush,City, Point};
    use std::collections::BTreeMap;

    use std::error::Error;
    use std::fs::File;
    use std::path::Path;

    fn index_kdbush(cities: Vec<City>) -> BTreeMap<String, (f64, f64)> {
        let mut cities_map = BTreeMap::new();
        cities
            .iter()
            .for_each(|city| { cities_map.insert(city.name.to_owned(), (city.lon, city.lat)); });
        cities_map
    }

    fn serialize_cities<P: AsRef<Path>>(path: P) -> Result<KDBush<City>, Box<Error>> {
        println!("Opening cities json file...");
        let f = File::open(path)?;
        let cities: Vec<City> = serde_json::from_reader(f)?;
        println!("{:?}", cities[0]);
        let cities: KDBush<City> = KDBush::new(cities, Box::new(|c: &City| Point(c.lon, c.lat)), 64).unwrap();
        // println!("{:?}", cities.get("Amsterdam"));
        Ok(cities)
    }

    #[test]
    fn text_search_max_results() {
        let path = Path::new("./all-the-cities/test-cities.json");
        let index = serialize_cities(path).unwrap();
        println!("start...");
        let points = around(&index, -119.7051, 34.4363, Some(5), None, &None);
        println!("{:?}", points);
        assert_eq!(true, true);
    }
}
