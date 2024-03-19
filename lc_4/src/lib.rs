pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let max_length = nums1.len() + nums2.len();
        let even_offset = (max_length + 1) % 2;
        let median_index = (max_length / 2) - even_offset;
        let mut nums1_index = 0;
        let mut nums2_index = 0;
        let mut medians = Vec::new();

        while nums1_index + nums2_index <= median_index + even_offset {
            let cur1 = if nums1_index < nums1.len() {
                nums1[nums1_index]
            } else {
                i32::MAX
            };

            let cur2 = if nums2_index < nums2.len() {
                nums2[nums2_index]
            } else {
                i32::MAX
            };

            let selected;
            let mut next1_index = nums1_index;
            let mut next2_index = nums2_index;

            if cur1 >= cur2 {
                selected = cur2;
                next2_index += 1;
            } else {
                selected = cur1;
                next1_index += 1;
            }

            let index_sum = nums1_index + nums2_index;
            if index_sum == median_index || index_sum == median_index + even_offset {
                medians.push(selected);
            }

            nums1_index = next1_index;
            nums2_index = next2_index;
        }

        medians.iter().fold(0, |acc, cur| acc + cur) as f64 / medians.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Input: nums1 = [1,3], nums2 = [2]
        // Output: 2.00000
        // Explanation: merged array = [1,2,3] and median is 2.
        let result = Solution::find_median_sorted_arrays(vec![1, 3], vec![2]);
        assert_eq!(result, 2.00000);
    }

    #[test]
    fn it_works_with_second_example() {
        // Input: nums1 = [1,2], nums2 = [3,4]
        // Output: 2.50000
        // Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
        let result = Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]);
        assert_eq!(result, 2.50000);
    }
}
