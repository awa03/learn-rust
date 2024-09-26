// only compiled if on linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// wont get compiled if on linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

// will run on $ rustc --cfg some_condition custom.rs && ./custom
#[cfg(some_condition)]
fn conditional_fn(){
    println!("Its been met");
}

fn main() {
     are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }   

    conditional_fn();
}
