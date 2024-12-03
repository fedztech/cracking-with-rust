#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex_01_03_replace_functional() {
        // Given
        let input = ex_01_03::Input {
            text: "hello my name is juan            ",
            num: 21,
        };
        // When
        let urlified = ex_01_03::urlify_functional(&input);
        
        // Then
        let resulting_str = "hello%20my%20name%20is%20juan";
        assert!(urlified.eq(resulting_str) == true);
    }

    #[test]
    fn ex_01_03_replace_in_place(){
        // Given
        let input = ex_01_03::Input {
            text: "hello my name is juan        ",
            num: 21,
        };
        // When
        let urlified = ex_01_03::urlify_inplace(&input);
        // Then
        let resulting_str = "hello%20my%20name%20is%20juan";
        assert!(resulting_str.len()==urlified.len());
        assert!(urlified.eq(resulting_str)==true);
    }
}

mod ex_01_03 {
    use std::{str::FromStr, thread::current};

    pub struct Input {
        pub text: &'static str,
        pub num: u16,
    }

    // URLify: Write a method to replace all spaces in a string with %20.
    // You may assume that the string has sufficient space at the end to hold the additional characters,
    // and that you are given the true length of the string.
    // EXAMPLE
    // Input : "Mr John Smith    ", 13
    // Output: "Mr%20John%20Smith"


    pub fn urlify_functional(input: &Input) -> String {
        let input_string = String::from_str(input.text).unwrap();
        input_string.trim_end_matches(" ").replace(" ", "%20")
    }

    pub fn urlify_inplace(input: &Input) -> String {
        let mut input_string = String::from_str(input.text).unwrap();
        
        unsafe {
            let mut str_vec =input_string.as_mut_vec();
            let mut current_char:usize=0;
            let mut current_last:usize = <u16 as TryInto<usize>>::try_into(input.num).unwrap();
            for _ in 1..input.num {
                if str_vec[current_char] == b' '{
                    // Copy all the characters from current_last to current_char+1
                    // 3 spaces to the back
                    for char_ix in (current_char+1..current_last).rev() {
                        str_vec[char_ix+2] = str_vec[char_ix];
                    }
                    str_vec[current_char] = b'%';
                    str_vec[current_char+1] = b'2';
                    str_vec[current_char+2] = b'0';
                    current_char+=3;
                    current_last+=2;
                }
                else {
                    current_char +=1;
                }
                let print_string = String::from_utf8(str_vec.to_vec()).unwrap();
            }
            String::from_utf8(str_vec.to_vec()).unwrap()
        }
    }
}
