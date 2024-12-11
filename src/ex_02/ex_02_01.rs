// Remove Dups: Write code to remove duplicates from an unsorted linked list.
// With and without a temporary buffer.

#[cfg(test)]
pub mod tests {

    use super::*;

    use crate::data_structures::linked_list::{self, linked_list::LinkedList};
    use rand::Rng;

    pub fn fill_linked_list(
        list: &mut linked_list::linked_list::LinkedList<i32>,
        number_of_elements: i32,
    ) {
        let mut generator = rand::thread_rng();

        for _ in 0..number_of_elements {
            let random_data = generator.gen_range(0..100);
            list.push_back(random_data as i32);
        }
    }

    pub fn are_there_duplicates(list: &linked_list::linked_list::LinkedList<i32>) -> bool {
        // Here we make our life easier and just use a map, if the element exists, then there is a duplicate.
        println!("are_there_duplicates called ");
        use std::collections::HashSet;

        if list.root.is_none() {
            // No duplicates in empty list
            println!("No duplicates in empty list");
            return false;
        }

        let mut hash_set: HashSet<i32> = HashSet::new();
        let mut current_node = list.root.clone();
        while let Some(node) = current_node.clone() {
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

#[cfg(test)]
pub mod ex_02 {
    use crate::data_structures::linked_list::linked_list::LinkedList;
    use crate::data_structures::linked_list::linked_list::LinkedListNode;
    use std::cell::RefCell;
    use std::fmt::Display;
    use std::rc::Rc;

    pub fn remove_duplicates<T>(node: Rc<RefCell<LinkedListNode<T>>>, data_value_to_remove: T)
    where
        T: std::cmp::PartialEq<T> + Copy + Display,
    {
        // Received unmuttable Rc, so we clone it as mutable to be able to assign it.
        let mut current_node = node.clone();

        // We loop all the way to the end of the linked list by jumping from node to node.
        while (*current_node).borrow().next.is_some() {
            let data_value = (*current_node) // Dereference
                .borrow() // Borrow unmutable as do not want to change.
                .next // Get the next optional
                .as_ref() // as ref as I do not want the unwrap to move the value
                .unwrap() // Unwrap the optional, safe because the while loop guarantees that
                .borrow() // Borrow contents of RefCell as read only
                .data; // Finally copy the data.

            if data_value == data_value_to_remove {
                // Found a duplicate so the duplicate node.
                // current_node.next now shall point to next_node.next

                // Here a pointer to the node to delete is stored.
                // Once this pointer is out of scope, the object shall be deleted,
                // as there should be no other Rc objects pointing to it.
                // I use this pointer to get to the next one it points.
                // It is safe to unwrap this Optional, because I checked it already in the while statement above.
                let pointer_to_node_to_delete =
                    (*current_node).borrow().next.as_ref().unwrap().clone();

                // Does this pointer point to a valid node?
                if (*pointer_to_node_to_delete).borrow().next.is_some() {
                    // What is happening here:
                    // 1. Dereference the pointer to get the RefCell
                    // 2. As I am not interested in modifying this RefCell, I borrow it unmutable to get the value of next
                    // 3. Next gives me an Option, for which I get a reference (otherwise it would be moved).
                    // 4. I checked that next is some in the if statement above, so it is safe to get a reference and unwrap it.
                    // 5. Having a Rc, I clone it.
                    let pointer_to_next_next = (*pointer_to_node_to_delete)
                        .borrow()
                        .next
                        .as_ref()
                        .unwrap()
                        .clone();

                    // To understand this (personal notes)
                    // (*current_node).borrow_mut().next
                    // 1. Dereference the Rc which then gets us the RefCell.
                    // 2. Borrow the contents of the RefCell as mutable, as we want to change the LinkedListNode
                    // 3. Assign the next, which is a Option(Rc(RefCell(LinkedListNode))).
                    // On the right side, we need to get a Some(Rc), so we just clone the poiner.
                    (*current_node).borrow_mut().next = Some(pointer_to_next_next.clone());
                } else {
                    // If what we want to delete points to nothing, then its previous node now shall point to nothing.
                    (*current_node).borrow_mut().next = None;
                }
            } else {
                // The node was no duplicate, so we need to jump to the next node.
                if (*current_node).borrow().next.is_some() {
                    // We need this indirection (having a copy of the pointer), because we cannot borrow the "current_node"
                    // and assign it in the same statement. So we first copy the next pointer, and then modify the current node.
                    let pointer_to_next = (*current_node).borrow().next.as_ref().unwrap().clone();
                    current_node = pointer_to_next.clone();
                }
            }
        }
    }

    /// Given a linked list of any type (whose data is comparable), remove the duplicates
    /// Without using a separate buffer (like to copy the values or something)
    pub fn remove_duplicates_no_buffer<T>(list: &mut LinkedList<T>)
    where
        T: std::cmp::PartialEq<T> + Copy + Display,
    {
        // Go through the linked list, and check every data element against the rest of the list.

        let mut current_node = list.root.clone();
        while let Some(node) = current_node.clone() {
            let data_value = node.as_ref().borrow().data;
            println!("Processing {data_value}");

            remove_duplicates(node.clone(), data_value);

            if (*node).borrow().next.is_some() {
                println!("Advancing");
                current_node = Some((*node).borrow().next.as_ref().unwrap().clone());
            } else {
                println!("End reached");
                current_node = None;
            }
        }
    }
}
