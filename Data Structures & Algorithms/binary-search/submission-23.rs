impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut pointer = (0 as i32, nums.len() as i32 - 1);
        while pointer.1 >= pointer.0 {
            let a = (pointer.0 + pointer.1) / 2;
            if nums[a as usize] == target {
                return a;
            } else if nums[a as usize] > target {
                pointer.1 = a - 1;
            } else if nums[a as usize] < target {
                pointer.0 = a + 1;
            } else {
                return -1;
            } 
        }
        -1
    }
}
