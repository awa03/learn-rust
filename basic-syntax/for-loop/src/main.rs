fn main() {
    println!("For Loop In Rust");
    let mut sum = 0;

    // itr for loop - ranging 1 to 11
    for i in 1..11 {
        println!("{}", i);
        sum += i;
    }    

    println!("{}", sum.to_string() );

    let mut last = 10;
    for i in 1..100 {
        if i == 12 { 
            break;
        }
        last = i;
    }
    println!("{}", last.to_string());
    assert_eq!(last, 11);

    // iterator loop
    let v = &[5, 1, 2, 4, 1];
    for num in v {
        print!(" {} ", num);
    }
    
    let j = &["apples", "cake", "coffee"];

    for text in j {
        println!("I like {}.", text);
    }

}
