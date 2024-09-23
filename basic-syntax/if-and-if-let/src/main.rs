fn main() {
    let i = 0;
    if i == 0 {
        println!("I is 0");
    }
    else {
        println!("I is not 0");
    }

    let y = if i == 0 {
        "I is 0"
    }
    else {
        "I is not 0"
    };

    assert_eq!(y, "I is 0");

    let dish = ("potato", "rice");
    if let ("potato", d) = dish {
        println!("Served with {}", d);
    }

    if let _ = 5 {
        println!("Irrefutable patterns are always true");
    }
    let x = Some(3);
    let a = if let Some(1) = x {
        1
    } else if x == Some(2) {
        2
    } else if let Some(y) = x {
        y
    } else {
        -1
    };
    assert_eq!(a, 3);

    enum E {
        X(u8), 
        Y(u8), 
        Z(u8)
    }

    let v = E::Y(12);
    if let E::X(n) | E::Y(n) = v {
        assert_eq!(n, 12);
    }
}
