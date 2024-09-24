use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber  {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        }
        else {
            Err(())
        }
    }
}

fn main() {
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8))); 
    assert_eq!(EvenNumber::try_from(16), Ok(EvenNumber(16))); 
    assert_eq!(EvenNumber::try_from(1024), Ok(EvenNumber(1024))); 
    assert_eq!(EvenNumber::try_from(5), Err(()));

    let res: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(res, Ok(EvenNumber(8)));
    let res: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(res, Err(()));
}
