pub fn hello_world() {
    println!("Hello, world!");
}
pub fn public_fn(){
    println!("public function calling private function");
    private_fn();
} 
fn private_fn(){
    println!("this is the private function");
}

