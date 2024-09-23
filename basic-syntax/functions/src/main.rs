// we can make const functions by using the const keyword

const fn zero() -> i32 {0}

// Async functions -- 
async fn example(x: &str) -> usize {
    x.len()
}
async fn regular_example() -> i32 {0}
async unsafe fn unsafe_example() -> i32 {0}

// Functions
fn answer_to_universe() -> i32 {43}

fn foo<A, B>(x: A, y: B) -> String{ String::new()}

fn first((value, _) : (i32, i32)) -> i32 { value }

fn main() {
    assert_eq!(answer_to_universe(), 43);
    assert_eq!(foo(32, 12), String::new());

    extern "C" fn new_i32() -> i32 { 0 }
    extern "stdcall" fn new_i32_stdcall() -> i32 { 0 }

    let c = new_i32();
    let j = new_i32_stdcall();

    assert_eq!(c, 0);
    assert_eq!(j, 0);
    assert_eq!(zero(), 0);

}
