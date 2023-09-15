use std::ops::Add;


fn increment_by_n<T>(collection: &Vec<T>, n: T) -> Vec<T> 
 where T : Add<Output = T> + Copy, // Add trait's associated type Output is same as T
{
    // map returns a new vector and 
    // for_each doesn't return anything
    return collection.iter().map(|&x| x + n).collect();
}

fn filter_by_condition<T, F>(collection: &Vec<T>, condition: F) -> Vec<T> 
 where
  F: Fn(T) -> bool,
  T: Copy, 
{
    let mut result: Vec<T> = Vec::new();

    // into_iter takes ownership of the vector
    collection.iter().filter(|&x| condition(*x)).for_each({
        |&x| result.push(x)
    });

    result
    

}

fn transform_to_string<T, F>(collection: &Vec<T>, transform: F) -> Vec<String> 
 where 
    T: Copy, 
    F: Fn(T) -> String,
 {
    return collection.iter().map(|&x| transform(x)).collect();
}

pub fn increment_by_ten_discard_odd(collections: &Vec<i32>) -> Vec<String>{
   let incremented_result = increment_by_n(collections, 10);
   let filtered_result = filter_by_condition(&incremented_result, |x| x % 2 == 0);
   let final_result   = transform_to_string(&filtered_result, |x| x.to_string());
   println!(" {:?}", final_result);

   final_result
}

#[cfg(test)]

mod tests {
    // crate is used to access items within the same crate, 
    //while super is used to access items from the parent module.

    use crate::generics::increment_by_ten_discard_odd;
    use crate::generics::increment_by_n;

    #[test]
    fn test_increment_by_n() {
        let v = vec![1, 2, 3, 4, 5];
        let result = increment_by_n(&v, 10);
        assert_eq!(result, vec![11, 12, 13, 14, 15]);
    }

    #[test]
    fn test_increment_by_ten_discard_odd() {
        let v = vec![1, 2, 3, 4, 5];
        let result = increment_by_ten_discard_odd(&v);
        assert_eq!(result, vec!["12", "14"]);
    }
}



