// Remove Dups: Write code to remove duplicates from an unsorted linked list.
// With and without a temporary buffer.

use crate::data_structures;

#[cfg(test)]
pub mod tests {
    use std::borrow::Borrow;

    use super::*;

    use data_structures::linked_list::{self, linked_list::LinkedList};
    use rand::Rng;

    
    pub fn fill_linked_list( list: &mut linked_list::linked_list::LinkedList<i32>, number_of_elements : i32) {
        let mut generator = rand::thread_rng();

        for _  in 0..number_of_elements{
            let random_data = generator.gen_range(0..1000);
            list.push_back(random_data as i32);
        }

    }

    pub fn are_there_duplicates(list : & linked_list::linked_list::LinkedList<i32>) -> bool{
        // Here we make our life easier and just use a map, if the element exists, then there is a duplicate.

        use std::collections::HashSet;

        if list.root.is_none(){
            // No duplicates in empty list
            return false;
        }
    
        let mut hash_set : HashSet<i32> = HashSet::new();
        let mut current_node = list.root.clone();
        while let Some(node) = &current_node {
            let data_value = node.as_ref().borrow().data;
            if hash_set.contains(&data_value) {
                return true;
            }
            hash_set.insert(data_value);
            let temp_node = node.borrow_mut().next.clone();
            current_node = temp_node;
        }
        
        return false;
    }


    #[test]
    fn ex_02_01_remove_duplicates() {
        // Given
        let mut my_list : LinkedList<i32> = linked_list::linked_list::LinkedList::new();
        fill_linked_list(&mut my_list, 100);
        // When
        ex_02::remove_duplicates_no_buffer(&mut my_list);
        // Then
        assert!(are_there_duplicates(&my_list) == false);
    }
}

pub mod ex_02 {
    use crate::data_structures::linked_list;



    pub fn remove_duplicates_no_buffer(list: &mut linked_list::linked_list::LinkedList<i32>) {
        println!("ex_02_01");
    }
}
