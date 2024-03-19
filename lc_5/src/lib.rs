pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() == 1 {
            return s;
        }

        let bytes = s.as_bytes();

        let is_palindrome = |start_index: usize, end_index: usize| {
            let len = end_index + 1 - start_index;
            for i in 0..(len / 2) {
                if bytes[start_index + i] == bytes[end_index - i] {
                    continue;
                }
                return false;
            }
            true
        };

        let mut take = bytes.len() - 1;
        let threshold = bytes.len() - 1;

        loop {
            for i in 0..threshold {
                let end_index = i + take;
                if end_index > threshold {
                    break;
                }
                if !is_palindrome(i, end_index) {
                    continue;
                }
                return s[i..=end_index].to_string();
            }
            take -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Input: s = "babad"
        // Output: "bab"
        // Explanation: "aba" is also a valid answer.
        let result = Solution::longest_palindrome("babad".into());
        assert_eq!(result, "bab");
    }

    #[test]
    fn it_works_with_second_example() {
        // Input: s = "cbbd"
        // Output: "bb"
        let result = Solution::longest_palindrome("cbbd".into());
        assert_eq!(result, "bb");
    }

    #[test]
    fn it_works_with_ac() {
        let result = Solution::longest_palindrome("ac".into());
        assert_eq!(result, "a");
    }
}
