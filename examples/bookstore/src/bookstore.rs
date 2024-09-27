pub struct Book {
    title: String,
    author: String,
    price: f64,
}
pub struct BookStore {
    pub books: Vec<Book>,
    pub register: f64,
}

impl Book {
}  

impl Book{

}

pub fn Display_All_Books(store: BookStore){
    for book in store.books {
        Display_Book(book);
        println!("____________________");
    }
}

pub fn Display_Book(book: Book){
    println!("Title: {}", book.title.to_string());     
    println!("Price: {}", book.price.to_string());
}

pub fn MakeNewBook() -> Book{
    use std::io;
    let mut _title = String::new();
    let mut _price = String::new();
    let mut _author = String::new();
    
    println!("Enter Title: ");
    io::stdin().read_line(&mut _title).expect("Failed To Readline");
    println!("Enter Author: ");
    io::stdin().read_line(&mut _author).expect("Failed To Readline");
    println!("Enter Price: ");
    io::stdin().read_line(&mut _price).expect("Failed To Readline");

   let _price: f64 = match _price.trim().parse::<f64>() {
        Ok(value) => value,
        Err(_) => {
            println!("Failed to parse price as f64");
            return Book { title: "".to_string(), author: "".to_string(), price: 0f64}
        }
    };
    
    Book { title: _title, price: _price, author: _author}
}

pub fn AddBook(store: &mut BookStore, book: Book){
    store.books.push(book);
}
pub fn DonateBook(store: &mut BookStore, bookname: String){
    for book_index in 0..store.books.len() {
        if store.books[book_index].title == bookname {
            store.books.remove(book_index);
        } 
    }
}
pub fn SellBook(store: &mut BookStore, bookname: String){
    for book_index in 0..store.books.len() {
        if store.books[book_index].title == bookname {
            store.register += store.books[book_index].price;
            store.books.remove(book_index);
        } 
    }
}


