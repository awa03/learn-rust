fn main() {
    // Basic int declaration
    let x = 5;
    
    // Explicit int decleration
    let y:i32 = 5;

    assert_eq!(y, x);

    let z:i32 =  { 
        1 + 1 + 3
    };

    assert_eq!(z, y);

    println!("x : {}", x);
    println!("y : {}", y);
    println!("z : {}", z);



}
