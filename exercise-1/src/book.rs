
#[derive(Debug)]
pub struct Book<'a> {
    pub title: &'a str, // not a dynamic value
    pub author: &'a str,
    pub is_available: bool,
    pub borrower: String,

}

impl Book<'_> {
    pub fn new<'a>(title: &'a str, author: &'a str, is_available: bool, borrower: String) -> Book<'a> {
        Book {
            title,
            author,
            is_available,
            borrower,
        }
    }
}

