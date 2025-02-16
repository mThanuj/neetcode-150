use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();

    for num in nums {
        if set.contains(&num) {
            return true;
        }
        set.insert(num);
    }

    false
}
