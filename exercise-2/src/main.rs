mod generics;

use generics::increment_by_ten_discard_odd;


fn main() {


    let v = vec![1, 2, 3, 4, 5];
    let _result = increment_by_ten_discard_odd(&v);
}    
