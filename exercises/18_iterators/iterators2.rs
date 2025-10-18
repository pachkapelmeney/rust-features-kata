// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next()
        {
            None => {String::new()},
            Some(first) => first.to_uppercase().chain(chars).collect()
        }
    // chars.into_iter().next().map(|x|x.to_ascii_uppercase()).iter().collect() //.flat_map(|x| x.to_uppercase()).collect()
    // chars.into_iter.flat_map(|x| x.to_uppercase())
    
    // match chars.next() {
    //     None => String::new(),
    //     Some(first) => todo!(),
    // }
}

// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // capitalize_first(words.iter().flatten().into());
    words.iter().copied().map(
        capitalize_first)
            .collect()
}

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().fold("".to_string(), |acc:String, x| {acc + &capitalize_first(x)})
    // ???
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
