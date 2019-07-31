// test4.rs
// This test covers the sections:
// - Modules
// - Macros

// Write a macro that passes the test! No hints this time, you can do it!

mod llc {
    #[macro_export]
    macro_rules! my_macro {
        ($arg: expr) => {
            {
                let mut s = String::new();
                s.push_str("Hello ");
                s.push_str($arg);
                s
            }
        };
    }
}


fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }
}
