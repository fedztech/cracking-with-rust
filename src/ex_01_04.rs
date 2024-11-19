// Palindrome permutation
// Given a string, write a function to check if it s a permutation of a palindrome.
// A palindrome is a word or phrase that is the same forwards and backwards.
// A permutation is a rearrangement of letters.
// The palindrome does not need to be limited to just dictionary words.
// EXAMPLE:
// Input: Tact Coa
// Output: True

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex_01_04_is_palindrome() {
        // Given
        let input_string: &str = "Tact Coa";
        // When
        let result = ex_01_04::is_palindrome_permutation(input_string);
        // Then
        assert!(result);
    }

    #[test]
    fn ex_01_04_is_not_palindrome() {
        // Given
        let input_string: &str = "Tact Cod";
        // When
        let result = ex_01_04::is_palindrome_permutation(input_string);
        // Then
        assert!(result==false);
    }
}

mod ex_01_04 {
    use std::{collections::HashMap};

    // For a palindrome permutation, we need a histogram of the lowercase letters
    // that count their occurrence. We are going to ignore spaces.
    // Then, if there is simmetry -> counts are multiples of 2,
    // except for 1 letter which can have just 1. Then true,
    // else false.
    pub fn is_palindrome_permutation(input_string: &str) -> bool {
        // We can use a hash map in order to store the letter as key and count as value.
        let mut distribution: HashMap<u8, u32> = HashMap::new();

        // We treat as ascii / bytes, so do not support utf8
        for letter in input_string.to_ascii_lowercase().as_bytes().iter() {
            let letter_count = distribution.entry(*letter).or_insert(0);
            *letter_count += 1;
        }

        let mut single_char_elements = 0;
        for map_element in distribution.iter() {
            // Ignore spaces
            if *map_element.0 == b' ' {
                continue;
            }
            if *map_element.1 == 1 {
                single_char_elements +=1;
                continue;
            }
            if *map_element.1 %2 != 0 {
                return false;
            } 
        }
        if single_char_elements > 1 {
            return false;
        }
        true
    }
}
