fn main() {
    let range = 1..11;
    let range_long = std::ops::Range {start: 1, end: 11};
    let mut string = String::new();
    let mut string2 = String::new();

    for i in range {
        string += &(i).to_string();
    }

    for i in range_long {
        string2 += &(i).to_string();
    }

    assert_eq!(string, string2);
}
