#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn linked_list_create() {
        let my_list: linked_list::LinkedList<i32> = linked_list::LinkedList::new();

        assert!(my_list.root.is_none());
    }

    #[test]
    fn linked_list_add_first() {
        // Given
        let mut my_list: linked_list::LinkedList<i32> = linked_list::LinkedList::new();
        // When
        let data_to_insert = 1;
        my_list.push_back(data_to_insert);
        // Then
        assert!(my_list.root.is_some());
        assert!(my_list.root.unwrap().borrow().data == data_to_insert);
    }

    #[test]
    fn linked_list_add_100() {
        // Given
        let mut my_list: linked_list::LinkedList<i32> = linked_list::LinkedList::new();
        // When
        for i in 1..100 {
            my_list.push_back(i);
        }

        // Then
        assert!(my_list.root.is_some());
        let mut next_element = my_list.root.unwrap();
        for i in 1..100 {
            assert!(next_element.borrow().data == i);
            if next_element.borrow().data == 99 {
                assert!(next_element.borrow().next.is_none());
                break;
            }
            let temp_element = next_element.borrow().next.clone().unwrap().clone();
            next_element = temp_element.clone();
        }
    }
}

pub mod linked_list {
    use std::cell::RefCell;
    use std::rc::Rc;

    pub struct LinkedListNode<T> {
        pub data: T,
        pub next: Option<Rc<RefCell<LinkedListNode<T>>>>,
    }

    pub struct LinkedList<T> {
        pub root: Option<Rc<RefCell<LinkedListNode<T>>>>,
    }

    impl<T> LinkedListNode<T> {
        pub fn new(the_data: T) -> Self {
            Self {
                data: the_data,
                next: None,
            }
        }
    }

    /// Recursively finds the last node in a single linked list
    pub fn find_last_node<T>(
        current_node: Rc<RefCell<LinkedListNode<T>>>,
    ) -> Rc<RefCell<LinkedListNode<T>>> {
        if current_node.borrow().next.is_none() {
            return current_node.clone();
        }
        let next_node = current_node.borrow().next.clone().unwrap();
        return find_last_node(next_node);
    }

    impl<T> LinkedList<T> {
        pub fn new() -> Self {
            Self { root: None }
        }

        pub fn push_back(&mut self, data: T) {
            let new_node = LinkedListNode::new(data);
            if self.root.is_none() {
                self.root = Some(Rc::new(RefCell::new(new_node)));
            } else {
                let mut last_node: Rc<RefCell<LinkedListNode<T>>> =
                    find_last_node(self.root.as_mut().unwrap().clone());

                let new_ptr = Rc::new(RefCell::new(new_node));
                last_node.borrow_mut().next = Some(new_ptr.clone());
            }
        }
    }
}
