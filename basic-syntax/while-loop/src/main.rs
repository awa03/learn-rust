fn main() {
    println!("While Loops!");

    let mut i = 0;
    loop {
        println!("Infinite Loop Operator");
        if i == 10 {
            break;
        }
        i = i + 1;
    }

    assert_eq!(i, 10);

    let mut list = vec![5, 10, 15, 20];
    while let Some(y) = list.pop() {
      print!("y = {}", y);  
    }


    let mut vals = vec![2, 3, 1, 2, 2];
    while let Some(v @ 1) | Some(v @ 2) = vals.pop() {
        // Prints 2, 2, then 1
        println!("{}", v);
}
}
