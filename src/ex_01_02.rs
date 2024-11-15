
use std::collections::HashMap;

// 1.2 Given two strings, wriea a method to decide if one is a permutation of the other.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex_01_02_is_permuatation(){
        // Given
        let str1 : &str = "hellomynameisjuan";
        let str2 : &str = "juanismynamehello";
        // When
        let is_it = ex_01_02::is_one_string_a_permutation(str1, str2);
        //Then
        assert!(is_it == true);
    }

    #[test]
    fn ex_01_02_is_not_permuatation_different_frequency(){
        // Given
        let str1 : &str = "hallomynameisjuan";
        let str2 : &str = "juanismynamehello";
        // When
        let is_it = ex_01_02::is_one_string_a_permutation(str1, str2);
        //Then
        assert!(is_it == false);
    }

    #[test]
    fn ex_01_02_is_not_permuatation_different_lengths(){
        // Given
        let str1 : &str = "hellomynameisjuan";
        let str2 : &str = "juanismynamehelloo";
        // When
        let is_it = ex_01_02::is_one_string_a_permutation(str1, str2);
        //Then
        assert!(is_it == false);
    }

}

mod ex_01_02{

// Tells whether a string is a permutation of the other one.
// A permutation occurs when:
// 1. both strings have the same length
// 2. the frequency of each character is the same.
// 3. the order of the characters does not matter.
pub fn is_one_string_a_permutation(text1 : &str, text2 : &str) -> bool{

    // We can use 2 maps, which maps a character to a frequency
    // Then we can iterate / zip the maps, to ensure that the key is the same, and the count.
    // The length of the maps has to be the same.
    if text1.len() != text2.len() {
        return false;
    }

    let mut map1 : std::collections::HashMap<u8, u32> = std::collections::HashMap::new();

    text1.as_bytes().iter().for_each(|character_byte| {
        if map1.contains_key(character_byte){
            match map1.get_mut(character_byte){
                Some(value) => *value+=1,
                None => println!("Error for {}", character_byte)
            }
        }
        else {
            map1.insert(*character_byte, 1);
        }
    });

    let mut map2 : std::collections::HashMap<u8, u32> = std::collections::HashMap::new();

    text2.as_bytes().iter().for_each(|character_byte| {
        if map2.contains_key(character_byte){
            match map2.get_mut(character_byte){
                Some(value) => *value+=1,
                None => println!("Error for {}", character_byte)
            }
        }
        else {
            map2.insert(*character_byte, 1);
        }
    });

    if map1.len() != map2.len(){
        return false;
    }

    let mut is_permutation : bool = true;

    for map1_val in map1.iter(){
        if map2.contains_key(map1_val.0) {
            match map2.get(map1_val.0){
                Some(value) => if map1_val.1 != value {is_permutation = false;}
                None => is_permutation = false
            }
        }
    }

    return is_permutation;
}


}

