pub mod test_continuous_array;

use std::cmp::max;
use std::collections::VecDeque;


/// Returns number `<i32>` of changes required to convert to continuous array.
pub fn num_changes(array: &[i32], length: i32) -> i32 {
    let lowest_val = array[0];
    let highest_val = array[length as usize-1];

    length - count_best_array(array, length, lowest_val, highest_val)
}

/// Returns largest number `<i32>` of unique values contained in one continuous array.
fn count_best_array(array: &[i32], length: i32, lowest_val: i32, highest_val: i32) -> i32 {
    /* Explanation of logic:
     We move a window of length array.len() across the number line, trying to find one window
     which captures as many unique values from the array as possible.

     As we shift the window, we only care about the value about to leave the window, and the value
     about to enter the window. For the entering value, we check if it exists in the array.
     For the exiting value, we take advantage of the search we did from when this value entered the
     array. Through use of a queue, we set up a sort of delay between when the value enters the
     array and when it leaves. Thus, it only requires one search of the array to do two jobs.

     Along with that, there may be significant gaps between values of the array, larger than the
     size of the window. To skip the gap, we calculate the next non-isolated value in the array
     using value_to_jump(), and "jump" the gap.
    */

    // The removal queue acts as the delay between a value entering the window and exiting.
    let mut removal_queue = VecDeque::from(vec![None; length as usize - 1]);

    let mut count = 1;
    let mut max_count = 1;

        let mut low_index = 0;

        let mut high = lowest_val + 1;
        let mut low = high - length;


        removal_queue.push_back(Some(0));
        while high <= highest_val {
            self.if_checks += 1;
            if count == 0 { // jumping!
                high = match self.value_to_jump(array, length, low_index) {
                    Some(i) => i,
                    None => break
                };
            }

            match removal_queue.pop_front().expect("Queue is empty") {
                Some(i) => {
                    count -= 1;
                    low_index = i;
                },
                None => ()
            }

            self.contains_checks += 1; //TODO: for debugging
            match array.binary_search(&high) {
                Ok(i) => {
                    count += 1;
                    removal_queue.push_back(Some(i));
                }
                Err(_) => {
                    removal_queue.push_back(None);
                }
            }

        max_count = max(max_count, count);

            high += 1;
            low = high - length;
        }
        max_count
    }
}

/// Returns [`Some<i32>`] of next non-isolated value.
///
/// Returns [`None`] if at end of array.
fn value_to_jump(array: &[i32], length: i32, current_index: usize) -> Option<i32> {
    let mut current_val;
    let mut next_val;

    current_val = match array.get(current_index) {
        Some(i) => *i,
        None => return None
    };

    next_val = match array.get(current_index+1) {
        Some(i) => *i,
        None => return Some(current_val)
    };

    let mut shift = 1;
    while current_val == next_val || current_val + length < next_val {
        shift += 1;
        current_val = next_val;

        next_val = match array.get(current_index + shift) {
            Some(i) => *i,
            None => return Some(current_val)
        };
    }

    Some(current_val)
}
