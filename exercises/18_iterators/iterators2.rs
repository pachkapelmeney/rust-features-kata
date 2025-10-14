// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

use std::ops::Deref;

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => todo!(),
    }
}

// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // let iter = words.iter();
    // iter.all()
    let mut new_vector = Vec::new();
    for word in words.iter() {
        
        if let Some(first_char) = word.chars().next(){
            let uppercased_first = first_char.to_uppercase().to_string();
            let rest_of_word = &word[1..];
        }
        //let uppercase = first_letter.unwrap().to_ascii_uppercase();



        new_vector.push(value);
        // new_vector.push(uppercase.to_string());
        // new_vector.push(&word[1..]);
        // for letter in word.chars(){
        //     letter
        //     word.chars().next()
        //     new_vector.push(&word[2..]);
        //     break;
        // }
        //----
        // let mut new_Word = Vec::new();
        // let mut counter = 0;
        // for letter in word.chars(){
        //     if (counter == 1)
        //         {new_Word.push(letter.to_ascii_uppercase());}
        //     else {
        //         new_Word.push(letter);
        //     }

        //     counter += 1;
        //     if (letter == ' ')
        //         {counter = 0;}
        // }
        // // for letter in words.iter(){ // тут надо цикл ++?
        // //     new_Word.push(letter);
        // // }
        // // new_vector.push(word.to_uppercase());
        // //println!("Got: {word}");
        // //-----
    }

    new_vector
}

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
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
