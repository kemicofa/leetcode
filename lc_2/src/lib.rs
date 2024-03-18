#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn vec_to_list_node(digits: &mut Vec<i32>) -> Option<Box<ListNode>> {
    digits.reverse();
    let list_node = Box::new(ListNode::new(*digits.first().unwrap()));
    Some(digits[1..].iter().fold(list_node, |acc, cur| {
        let mut n_ln = ListNode::new(*cur);
        n_ln.next = Some(acc);
        Box::new(n_ln)
    }))
}

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // both lists guaranteed to have at least one list node
        let mut next1 = l1;
        let mut next2 = l2;
        let mut remainder = 0;

        let mut digits = vec![];

        loop {
            let mut next_val1 = 0;
            let mut next_val2 = 0;
            let mut done_count = 0;

            match next1 {
                Some(ln1) => {
                    next_val1 = ln1.val;
                    next1 = ln1.next;
                }
                None => {
                    done_count += 1;
                }
            }

            match next2 {
                Some(ln2) => {
                    next_val2 = ln2.val;
                    next2 = ln2.next;
                }
                None => {
                    done_count += 1;
                }
            }

            if done_count == 2 && remainder == 0 {
                break;
            }

            let sum = next_val1 + next_val2 + remainder;
            let val = sum % 10;
            remainder = sum / 10;

            digits.push(val);
        }

        println!("{digits:?}");
        vec_to_list_node(&mut digits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Input: l1 = [2,4,3], l2 = [5,6,4]
        // Output: [7,0,8]
        // Explanation: 342 + 465 = 807.
        let input1 = vec_to_list_node(&mut vec![2, 4, 3]);
        let input2 = vec_to_list_node(&mut vec![5, 6, 4]);
        let expected_result = vec_to_list_node(&mut vec![7, 0, 8]);
        let result = Solution::add_two_numbers(input1, input2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn it_works_with_second_example() {
        // Input: l1 = [0], l2 = [0]
        // Output: [0]
        let input1 = vec_to_list_node(&mut vec![0]);
        let input2 = vec_to_list_node(&mut vec![0]);
        let expected_result = vec_to_list_node(&mut vec![0]);
        let result = Solution::add_two_numbers(input1, input2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn it_works_with_third_example() {
        // Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
        // Output: [8,9,9,9,0,0,0,1]
        let input1 = vec_to_list_node(&mut vec![9, 9, 9, 9, 9, 9, 9]);
        let input2 = vec_to_list_node(&mut vec![9, 9, 9, 9]);
        let expected_result = vec_to_list_node(&mut vec![8, 9, 9, 9, 0, 0, 0, 1]);
        let result = Solution::add_two_numbers(input1, input2);
        assert_eq!(result, expected_result);
    }
}
