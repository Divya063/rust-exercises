

pub fn find_median(array_one: &[f64], array_two: &[f64]) -> f64 {

        let mut merged_array: Vec<f64> = Vec::new();
        merged_array.extend_from_slice(array_one);
        merged_array.extend_from_slice(array_two);

        merged_array.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!("Merged array is {:?}", merged_array);

        let mid = merged_array.len()/2;

        if merged_array.len() == 0 {
            return 0.0;
        }
         
        match merged_array.len() %2 == 0 {
            true => (merged_array[mid] + merged_array[mid-1])/2.0,
            false => merged_array[mid]
        }
    
}


#[cfg(test)]

mod tests {

    use crate::median::find_median;
    #[test]
    fn test_jumbled_arrays() {
        let array_one = [1.0, 3.0, 6.0, 4.0];
        let array_two = [5.0, 6.0, 7.0, 8.0];
        let result = find_median(&array_one, &array_two);
        assert_eq!(result, 5.5);
    }

    #[test]
    fn test_sorted_arrays() {
        let array_one = [1.0, 2.0, 3.0, 4.0];
        let array_two = [5.0, 6.0, 7.0, 8.0];
        let result = find_median(&array_one, &array_two);
        assert_eq!(result, 4.5);
    }

    #[test]
    fn test_odd_length_arrays() {
        let array_one = [1.0, 2.0, 3.0, 4.0, 5.0];
        let array_two = [6.0, 7.0, 8.0, 9.0];

        let result = find_median(&array_one, &array_two);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_empty_arrays() {
        let array_one: [f64; 0] = [];
        let array_two: [f64; 0] = [];

        let result = find_median(&array_one, &array_two);
        assert_eq!(result, 0.0);
    }

}

