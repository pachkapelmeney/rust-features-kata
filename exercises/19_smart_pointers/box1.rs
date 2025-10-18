// At compile time, Rust needs to know how much space a type takes up. This
// becomes problematic for recursive types, where a value can have as part of
// itself another value of the same type. To get around the issue, we can use a
// `Box` - a smart pointer used to store data on the heap, which also allows us
// to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the "cons list", a
// data structure frequently found in functional programming languages. Each
// item in a cons list contains two elements: The value of the current item and
// the next item. The last item is a value called `Nil`.

use std::ops::Deref;

// TODO: Use a `Box` in the enum definition to make the code compile.
#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, Box<List>), //Box<List> - means the tail of the list is ALWAYS (even it's empty Nil) will be the box
    Nil,
}
// impl PartialEq for List {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (Self::Cons(l0, l1), 
//              Self::Cons(r0, r1))  // может ут ткак-то указать что можно сравнивать Box<List> с List ?
//                 => l0 == r0 && l1 == r1,
//             _ => false,
//         }
//     }
// }

// TODO: Create an empty cons list.
fn create_empty_list() -> List {
    List::Nil
}

// TODO: Create a non-empty cons list.
fn create_non_empty_list() -> List {
    // List::Cons(3, List::Nil)
    List::Cons(3, Box::new(List::Nil))
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}
