
// URLify: Write a method to replace all spaces in a string with %20.
// You may assume that the string has sufficient space at the end to hold the additional characters,
// and that you are given the true length of the string.
// EXAMPLE
// Input : "Mr John Smith    ", 13
// Output: "Mr%20John%20Smith"

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex_01_03_replace(){
        // Given
        let input = ex_01_03::Input{text:"hello my name is juan    ", num:21};
        // When
        let urlified = ex_01_03::urlify(&input);
        // Then
        assert!(urlified == "hi");
    
    }
}

mod ex_01_03{

    pub struct Input
    {
        text : &'static str,
        num : i32
    }

    pub fn urlify( input : &Input ) -> String
    {
        String::from_str("")
    }

}