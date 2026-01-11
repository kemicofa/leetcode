use std::cmp::min;

pub struct Solution {

}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {

        let mut left_pointer = 0;
        let mut right_pointer = height.len() - 1;
        let mut max_area = 0;

        while left_pointer != right_pointer {
            let nmax_area = (right_pointer - left_pointer) as i32 * min(height[left_pointer], height[right_pointer]);

            if height[left_pointer] > height[right_pointer] {
                right_pointer -= 1;
            } else {
                left_pointer += 1;
            }

            if nmax_area > max_area {
                max_area = nmax_area;
            }
        }

       max_area 
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn should_return_49() {
        let input = Vec::from([1,8,6,2,5,4,8,3,7]);
        let expected = 49;

        let output = Solution::max_area(input);
        assert_eq!(output, expected);
    }
}