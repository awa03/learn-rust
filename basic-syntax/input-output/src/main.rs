use std::io::Write;

fn main() {
    let mut line = String::new();

    // output
    println!("Enter Your Name");

    // input
    let b1 = std::io::stdin().read_line(&mut line).unwrap();

    println!("Hello {}", line);

    println!("no of bytes read , {}", b1);

    let b1 = std::io::stdout().write("Testing ".as_bytes()).unwrap();
    let b2 = std::io::stdout().write(String::from("This").as_bytes()).unwrap();
    std::io::stdout().write_all(format!("\nbytes written {}",(b1+b2)).as_bytes()).unwrap();

    let cmd_line = std::env::args();
    println!("No of elements in arguments is :{}",cmd_line.len()); 
    for arg in cmd_line {
      println!("[{}]",arg); //print all values passed 
   }


}
