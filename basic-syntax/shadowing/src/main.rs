fn main() {
    let var = 1;
    {
        // In Scope
        let var = "this is the shadow";
        println!("var: {}", var.to_string());
    }
    // local var is now out of scope
    println!("var: {}", var.to_string());
}
