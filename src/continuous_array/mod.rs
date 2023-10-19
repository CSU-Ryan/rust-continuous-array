pub mod test_continuous_array;
// pub mod continuous_sub_array;

use std::cmp::max;
use std::collections::VecDeque;
use std::thread::current;
// use crate::continuous_array::continuous_sub_array::ContinuousSubArray;

fn binary_contains(array: &[i32], val: &i32) -> bool {
    match array.binary_search(val) {
        Ok(_) => true,
        Err(_) => false
    }
}

pub struct ContinuousArray {
    // pub length: i32,
    // pub array: Vec<i32>,

    pub if_checks: usize,
    pub contains_checks: usize, //TODO: for debugging purposes

    // pub subarray: ContinuousSubArray,
    // pub sub_arrays: Vec<ContinuousSubArray>
}


impl ContinuousArray {
    /// Turn a [`Vec<i32>`] into a [`ContinuousArray`].
    pub fn new() -> Self {
        // let mut array = array.clone();
        // array.sort();

        ContinuousArray {
            // length: array.len() as i32,
            // array: array,

            if_checks: 0, //TODO: for debugging
            contains_checks: 0, // TODO: for debugging purposes

            // subarray: ContinuousSubArray::new(0),
            // sub_arrays: Vec::new()
        }
    }

    /// Returns number `<i32>` of changes required to convert to continuous array.
    pub fn num_changes(&mut self, array: &[i32], length: i32) -> i32 {
        let lowest_val = array[0];
        let highest_val = array[length as usize-1];
        // let lowest_val = *self.array.iter().min().expect("Array is empty");
        // let highest_val = *self.array.iter().max().expect("Array is empty");

        // println!("{:?}", self.array);
        length - self.count_best_array(array, length, lowest_val, highest_val)
    }

    /// Returns [`Some<i32>`] of next value to jump over gap.
    ///
    /// Returns [`None`] if at end of array.
    fn value_to_jump(&mut self, array: &[i32], length: i32, current_index: usize) -> Option<i32> {
        let mut current_val;
        let mut next_val;

        current_val = match array.get(current_index) {
            Some(i) => *i,
            None => return None // TODO: figure out return
        };

        next_val = match array.get(current_index+1) {
            Some(i) => *i,
            None => return Some(current_val)
        };

        // println!("current value for jumping: {}", current_val);

        let mut shift = 1;
        while current_val == next_val || current_val + length < next_val {
            self.if_checks += 1;
            // println!("skipped {}", current_val);
            shift += 1;
            current_val = next_val;

            next_val = match array.get(current_index + shift) {
                Some(i) => *i,
                None => return Some(current_val)
            }
        }

        Some(current_val)
    }

    /// Returns largest number `<i32>` of unique values contained in one continuous array.
    fn count_best_array(&mut self, array: &[i32], length: i32, lowest_val: i32, highest_val: i32) -> i32 {
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

