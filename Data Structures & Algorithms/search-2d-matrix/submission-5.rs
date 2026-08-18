impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut row = None;
        for (i, u) in matrix.iter().enumerate() {
            if u[0] <= target {
                row = Some(i);
            } else {
                break;
            }
        }
        let row = match row {
            Some(r) => &matrix[r],
            None => return false,
        };
        let mut inner_ind = (0, (row.len() - 1) as i32);
        while inner_ind.0 as i32 <= inner_ind.1 as i32 {
            let mid_point = (inner_ind.0 as i32 + inner_ind.1 as i32) / 2;
            if row[mid_point as usize] == target {
                return true
            }
            if row[mid_point as usize] > target {
                inner_ind.1 = mid_point - 1;
            } else {
                inner_ind.0 = mid_point + 1;
            }
        }
        false
    }
}
