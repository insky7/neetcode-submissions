use std::collections::HashSet;
impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let set: HashSet<&i32> = nums.iter().collect();
        if set.len() == nums.len() {
            return false
        } else {
            return true
        }
    }
}
