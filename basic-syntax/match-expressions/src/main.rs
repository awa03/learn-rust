fn main() {
// A match behaves differently depending on whether or not the scrutinee expression is a place expression or value expression. If the scrutinee expression is a value
// expression, it is first evaluated into a temporary location, and the resulting value is sequentially compared to the patterns in the arms until a match is found.
// The first arm with a matching pattern is chosen as the branch target of the match, any variables bound by the pattern are assigned to local variables in the arm’s block, and control enters the block.
// When the scrutinee expression is a place expression, the match does not allocate a temporary location; however, a by-value binding may copy or move from the memory location.
// When possible, it is preferable to match on place expressions, as the lifetime of these matches inherits the lifetime of the place expression rather than being restricted to the inside of the match.

    let x = 1;
    let mut _m;
    match x {
        1 =>_m = true,
        _ =>_m = false
    }

    assert!(_m);

    let y = 10;
    let message = match y {
        0 | 1 => "not many",
        2..=9 => "maybe enough",
        10..=100 => "loads",
        _ => "I dont know bro"
    };

    assert_eq!(message, "loads");
    struct S(i32, i32);

    match S(1, 2) {
        S(z @ 1, _) | S(_, z @ 2) => assert_eq!(z, 1),
        _ => panic!(),
    }


}
