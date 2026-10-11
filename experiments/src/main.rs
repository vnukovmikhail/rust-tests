#[derive(Debug, Clone)]
pub struct Book {
    title: String,
    desc: String,
}

impl Book {
    pub fn new(title: impl Into<String>, desc: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            desc: desc.into(),
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn desc(&self) -> &str {
        &self.desc
    }
}

#[derive(Debug, Default)]
pub struct Library {
    name: String,
    books: Vec<Book>,
}

impl Library {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            books: Vec::new(),
        }
    }

    pub fn with_books(name: impl Into<String>, books: Vec<Book>) -> Self {
        Self {
            name: name.into(),
            books,
        }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn find_book(&self, title: &str) -> Option<&Book> {
        self.books.iter().find(|b| b.title == title)
    }

    pub fn book_count(&self) -> usize {
        self.books.len()
    }
}

fn main() {
    let mut lib = Library::new("Yoro Library");

    lib.add_book(Book::new("The Rust Book", "Official guide"));
    lib.add_book(Book::new("Programming Rust", "Systems programming"));

    if let Some(book) = lib.find_book("The Rust Book") {
        println!("Book found: {} - {}", book.title(), book.desc());
    }

    println!("Total books in {}: {}", lib.name, lib.book_count());
}
