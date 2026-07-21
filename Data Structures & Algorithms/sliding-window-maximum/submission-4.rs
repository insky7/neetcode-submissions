impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ans = Vec::with_capacity(((nums.len() as i32 - k) + 1) as usize);
        for a in nums.windows(k.try_into().unwrap()) {
            ans.push(*a.iter().max().unwrap());
        }
        ans
    }
}
