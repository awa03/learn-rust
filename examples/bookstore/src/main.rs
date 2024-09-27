use std::io::*;
use bookstore::BookStore;
mod menu; 
mod bookstore;

fn main() {
    menu::show_menu();  
    let mut buff = String::new();

    let mut store: bookstore::BookStore = BookStore {
        books: Vec::new(),
        register: 0f64
    };

    'mainloop: loop {
        stdin().read_line(&mut buff).unwrap();
        match buff.to_uppercase().as_str() {
            "A" => bookstore::AddBook(&mut store, bookstore::MakeNewBook()),
            "B" => bookstore::DonateBook(&mut store, menu::get_book_name()),
            "C" => bookstore::SellBook(&mut store, menu::get_book_name()),
            _ => continue 'mainloop,
        }
    }    
}
