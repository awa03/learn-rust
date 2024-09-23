// Suppress warnings about overflows
#![allow(overflowing_literals)]

fn main() {
    let mut _dec = 65.48_f32;
    let mut _var: u8 = 10;
    let mut _ch = _var as char; 
    let mut _int = _dec as u8;

    println!("dec {}", _dec);
    println!("var {}", _var);
    println!("ch {}", _ch);
    println!("int {}", _int);

    println!("1000 as a u16 is: {}", 1000 as u16);
    println!("1000 as a u8 is : {}", 1000 as u8);

    println!("   nan as u8 is : {}", f32::NAN as u8);
}
