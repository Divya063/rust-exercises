// Title: Book
pub struct Book {
    pub title: String,
    pub author: String,
    pub is_available: bool,
    pub borrower: String,

}

impl Book {
    pub fn new(title: String, author: String, is_available: bool, borrower: String) -> Book {
        Book {
            title,
            author,
            is_available,
            borrower,
        }
    }
}

