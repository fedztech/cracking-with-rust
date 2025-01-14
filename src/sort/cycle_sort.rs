// https://www.geeksforgeeks.org/cycle-sort/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycle_sort_empty_array() {
        // Given: An empty array of integers
        let mut my_array: [i32; 0] = [];
        assert_eq!(my_array.is_empty(), true);
        // When: Cycle sort is called for it, with its comparison operator for integers implemented.
        sort::cycle_sort(&mut my_array, sort::Monotonicity::Increasing);
        // Then: Array is unmodified.
        assert_eq!(my_array.is_empty(), true);
    }

    #[test]
    fn cycle_sort_one_element_array() {
        // Given: An i32 array with 1 element with value 4
        let mut my_array: [i32; 1] = [4];
        // When: Cycle sort is called with the array an a comparison operator.
        sort::cycle_sort(&mut my_array, sort::Monotonicity::Increasing);
        // Then: The array shall not be empty and the only element shall be 4
        assert_eq!(my_array.is_empty(), false);
        assert_eq!(my_array[0], 4);
    }

    #[test]
    fn cycle_sort_two_element_array() {
        // Given: An i32 array with 2 elements 9 and 4 (in that order)
        let mut my_array: [i32; 2] = [9, 4];
        // When: Cycle sort is called with comparison operator returning true if the first element is greater than second.
        sort::cycle_sort(&mut my_array, sort::Monotonicity::Increasing);
        // Then: The array is sorted asc.
        assert_eq!(my_array.is_empty(), false);
        assert_eq!(my_array[0], 4);
        assert_eq!(my_array[1], 9);
    }

    #[test]
    fn cycle_sort_ten_element_array() {
        // Given: An i32 array with 2 elemen, ts 9 and 4 (in that order)
        let mut my_array: [i32; 10] = [9, 4, 4, 5, 10, 45, 2, 1, 8, 33];
        // When: Cycle sort is called with comparison operator returning true if the first element is greater than second.
        sort::cycle_sort(&mut my_array, sort::Monotonicity::Increasing);
        // Then: The array is sorted asc.
        assert_eq!(my_array.is_empty(), false);
        assert_eq!(my_array[0], 1);
        assert_eq!(my_array[9], 45);
        for ix in 0..9 {
            assert_eq!(my_array[ix] <= my_array[ix + 1], true);
        }
    }

    #[test]
    fn cycle_sort_ten_element_array_decreasing() {
        let mut my_array: [i32; 10] = [9, 4, 4, 5, 10, 45, 2, 1, 8, 33];

        // When: Cycle sort is called with comparison operator returning true if the first element is greater than second.
        sort::cycle_sort(&mut my_array, sort::Monotonicity::Decreasing);
        // Then: The array is sorted asc.
        assert_eq!(my_array.is_empty(), false);
        assert_eq!(my_array[0], 45);
        assert_eq!(my_array[9], 1);
        for ix in 0..9 {
            assert_eq!(my_array[ix] >= my_array[ix + 1], true);
        }
    }

    #[test]
    fn cycle_sort_ten_element_array_decreasing2() {
        let mut my_array: [i32; 10] = [9, 4, 4, 5, 10, 4, 2, 4, 8, 33];

        // When: Cycle sort is called with comparison operator returning true if the first element is greater than second.
        sort::cycle_sort(&mut my_array, sort::Monotonicity::Decreasing);
        // Then: The array is sorted asc.
        assert_eq!(my_array.is_empty(), false);
        assert_eq!(my_array[0], 33);
        assert_eq!(my_array[9], 2);
        for ix in 0..9 {
            assert_eq!(my_array[ix] >= my_array[ix + 1], true);
        }
    }
}

mod sort {
    use std::mem::swap;

    pub enum Monotonicity {
        Increasing,
        Decreasing,
    }

    pub fn cycle_sort<ElementType>(elements: &mut [ElementType], monotonicity: Monotonicity)
    where
        ElementType: Sized + Clone + std::cmp::PartialOrd + std::cmp::Eq,
    {
        // Initialiye cycle start to 0
        let num_elements = elements.len();

        if num_elements < 2 {
            return;
        }

        // We have to test the greater_equal_than function

        // For each element in the array, compare it with every other element to its right.
        // Increase the position counter in order to then insert it in its final position.
        // This is achieved by counting all of the items that are smaller.

        for current_element_position in 0..num_elements {
            // Go through the complete array.

            // Find the position of current_element_position
            // A do while kind of loop is used here as I have to go through the elements as many times
            // until I do not need to swap anymore.
            loop {
                let mut swap_position = current_element_position;
                for comparing_element_position in (current_element_position + 1)..num_elements {
                    match monotonicity {
                        Monotonicity::Increasing => {
                            if elements[current_element_position]
                                > elements[comparing_element_position]
                            {
                                swap_position = swap_position + 1;
                            }
                        }
                        Monotonicity::Decreasing => {
                            if elements[current_element_position]
                                < elements[comparing_element_position]
                            {
                                swap_position = swap_position + 1;
                            }
                        }
                    }
                }

                if swap_position == current_element_position {
                    // The element is already in its sorted position,
                    // So continue with the next element.
                    break;
                }

                // If we have the same value appear multiple times in the array, we will come to the
                // point where we would be swapping the same value and then end up in an endless loop
                // Thus we need to increase the swap position until this condition is not true.
                while elements[swap_position] == elements[current_element_position] {
                    swap_position = swap_position + 1;
                }

                // Now do the swapping.

                // ToDo: Better swapping
                let temp = elements[swap_position].clone();
                elements[swap_position] = elements[current_element_position].clone();
                elements[current_element_position] = temp;
            }
        }
    }
}
