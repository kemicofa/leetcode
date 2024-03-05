pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        if x >= 0 && x <= 9 {
            return true;
        }

        let mut normal = x;
        let mut reverse = 0;

        loop {
            if normal == 0 {
                break;
            }

            reverse = (reverse * 10) + (normal % 10);
            normal /= 10;
        }

        x == reverse
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::is_palindrome(121);
        assert_eq!(result, true);
    }

    #[test]
    fn it_does_not_pass_with_negative_number() {
        let result = Solution::is_palindrome(-121);
        assert_eq!(result, false);
    }

    #[test]
    fn it_does_not_pass_with_non_palindrome_numbers() {
        let result = Solution::is_palindrome(10);
        assert_eq!(result, false);
    }
}
