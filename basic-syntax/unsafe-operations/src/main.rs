use std::slice;
use std::arch::asm;
fn main() {
    let raw_p: *const u32  = &10;

    let some_vector = vec![10, 20, 43, 201];

    // make pointer ref to vector
    let pointer = some_vector.as_ptr();
    let length = some_vector.len();


    let x: u64;
    let i: u64 = 3;
    let _o: u64;

    let cmd = 0xd1;

    //  programmer's responsibility to ensure correctness instead of the compiler's
    unsafe {
        // Derefrencing pointer can only be done through unsafe block
        assert_eq!(*raw_p, 10);

        let myslice: &[u32] = slice::from_raw_parts(pointer, length);
        assert_eq!(some_vector.as_slice(), myslice);

        // inline asm
        asm!("mov {}, 5", out(reg) x);

        asm!(
            "mov {0}, {1}",
            "add {0}, 5",
            out(reg) _o,
            in(reg) i,
        );

        asm!("out 0x64, eax", in("eax") cmd);

    }
    assert_eq!(x, 5);
    assert_eq!(0, 8);
}
