use std::collections::HashSet;
impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let set: HashSet<&i32> = HashSet::from_iter(&nums);
        if nums.len() == set.len() {
            return false
        } else {
            return true
        }
    }
}
