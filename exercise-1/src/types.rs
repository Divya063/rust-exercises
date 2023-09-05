
pub struct Person {
    pub name: String,
    pub age: u32,
}


pub struct Library {
    books: Vec<Book>,
}

pub struct Book {
    title: String,
    author: String,
    is_avaialble: bool,
}