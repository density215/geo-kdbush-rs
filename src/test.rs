
use kdbush::kdbush::{KDBush};
use kdbush::geokdbush::{around, distance};

#[cfg(test)]
mod tests {

    #[test]
    fn text_search_max_results() {
        let points = around(None, -119.7051, 34.4363, Some(1), None);
        assert_eq!(true, true);
    }
    
    #[test]
    fn test_fail() {
        panic!("Fail man");
    }
}