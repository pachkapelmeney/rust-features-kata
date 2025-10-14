macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

macro_rules! my_another_macro {
    () => {
        println!("test");
    };
}

fn main() {
    // TODO: Fix the macro call.
    my_macro!();
    my_another_macro!();
}
