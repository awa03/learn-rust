
pub fn show_menu(){
    println!("Welcome To The Bookstore");
}

pub fn get_book_name() -> String {
    use std::io::*;
    let mut book_name = String::new();
    stdin().read_line(&mut book_name).unwrap();
    book_name
}
