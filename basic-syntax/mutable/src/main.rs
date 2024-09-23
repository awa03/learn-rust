static VAR: i32 = 10;

fn main() {
    // cannot change imutable value
    let _imutable = 10;
    let mut mutable = 10;

    // can still print it like normal
    println!("imutable: {}", _imutable);
    println!("mutable: {}", mutable);
    println!("static: {}", VAR);

    mutable += 1;
    println!("mutable: {}", mutable);
}
