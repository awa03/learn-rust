// All values are by default allocated to the stack
// values can be boxed (allocated to the heap) by creating a box

use std::collections::HashMap;
use std::time::Duration;
use std::sync::Arc;
use std::thread;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

// rectangle can be found from these points
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // Allocate this point on the heap, and return a pointer to it
    Box::new(Point { x: 0.0, y: 0.0 })
}

use std::mem;
fn main() {
    let point: Point = origin();

    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 }
    };

    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    // The output can also be boxed
    let boxed_point: Box<Point> = Box::new(origin());

    // Pointer to a Pointer | Indirection
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    // box size == pointer size
    println!("Boxed point occupies {} bytes on the stack",
             mem::size_of_val(&boxed_point));

    // Derefrence the boxed_point
    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point occupies {} bytes on the stack",
             mem::size_of_val(&unboxed_point));

    // Vectors - cant push to non mutable
    let mut _vector: Vec<i32> = (1..100).collect();

    let mut xs = vec![1i32, 2, 3];

    println!("Initial vector: {:?}", xs);
    
    // Add item to vector
    xs.push(10);

    println!("Second element: {}", xs[1]);

    for i in xs.iter() {
        println!("{}", i);
    }

    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }

    for x in xs.iter_mut() {
        *x *= 3;
    }

    println!("Updated vector: {:?}", xs);

    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    let mut chars: Vec<char> = pangram.chars().collect();

    // sort the list
    chars.sort();
    // remove repeated elements
    chars.dedup();


    let alice = String::from("I like dogs");
    let bob: String = alice.replace("dog", "cat");
    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
    check_division(3, 3);
    try_division(2, 3);
    try_division(2, 0);

    let mut contacts = HashMap::new();
    contacts.insert("aiden", "324-235-3521");
    contacts.insert("joe", "813-571-5514");

    match contacts.get(&"aiden") {
        Some(&number) => println!("Hello"),
        _ => println!("Number not found")
    }

    contacts.remove(&"aiden");
    for (contact, &number) in contacts.iter() {
        println!("Calling {}", contact); 
    }
    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        // Here there is no value specification as it is a pointer to a
        // reference in the memory heap.
        let apple = Arc::clone(&apple);

        thread::spawn(move || {
            // As Arc was used, threads can be spawned using the value allocated
            // in the Arc variable pointer's location.
            println!("{:?}", apple);
        });
    }

    // Make sure all Arc instances are printed from spawned threads.
    thread::sleep(Duration::from_secs(1));
}

// When shared ownership between threads is needed, Arc(Atomically Reference Counted) can be used. This struct, via the Clone implementation can create a reference pointer for the location of a value in the memory heap while increasing the reference counter. As it shares ownership between threads, when the last reference pointer to a value is out of scope, the variable is dropped.

fn check_division(a:i32, b:i32) -> Option<i32> {
    if b == 0 {
        None
    }
    else {
        Some(a / b)
    }
}

fn try_division(dividend: i32, divisor: i32) {
    // `Option` values can be pattern matched, just like other enums
    match check_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient)
        },
    }
}
