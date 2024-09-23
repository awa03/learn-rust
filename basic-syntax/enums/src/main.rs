fn main() {
    enum Animal {
        Dog(String, f64),
        Cat { name: String, weight: f64 },
        Bird, 
        Chinchilla,
        Rabbit
    }

    let mut a:Animal = Animal::Cat { name: ("Algea".to_string()), weight: (32.2) };
    a = Animal::Cat {name: "Lady Grey".to_string(), weight: 10.12};

    enum Feildless {
        Tuple(),
        Struct(),
        Unit,
    }

    enum Enum {
        Foo = 3,
        Bar = 2,
        Joe = 1,
        Greg = 0
    }

    // Discriminant
    #[repr(u8)]
    enum Enum2 {
        Unit = 3,
        Tuple(u16),
        Struct {
            a: u8,
            b: u16,
        } = 1,
    }


    // Implicit Discriminant
    enum Foo {
        Bar,            // 0
        Baz = 123,      // 123
        Quux,           // 124
    }

    let baz_discriminant = Foo::Baz as u32;
    assert_eq!(baz_discriminant, 123);



}
