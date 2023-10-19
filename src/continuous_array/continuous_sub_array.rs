use crate::continuous_array::ContinuousArray;

pub struct ContinuousSubArray {
    pub start: i32,
    pub length: i32
}

impl ContinuousSubArray {
    pub fn new(start: i32) -> Self {
        ContinuousSubArray {
            start,
            length: 0
        }
    }

    pub fn clone(&self) -> Self {
        ContinuousSubArray {
            start: self.start,
            length: self.length
        }
    }
}

impl ContinuousArray {
    fn count_sub_array (&mut self, low: i32) -> i32 {
        let mut count = 0;
        for i in low..(low +self.length) {
            self.contains_checks += 1; //TODO: for debugging
            if self.array.contains(&i) {
                count += 1;
            }
        }
        count
    }

    fn find_sub_arrays(&mut self) {
        self.array.sort();
        let mut prev_i = self.array[0];
        let mut current_subarray = ContinuousSubArray::new(prev_i);
        current_subarray.length += 1;

        for i in &self.array[1..] {
            if prev_i + 1 != *i && prev_i + 2 != *i { // if not sequential
                if current_subarray.length > 1 {
                    self.sub_arrays.push(current_subarray.clone());
                }
                current_subarray = ContinuousSubArray::new(*i);
            }
            current_subarray.length += 1;
            prev_i = *i;
        }
        self.sub_arrays.push(current_subarray);
    }

    pub fn largest_subarray(&mut self) {
        let mut largest = self.sub_arrays[0].clone();
        self.sub_arrays.remove(0);
        for subarray in &self.sub_arrays {
            if largest.length < subarray.length {
                largest = subarray.clone();
            }
        }
        self.subarray = largest;
    }
}