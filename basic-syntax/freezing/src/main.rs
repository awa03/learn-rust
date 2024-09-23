fn main() {
    let mut _var = 10;
    {
        // frozen in scope
        let _var = _var;
        // _var = 50;
    }
    // unfrozen since the imutable fell out of scope 
    _var = 11;
}
