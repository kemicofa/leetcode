use std::collections::HashMap;

pub fn make_equal(words: Vec<String>) -> bool {
    let map: HashMap<char, usize> = HashMap::new();
    words
        .iter()
        .fold(map, |mut acc, cur| {
            cur.chars().for_each(|c| {
                if let Some(entry) = acc.get_mut(&c) {
                    *entry += 1;
                } else {
                    acc.insert(c, 1);
                }
            });
            acc
        })
        .values()
        .all(|count| count % words.len() == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(make_equal(vec![
            String::from("abc"),
            String::from("aabc"),
            String::from("bc")
        ]));
    }

    #[test]
    fn it_works_too() {
        assert!(make_equal(vec![
            String::from("bb"),
            String::from("cc"),
        ]));
    }

    #[test]
    fn it_does_not_work() {
        assert_eq!(make_equal(vec![
            String::from("b"),
            String::from("cc"),
        ]), false);
    }
}
