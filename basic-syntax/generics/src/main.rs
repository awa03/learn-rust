use std::fmt::{
    Display, 
    Debug,
};

struct A;          // Concrete type A
struct S(A);       // Concrete type S
struct SGen<T>(T); // Generic type 

fn not_generic(_s: S) {}

fn gen_specific(_s: SGen<A>) {}

fn generic<T>(_s: SGen<T>) {}

// Implementation
struct GenericVal<T>(T);

impl GenericVal<f32> {}
impl GenericVal<S> {}
impl<T> GenericVal<T> {}

struct Val {
    val: i32,
}

struct GenVal<T> {
    gen_val: T,
}

struct NULL;
struct Empty;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U{
    fn double_drop(self, _: T) {}
}

impl Val {
    fn value(&self) -> &i32 {
        &self.val
    }
}

impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn print<T:Display>(line_to_print: T) -> T{
    println!("{}", line_to_print);
    line_to_print
}

struct Dis<T: Display>(T);

trait HasArea {
    fn area (&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}

#[derive(Debug)]
struct Rectangle {length: f64, height: f64}

#[allow(dead_code)]
struct Triangle  { length: f64, height: f64 }

fn area<T: HasArea>(t: &T) -> f64 { t.area() }

struct Turkey;
struct BlueJay;
struct Pigeon;

trait Red {}
trait Blue {}
trait Grey {}

impl Red for Turkey {}
impl Blue for BlueJay {}
impl Grey for Pigeon {}

// Only valid for the traits that are implemented for these types
fn red<T: Red>(_: &T) -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }
fn grey<T: Grey>(_: &T) -> &'static str { "grey" }

fn main() {
    not_generic(S(A));
    gen_specific(SGen(A));
    generic::<char>(SGen('a'));
    generic(SGen('c'));

    let x = Val { val: 3 };
    let y = GenVal { gen_val: 2f32 };

    assert_eq!(x.val, 3i32);
    assert_eq!(y.value(), &2f32);

    let empty = Empty;
    let null = NULL;
    
    empty.double_drop(null);

    assert_eq!(print("Testing Generic"), "Testing Generic");
    
    let _display = Dis("Testing Display");

    let _rectangle = Rectangle { length: 3.0, height: 2.0 };    
    let _triangle = Triangle  { length: 3.0, height: 4.0 };

    let _bird = Turkey;
    assert_eq!(red(&_bird), "red");

    let blue_jay = BlueJay;

    // Trying grey blue jay
    // println!("A blue jay is {}", grey(&blue_jay));
    
}
