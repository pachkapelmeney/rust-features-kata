fn bigger(a: i32, b: i32) -> i32 {
    // TODO: Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables
    // if a > b
    //     {a}
    // else if b > a
    //     {b}
    // else
    //     {return a;}

    //еще корогче
    if a > b { a } else { b }

    // match a {
    //     a if a > b => {a}
    //     b if b > a => {b}  // b - здесь это новая переменная, сопостовлаяемого выражания ( тоесть а). -> поэтому эквивалентоно ( a > a)
    //     any if a == b => {any} //
    //     i32::MIN..=i32::MAX => todo!()
    // }
}

fn main() {
    // You can optionally experiment here.
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
