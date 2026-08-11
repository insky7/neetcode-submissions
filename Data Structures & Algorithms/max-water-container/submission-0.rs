impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, ((heights.len() - 1) as i32));
        let mut res = 0;
        while l < r {
            let area = (r - l) * min(heights[l as usize], heights[r as usize]);
            res = max(res, area);
            if heights[l as usize] < heights[r as usize] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        res
    }
}
