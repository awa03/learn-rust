fn main() {
    // A module is a container for zero or more items
    // Modules and types share a namespace, we cant declare a named type with the module name
    // if we use the unsafe keyword in the module we must specify it like... unsafe mod math {...}
    mod math {
    type Complex = (f64, f64);
        fn sin(f: f64) -> f64 {
            return 4.00;
        }
        fn cos(f: f64) -> f64 {
            /* ... */
            return 3.00;
        }
        fn tan(f: f64) -> f64 {
            /* ... */
            return 2.00;
        }
}

}
