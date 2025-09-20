// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }


pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

pub fn is_palindrome(input: &str) -> bool {
    let cleaned_input: String = input.chars()
                                     .filter(|c| c.is_alphanumeric())
                                     .collect::<String>()
                                     .to_lowercase();
    cleaned_input == cleaned_input.chars().rev().collect::<String>()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("hello"), "olleh");
        assert_eq!(reverse("Rust"), "tsuR");
        assert_eq!(reverse("12345"), "54321");
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("Madam, in Eden, I'm Adam"));
        assert!(!is_palindrome("hello"));
    }
}


// cargo new --lib text_magic
// crates.io 
