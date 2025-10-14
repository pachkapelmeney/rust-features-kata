// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.

use std::process::Output;
use std::ops::Mul;

// Obtain the number of bytes (not characters) in the given argument
// (`.len()` returns the number of bytes in a string).
// TODO: Add the `AsRef` trait appropriately as a trait bound.
fn byte_counter<T: AsRef<str>>(arg: T) -> usize{
    arg.as_ref().len()
}

// Obtain the number of characters (not bytes) in the given argument.
// TODO: Add the `AsRef` trait appropriately as a trait bound.
fn char_counter<T: AsRef<str>>(arg: T) -> usize 
{
    arg.as_ref().chars().count()
}

// Squares a number using `as_mut()`.
// TODO: Add the appropriate trait bound.
// fn num_sq(arg: &mut Box<u32>)
fn num_sq<T: AsMut<u32>>(arg: &mut T)
// where T: Mul<Output = T> + Clone + Copy 
{
    let mut value = arg.as_mut();
    println!("{} {}", value, {"VALUE: :?"});
    arg.as_mut() = &mut (*value * *value)   // как это сделать блин, как сделать assign?
    // let value = arg.as_mut();
    // value = &mut 3u32;
    // let mut value = arg.as_mut();
    // value = &mut (*value * *value)
    // let mut v = *arg.as();
    // let mut v = *arg.as_mut();
    //` where &mut T: Mul<&mut T, Output = ()>`
    // v = v * v
    // TODO: Implement the function body.
}
/* 
    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }
*/

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
