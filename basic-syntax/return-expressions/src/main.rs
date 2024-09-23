fn max(a: i32, b: i32) -> i32{
    if a > b  {
        return a;
    }
    return b;
}
fn main() {
    let res = max(10, 20);
    assert_eq!(res, 20);

    // denoted with the return keyword
}
