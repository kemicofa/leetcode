pub struct Solution;

impl Solution {
    pub fn int_to_romain(num: i32) -> String {
        let mut cur = num;
        let mut res: Vec<String> = Vec::default();

        while cur >= 1 {
            let digits = cur.ilog10() + 1;
            let factor = 10_i32.pow(digits - 1);
            let first_digit = cur / 10_u64.pow(digits - 1) as i32;
            let stripped = (cur / factor) * factor;

            let part: String = match stripped {
                1000.. => "M".repeat(first_digit as usize),
                900 => "CM".into(),
                500..900 => {
                    let mut s = "D".to_string();
                    let left = (stripped - 500) / 100;
                    if left > 0 {
                        s.push_str(&"C".repeat(left as usize));
                    }
                    s
                }
                400 => "CD".into(),
                100..400 => "C".repeat(first_digit as usize),
                90 => "XC".into(),
                50..90 => {
                    let mut s = "L".to_string();
                    let left = (stripped - 50) / 10;
                    if left > 0 {
                        s.push_str(&"X".repeat(left as usize));
                    }
                    s
                }
                40 => "XL".into(),
                10..40 => "X".repeat(first_digit as usize),
                9 => "IX".into(),
                5..9 => {
                    let mut s = "V".to_string();
                    let left = stripped - 5;
                    if left > 0 {
                        s.push_str(&"I".repeat(left as usize));
                    }
                    s
                }
                4 => "IV".into(),
                1..4 => "I".repeat(first_digit as usize),
                _ => panic!("Never supposed to be here"),
            };

            println!("{digits}, {cur}, {factor}, {stripped}, {part}");
            res.push(part);

            cur -= stripped;
        }

        res.join("")
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn should_return_iv() {
        let romain = Solution::int_to_romain(4);
        assert_eq!("IV".to_string(), romain);
    }

    #[test]
    fn should_return_ix() {
        let romain = Solution::int_to_romain(9);
        assert_eq!("IX".to_string(), romain);
    }

    #[test]
    fn should_return_xl() {
        let romain = Solution::int_to_romain(40);
        assert_eq!("XL".to_string(), romain);
    }

    #[test]
    fn should_return_mmmdccxlix() {
        let romain = Solution::int_to_romain(3749);
        assert_eq!("MMMDCCXLIX".to_string(), romain);
    }

    #[test]
    fn should_return_mmmcmxcix() {
        let romain = Solution::int_to_romain(3999);
        assert_eq!("MMMCMXCIX".to_string(), romain);
    }

    #[test]
    fn should_handle_subtractive() {
        let cases: [(i32, &str); 6] = [
            (4, "IV"),
            (9, "IX"),
            (40, "XL"),
            (90, "XC"),
            (400, "CD"),
            (900, "CM"),
        ];

        for case in cases {
            assert_eq!(Solution::int_to_romain(case.0), case.1);
        }
    }

    #[test]
    fn should_handle_multiple_subtractive_rules() {
        let cases: [(i32, &str); 6] = [
            (49, "XLIX"),
            (99, "XCIX"),
            (444, "CDXLIV"),
            (944, "CMXLIV"),
            (1994, "MCMXCIV"),
            (3999, "MMMCMXCIX"),
        ];

        for case in cases {
            assert_eq!(Solution::int_to_romain(case.0), case.1);
        }
    }

    #[test]
    fn should_handle_repeated_numerals() {
        let cases: [(i32, &str); 6] = [
            (2, "II"),
            (3, "III"),
            (20, "XX"),
            (30, "XXX"),
            (300, "CCC"),
            (3000, "MMM"),
        ];
        for case in cases {
            assert_eq!(Solution::int_to_romain(case.0), case.1);
        }
    }
}
