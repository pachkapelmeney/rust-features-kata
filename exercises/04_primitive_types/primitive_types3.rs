fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???
    // let a = [1..100];
    let a = (1..=100).collect::<std::vec::Vec<i32>>(); //Veec whih contains entire range
    // let a = [1; 100];//an array
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
