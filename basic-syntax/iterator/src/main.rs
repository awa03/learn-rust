struct Fib {
    curr: u32, 
    next: u32,
}

impl Iterator for Fib {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item>{
        let current = self.curr; 
        self.curr = self.next;
        self.next = current + self.next;
        Some(current)
    }
}

fn fibonacci() -> Fib {
    Fib {curr: 0, next: 1}
}

fn main() {
    let mut sequence = 0..3;
    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next()); 

    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }
}
