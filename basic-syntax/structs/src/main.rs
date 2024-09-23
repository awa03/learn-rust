fn main() {
    struct Point(i32, i32);
    let p = Point(10, 11);
    let px: i32 = match p { Point(x, _) => x };

    struct Cake;
    let c = [Cake, Cake {}, Cake, Cake{}];
    
    struct Cookie {}
    const Cookie: Cookie = Cookie {};
    let c = [Cookie, Cookie {}, Cookie, Cookie {}];
}
