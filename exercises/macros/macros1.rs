// macros1.rs
// Make me compile! Execute `rustlings hint macros1` for hints :)

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
macro_rules! my_macro_two {
    () => {
        println!("Check out my macro two!");
    };
}

fn main() {
    my_macro_two!();
}
