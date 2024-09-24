#![warn(clippy::no_effect)]
fn main() {
    // Underscores can be used for readability
    21_000_213;

    // 1 + 2
    println!("1 + 2 = {}", 1u32 + 2);
    
    // 3 + 2
    println!("3 - 2 (i32){}", 3i32 - 2);
    println!("3 - 2 (u32){}", 3i32 - 2);

    // Scientific Notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // boolean algebra
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true {}", !true);

    // Bitwise
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);

    // Another underscore example
    println!("One million is written as {}", 1_000_000u32);

}
