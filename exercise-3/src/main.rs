mod median;

use median::find_median;

fn main() {
    let array_one = [1.0, 3.0, 6.0, 4.0];
    let array_two = [5.0, 6.8, 7.0];

    let result = find_median(&array_one, &array_two);
    println!("Median is {}", result)
}
