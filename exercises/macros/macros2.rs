// macros2.rs
// Make me compile! Execute `rustlings hint macros2` for hints :)

// the solution is to move the macro definition above the use
// #[macro_export] unnecessary here
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}

