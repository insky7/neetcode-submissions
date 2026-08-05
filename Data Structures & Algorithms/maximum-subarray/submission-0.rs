impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut curSum = 0;
        let mut maxSum = nums[0];
        for n in nums {
            if curSum < 0 {
                curSum = 0;
            }
            curSum += n;
            maxSum = max(curSum, maxSum);
        }
        return maxSum
    }
}
