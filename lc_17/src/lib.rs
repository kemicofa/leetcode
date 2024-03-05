const LOWER_CASE_ASCII_START_INDEX: u8 = 97;
const LETTER_GROUP_LEN: u8 = 3;

pub fn get_letter_group(digit: u32) -> Vec<char> {
    match digit {
        7 => vec!['p', 'q', 'r', 's'],
        8 => vec!['t', 'u', 'v'],
        9 => vec!['w', 'x', 'y', 'z'],
        _ => {
            let start_index: u8 =
                LOWER_CASE_ASCII_START_INDEX + (digit as u8 - 2) * LETTER_GROUP_LEN;
            let mut letters = vec![];
            for ascii in start_index..(start_index + LETTER_GROUP_LEN) {
                letters.push(ascii as char)
            }
            letters
        }
    }
}

struct IndexStepper {
    max_index_for_each_step: Vec<usize>,
    indexes: Vec<usize>,
    done: bool,
}

impl IndexStepper {
    pub fn new(max_index_for_each_step: Vec<usize>) -> Self {
        let capacity = max_index_for_each_step.len();
        Self {
            max_index_for_each_step,
            indexes: vec![0; capacity],
            done: false,
        }
    }

    pub fn step(&mut self) {
        let mut resets_count = 0;
        for (i, current_step) in self.indexes.iter_mut().enumerate() {
            if *current_step == self.max_index_for_each_step[i] - 1 {
                *current_step = 0;
                resets_count += 1;
                continue;
            }
            *current_step += 1;
            break;
        }

        if resets_count == self.indexes.len() {
            self.done = true;
        }
    }

    pub fn get_current_indexes(&self) -> &Vec<usize> {
        &self.indexes
    }

    pub fn is_done(&self) -> bool {
        self.done
    }
}

pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.len() == 0 {
        return vec![];
    }

    let mut letter_groups: Vec<Vec<char>> = vec![];
    for c in digits.chars() {
        let digit = c.to_digit(10).unwrap();
        let letter_group = get_letter_group(digit);
        letter_groups.push(letter_group);
    }

    let mut result: Vec<String> = vec![];

    let mut index_stepper = IndexStepper::new(
        letter_groups
            .iter()
            .map(|letter_group| letter_group.len())
            .collect(),
    );

    loop {
        let indexes = index_stepper.get_current_indexes();
        let mut comb = String::default();
        for (i, index) in indexes.iter().enumerate() {
            comb.push(letter_groups[i][*index]);
        }
        result.push(comb);
        index_stepper.step();

        if index_stepper.is_done() {
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = letter_combinations("23".into());
        assert_eq!(
            result,
            ["ad", "bd", "cd", "ae", "be", "ce", "af", "bf", "cf"]
        );
    }

    #[test]
    fn it_works_with_7() {
        let result = letter_combinations("7".into());
        assert_eq!(result, ["p", "q", "r", "s"]);
    }

    #[test]
    fn it_works_with_27() {
        let result = letter_combinations("27".into());
        assert_eq!(
            result,
            ["ap", "bp", "cp", "aq", "bq", "cq", "ar", "br", "cr", "as", "bs", "cs"]
        );
    }

    #[test]
    fn it_works_with_99() {
        let result = letter_combinations("99".into());
        assert_eq!(
            result,
            [
                "ww", "xw", "yw", "zw", "wx", "xx", "yx", "zx", "wy", "xy", "yy", "zy", "wz", "xz",
                "yz", "zz"
            ]
        );
    }

    #[test]
    fn it_works_with_9387() {
        let result = letter_combinations("9387".into());
        assert_eq!(
            result,
            [
                "wdtp", "xdtp", "ydtp", "zdtp", "wetp", "xetp", "yetp", "zetp", "wftp", "xftp",
                "yftp", "zftp", "wdup", "xdup", "ydup", "zdup", "weup", "xeup", "yeup", "zeup",
                "wfup", "xfup", "yfup", "zfup", "wdvp", "xdvp", "ydvp", "zdvp", "wevp", "xevp",
                "yevp", "zevp", "wfvp", "xfvp", "yfvp", "zfvp", "wdtq", "xdtq", "ydtq", "zdtq",
                "wetq", "xetq", "yetq", "zetq", "wftq", "xftq", "yftq", "zftq", "wduq", "xduq",
                "yduq", "zduq", "weuq", "xeuq", "yeuq", "zeuq", "wfuq", "xfuq", "yfuq", "zfuq",
                "wdvq", "xdvq", "ydvq", "zdvq", "wevq", "xevq", "yevq", "zevq", "wfvq", "xfvq",
                "yfvq", "zfvq", "wdtr", "xdtr", "ydtr", "zdtr", "wetr", "xetr", "yetr", "zetr",
                "wftr", "xftr", "yftr", "zftr", "wdur", "xdur", "ydur", "zdur", "weur", "xeur",
                "yeur", "zeur", "wfur", "xfur", "yfur", "zfur", "wdvr", "xdvr", "ydvr", "zdvr",
                "wevr", "xevr", "yevr", "zevr", "wfvr", "xfvr", "yfvr", "zfvr", "wdts", "xdts",
                "ydts", "zdts", "wets", "xets", "yets", "zets", "wfts", "xfts", "yfts", "zfts",
                "wdus", "xdus", "ydus", "zdus", "weus", "xeus", "yeus", "zeus", "wfus", "xfus",
                "yfus", "zfus", "wdvs", "xdvs", "ydvs", "zdvs", "wevs", "xevs", "yevs", "zevs",
                "wfvs", "xfvs", "yfvs", "zfvs"
            ]
        );
    }
}
