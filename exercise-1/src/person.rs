pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Person {
    pub fn new(name: String, age:u32) -> Person{
        Person {
            name,
            age,
        }
    }
    
    // reference to avoid ownership
    pub fn display(&self) {
        println!("{} is {} years old", self.name, self.age)
    }
}