use crate::book::Book;

// #[derive(Debug)] implements Debug trait such that we can do println!("{:?}", person)  
//-- useful to not implement a Display trait for printing
#[derive(Debug)]
pub struct Library<'a> {
    pub books: Vec<Book<'a>>,
}

impl Library<'_> {
    pub fn new() -> Library<'static> {
        Library {
            books: Vec::new(),
        }
    }

    pub fn checkout(&mut self, book_name: &str, borrower: &str) {
        for book in &mut self.books{
            if book.title == book_name {
                if book.is_available {
                    book.is_available = false;
                    book.borrower = borrower.to_string();
                    println!("{} by {} is checked out by {}", book.title, book.author, book.borrower)
                } else {
                    println!("{} by {} is not available", book.title, book.author)
                }
            }
        }
    }

    pub fn return_book(&mut self, book_name: &str) {
        for book in &mut self.books{
            if book.title == book_name {
                if !book.is_available {
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
       self.books.iter().filter(|book| !book.is_available)
        .for_each(|book| { println!("{:?}", book)});

        // for book  in checked_out_books {
        //     if book.is_available == false {
        //         println!("{}, {}, {}", book.title, book.author, book.borrower);
        //     }
        // }
    }

   pub fn list_available_books(&self) {
        println!("List of available books:");
       self.books.iter().filter(|book| book.is_available).for_each(|book| {println!("{:?}", book)});


        // for book in available_books {
        //     if book.is_available == true {
        //         println!("{} by {} is available", book.title, book.author)
        //     }
        // }
    }
}
