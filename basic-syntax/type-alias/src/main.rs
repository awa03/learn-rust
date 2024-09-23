fn main() {
    type Point = (i32, i32);
    let p: Point = (42, 41);

    assert_eq!(p, (42, 41));

    struct MyStruct(i32);
    use MyStruct as UseAlias;

    let _ = UseAlias(5);
}
