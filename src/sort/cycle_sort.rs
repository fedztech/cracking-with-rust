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
            println!(
                "Value[{}] {} vs Value[{}] {}",
                ix,
                my_array[ix],
                ix + 1,
                my_array[ix + 1]
            );
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
            println!(
                "Value[{}] {} vs Value[{}] {}",
                ix,
                my_array[ix],
                ix + 1,
                my_array[ix + 1]
            );
            assert_eq!(my_array[ix] >= my_array[ix + 1], true);
        }
    }
}

mod sort {

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

        for current_element in 0..num_elements {
            // Go through the complete array.

            // Find the position of current_element
            // A do while kind of loop is used here as I have to go through the elements as many times
            // until I do not need to swap anymore.
            loop {
                let mut position_to_swap = current_element;
                for comparing_element in (current_element + 1)..num_elements {
                    match monotonicity {
                        Monotonicity::Increasing => {
                            if elements[current_element] >= elements[comparing_element] {
                                position_to_swap = position_to_swap + 1;
                            }
                        }
                        Monotonicity::Decreasing => {
                            if elements[current_element] <= elements[comparing_element] {
                                position_to_swap = position_to_swap + 1;
                            }
                        }
                    }
                }
                if position_to_swap > current_element {
                    if elements[position_to_swap] == elements[current_element] {
                        match monotonicity {
                            Monotonicity::Increasing => {
                                // We break the cycle if the source and destination are the same
                                // as otherwise, endless loop would occur.
                                // Besides, that means that the current_element is already
                                // in the position it should be.
                                break;
                            }
                            Monotonicity::Decreasing => {
                                // If the values to be swapped are the same when the monotonicity is decreasing
                                // Then we end up in an endless loop, so for the case of elements[current_element]
                                // we need to do compare only if it is smaller. So this wastes runtime here.

                                // One optimization could be to store the numbers that repeat, and check that before
                                // to decide if the comparison needs to be le or only lt.
                                position_to_swap = current_element;
                                for comparing_element in (current_element + 1)..num_elements {
                                    if elements[current_element] < elements[comparing_element] {
                                        position_to_swap = position_to_swap + 1;
                                    }
                                }
                                if elements[position_to_swap] == elements[current_element] {
                                    break;
                                }
                            }
                        }
                    }
                    // ToDo: Better swapping
                    let temp = elements[position_to_swap].clone();
                    elements[position_to_swap] = elements[current_element].clone();
                    elements[current_element] = temp;
                } else {
                    // If position_to_swap is the same as the first current element,
                    // It means that I did not find any number less than this, and thus
                    // this number is in the correct position.
                    break;
                }
            }
        }
    }
}
