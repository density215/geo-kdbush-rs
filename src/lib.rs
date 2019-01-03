pub mod geokdbush;
pub mod kdbush;

extern crate flate2;

#[cfg(test)]
mod tests {
    use crate::geokdbush::around;
    use crate::kdbush::{City, KDBush, Point};

    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    use flate2::read::GzDecoder;

    fn serialize_cities<P: AsRef<Path>>(path: P) -> Result<KDBush<City>, Box<Error>> {
        println!("Opening cities json file...");
        let mut s: String = "".to_string();
        let f = File::open(path)?;
        GzDecoder::new(f).read_to_string(&mut s).unwrap();
        let cities: Vec<City> = serde_json::from_str(s.as_str())?;
        println!("{:?}", cities[0]);
        let cities: KDBush<City> =
            KDBush::new(cities, Box::new(|c: &City| Point(c.lon, c.lat)), 64).unwrap();
        Ok(cities)
    }

    #[test]
    fn text_search_max_results() {
        let path = Path::new("./all-the-cities/cities.json.gz");
        let index = serialize_cities(path).unwrap();
        println!("start...");
        // let points = around::<City>(&index, -119.7051, 34.4363, Some(5), None, &None));
        // dead center Amsterdam 4.88969, 52.37403
        let points = around::<City>(
            &index,
            4.8285843,
            52.3546274,
            Some(15),
            None,
            &Some(Box::new(|c: &City| c.population > 15000)),
        );
        let names: Vec<String> = points.iter().map(|p| p.name.to_string()).collect();
        println!("{:?}", names);
        assert_eq!(true, true);
    }
}
