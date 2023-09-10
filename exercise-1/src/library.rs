use crate::book::Book;
pub struct Library {
    pub books: Vec<Book>,
}

impl Library {
    pub fn new() -> Library {
        Library {
            books: Vec::new(),
        }
    }

    pub fn checkout(&mut self, book_name: &String, borrower: &String) {
        for book in &mut self.books{
            if book.title == book_name.to_string() {
                if book.is_available == true {
                    book.is_available = false;
                    book.borrower = borrower.to_string();
                    println!("{} by {} is checked out by {}", book.title, book.author, book.borrower)
                } else {
                    println!("{} by {} is not available", book.title, book.author)
                }
            }
        }
    }

    pub fn return_book(&mut self, book_name: String) {
        for book in &mut self.books{
            if book.title == book_name {
                if book.is_available == false {
                    book.is_available = true;
                    book.borrower = "".to_string();
                    println!("{} by {} is returned", book.title, book.author)
                } else {
                    println!("{} by {} is already available", book.title, book.author)
                }
            }
        }
    }

   pub fn list_checkout_books(&self) {
        println!("List of checked out books:");
        for book  in &self.books {
            if book.is_available == false {
                println!("{}, {}, {}", book.title, book.author, book.borrower);
            }
        }
    }

   pub fn list_available_books(&self) {
        println!("List of available books:");
        for book in &self.books {
            if book.is_available == true {
                println!("{} by {} is available", book.title, book.author)
            }
        }
    }
}
