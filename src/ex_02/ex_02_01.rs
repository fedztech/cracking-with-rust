// Remove Dups: Write code to remove duplicates from an unsorted linked list.
// With and without a temporary buffer.

use crate::data_structures;

#[cfg(test)]
pub mod tests {

    use super::*;

    use data_structures::linked_list::{self, linked_list::LinkedList};
    use rand::Rng;

    pub fn fill_linked_list(
        list: &mut linked_list::linked_list::LinkedList<i32>,
        number_of_elements: i32,
    ) {
        let mut generator = rand::thread_rng();

        for _ in 0..number_of_elements {
            let random_data = generator.gen_range(0..200);
            list.push_back(random_data as i32);
        }
    }

    pub fn are_there_duplicates(list: &linked_list::linked_list::LinkedList<i32>) -> bool {
        // Here we make our life easier and just use a map, if the element exists, then there is a duplicate.

        use std::collections::HashSet;

        if list.root.is_none() {
            // No duplicates in empty list
            return false;
        }

        let mut hash_set: HashSet<i32> = HashSet::new();
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
        let mut my_list: LinkedList<i32> = linked_list::linked_list::LinkedList::new();
        fill_linked_list(&mut my_list, 100);
        // When
        ex_02::remove_duplicates_no_buffer(&mut my_list);
        // Then
        assert!(are_there_duplicates(&my_list) == false);
    }
}

pub mod ex_02 {
    use crate::data_structures::linked_list::linked_list::LinkedList;
    use crate::data_structures::linked_list::linked_list::LinkedListNode;
    use std::borrow::Borrow;
    use std::borrow::BorrowMut;
    use std::cell::RefCell;
    use std::fmt::Display;
    use std::rc::Rc;

    pub fn remove_duplicates<T>(
        //current_node: &mut Rc<RefCell<LinkedListNode<T>>>,
        current_node: Rc<RefCell<LinkedListNode<T>>>,
        data_value_to_remove: T,
    ) where
        T: std::cmp::PartialEq<T> + Copy + Display
    {
        println!("Removing duplicates of {data_value_to_remove}");

        while let Some(next_node) = &mut current_node.borrow().next.clone() {
            let data_value = next_node.borrow().data;
            if data_value == data_value_to_remove {
                // Remove this node.
                // current_node.next now shall point to next_node.next
                current_node.borrow_mut().next = Some(next_node.clone());
            }
        }
    }

    pub fn remove_duplicates_no_buffer(list: &mut LinkedList<i32>) {
        // Go through the linked list, and check every data element against the rest of the list.

        let mut current_node = list.root.clone();
        while let Some(node) = &mut current_node {
            let data_value = node.as_ref().borrow().data;

            remove_duplicates(node, data_value);
            current_node = current_node.clone().unwrap().borrow().next.clone();
        }
    }
}
