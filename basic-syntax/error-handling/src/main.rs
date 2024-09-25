fn give_adult(drink: Option<&str>){
    match drink {
        Some("lemonade") => println!("Too much sugar"),
        Some(inner)      => println!("Thats pretty good"),
        None             => println!("Theres nothing in this cup...")
    }

}
fn drink(beverage: &str) {
   if beverage == "lemonade" {
        if cfg!( panic = "abort") {
            println!("This is not your party");
        }
        else {
            println!("Spit it out");
        }
    }
   else {
        println!("Some {} is all I need", beverage);
    }
}

struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    fn work_phone_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}

fn main() {
    // rustc  main.rs -C panic=abort
    // panic("Error")
    drink("juice");
    drink("lemonade");

    let water = Some("water");
    let void = None;
    give_adult(water);
    give_adult(void);

    let p = Person {
        job: Some(
            Job {
                phone_number: Some(
                    PhoneNumber {
                        area_code: Some(1),
                        number: 2213213321,
                    }
                )
            }
        )
    };

    assert_eq!(p.work_phone_area_code(), Some(1));
}
