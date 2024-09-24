fn main() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // We can get values from this through using
    println!("4th tuple entry {}", long_tuple.4);

    // loads of tuples
    let tuple_of_tuples = (
        (3, 2i16, -32, 234), 
        (32, 1i8, 1u8, -31, -324)
    );

    println!("Tuple being indexed to 1.3, {}", tuple_of_tuples.1.3);

    // longer than 12 elements wont be printed
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // pairs
    let pair = (1, true);
    println!("pair: {:?}", pair);

}
