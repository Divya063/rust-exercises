mod person;
mod book;
mod library;

use person::Person;
use book::Book;
use library::Library;

fn main() {
    let person = Person::new("random".to_string(), 20);
    person.display();

    // verify ownership
    // println!("{}", person.name);

    let mut library: Library = Library::new();
    let book: Book = Book::new("Rust in action", "Tim McNamara", true, "".to_string());
    let book2: Book = Book::new("Operating Systems", "Anonymous", true, "".to_string());
    
    // ownership is moved to vector, so we cannot access book.title
    // but we can access library.books[0].title
    library.books.push(book);
    library.books.push(book2);


    library.checkout("Rust in action", &"ipsum".to_string());

    library.list_checkout_books();

    library.return_book("Rust in action");

    library.list_available_books();

}
