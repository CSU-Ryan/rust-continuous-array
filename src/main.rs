mod continuous_array;
pub mod inputs;

use crate::continuous_array::test_continuous_array;


fn main() {
    test_continuous_array::continuous_array_loader();
    // test_continuous_array::big_test();
}

// For LeetCode submission

// impl Solution {
//     pub fn min_operations(nums: Vec<i32>) -> i32 {
//         let mut cont_array = ContinuousArray::from(nums);
//
//         cont_array.num_changes()
//     }
// }