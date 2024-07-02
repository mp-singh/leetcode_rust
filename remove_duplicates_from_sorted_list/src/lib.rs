pub struct Solution {}

// Definition for singly-linked list.
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

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = &mut head;

        while let Some(ref mut node) = current {
            while let Some(ref next) = node.next {
                if node.val == next.val {
                    node.next = next.next.clone();
                } else {
                    break;
                }
            }
            current = &mut node.next;
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &val in vec.iter().rev() {
            let mut new_node = Box::new(ListNode::new(val));
            new_node.next = current;
            current = Some(new_node);
        }
        current
    }

    fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = Vec::new();
        while let Some(node) = head {
            vec.push(node.val);
            head = node.next;
        }
        vec
    }

    #[test]
    fn test_delete_duplicates() {
        let input = vec![1, 1, 2, 3, 3];
        let expected_output = vec![1, 2, 3];

        let input_list = vec_to_list(input);

        let result_list = Solution::delete_duplicates(input_list);
        let result_vec = list_to_vec(result_list);

        assert_eq!(result_vec, expected_output);
    }
}
