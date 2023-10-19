fn sub_array_counter(&mut self, lowest: i32, highest/*inclusive*/: i32) -> i32 {
    let mut count = self.count_sub_array(lowest);

    let mut max_count = count;
    let mut high;
    let mut current_array_index;

    let mut low = lowest;
    while low <= highest {
        self.contains_checks += 1; //TODO: for debugging
        if self.array.contains(&low) { // if about to exclude value from list
            count -= 1;
            if count == 0 {
                current_array_index =
                    self.array.iter().position(|&i| i == low)
                    .expect("element does not exist");


                low = match self.array.get(current_array_index+1) {
                    Some(i) => *i,
                    None => break
                };
                count = self.count_sub_array(low);
                continue;
            }
        }
        high = low + self.length;
        self.contains_checks += 1; //TODO: for debugging
        if self.array.contains(&high) { // if about to include value from list
            count += 1;
        }
        max_count = max(count, max_count);
        low += 1;
    }
    max_count
}