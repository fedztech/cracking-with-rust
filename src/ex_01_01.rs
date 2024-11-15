

// 1.1 Given 2 Strings, write a method to determine if a string has all unique characters.
// What if you cannot use additional datastructures?

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex_01_01_all_unique() {
        // Given
        let unique_string : &str = "asdfghjkl";
        // When
        let was_unique = ex_01_01::are_all_characters_unique(unique_string);
        // Then
        assert_eq!(was_unique, true);
    }

    #[test]
    fn ex_01_01_not_all_unique() {
        // Given
        let unique_string : &str = "asdfghjkla";
        // When
        let was_unique = ex_01_01::are_all_characters_unique(unique_string);
        // Then
        assert_eq!(was_unique, false);
    }
}

mod ex_01_01 {

pub fn are_all_characters_unique(text : &str) -> bool
{
    // Start from beging till end, and go through each letter and go through all the other ones to see if one is the same
    let total_chars = text.len();
    let text_bytes = text.as_bytes();
    for (ix, value) in text_bytes.iter().enumerate() {
        if ix+1 < total_chars {
            for char_to_compare in ix+1..text.as_bytes().len() {
                if *value == text_bytes[char_to_compare] {
                    return false;
                }
            }            
        }
    }
    return true;
}  

}
