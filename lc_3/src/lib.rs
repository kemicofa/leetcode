pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();

        let mut start_index = 0;
        let mut end_index = 0;

        let mut longest_count = 0;

        while end_index < bytes.len() {
            let byte = bytes[end_index];

            for index in start_index..end_index {
                let seen_byte = bytes[index];
                if seen_byte != byte {
                    continue;
                }
                start_index = index + 1;
                break;
            }

            let current_count = end_index - start_index + 1;
            if current_count > longest_count {
                longest_count = current_count;
            }
            end_index += 1;
        }
        longest_count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Input: s = "abcabcbb"
        // Output: 3
        // Explanation: The answer is "abc", with the length of 3.

        let result = Solution::length_of_longest_substring("abcabcbb".into());
        assert_eq!(result, 3);
    }

    #[test]
    fn it_works_with_second_example() {
        // Input: s = "bbbbb"
        // Output: 1
        // Explanation: The answer is "b", with the length of 1.
        let result = Solution::length_of_longest_substring("bbbbb".into());
        assert_eq!(result, 1);
    }

    #[test]
    fn it_works_with_third_example() {
        // Input: s = "pwwkew"
        // Output: 3
        // Explanation: The answer is "wke", with the length of 3.
        // Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
        let result = Solution::length_of_longest_substring("pwwkew".into());
        assert_eq!(result, 3);
    }

    #[test]
    fn it_works_with_dvdf() {
        // "dvdf"
        let result = Solution::length_of_longest_substring("dvdf".into());
        assert_eq!(result, 3);
    }
}
