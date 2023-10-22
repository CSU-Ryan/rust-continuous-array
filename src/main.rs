use crate::continuous_array::test_continuous_array;

mod continuous_array;
pub mod inputs;

fn main() {
    test_continuous_array::continuous_array_loader();
}

// For LeetCode submission

// impl Solution {
//     pub fn min_operations(mut nums: Vec<i32>) -> i32 {
//         nums.sort();
//         num_changes(&nums, (nums.len() as i32);
//     }
// }
