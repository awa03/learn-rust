use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Circle {
    r: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle Radias {}", self.r)
    }
}

impl FromStr for Circle {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse() {
            Ok(num) => Ok(Circle {r: num}),
            Err(e) => Err(e),
        } 
    }
}

fn main() {
    let circle = Circle {r: 10}; 
    println!("{}", circle.to_string());

    // parsing string
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed: i32 = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("{}", sum);

    let radius = "    4                ";
    let circle: Circle = radius.parse().unwrap();
    println!("{}", circle);
}
