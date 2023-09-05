mod types;

use types::Person;

fn main() {
    let person = Person {
        name: "Rust".to_string(),
        age: 5,
    };

    println!("{}", person.name)

}
