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
    fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut count = 0;
        let mut current = head.clone();
        let mut middle = head;
        while current.is_some() {
            if count % 2 == 1 {
                middle = middle.unwrap().next;
            }
            current = current.unwrap().next;
            count += 1;
        }
        Some(middle.unwrap())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_middle_node() {
        let mut head = Some(Box::new(ListNode::new(1)));
        head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
        head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
        head.as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next = Some(Box::new(ListNode::new(4)));
        head.as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next = Some(Box::new(ListNode::new(5)));

        let mut expected = Some(Box::new(ListNode::new(3)));
        expected.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
        expected.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(5)));
        assert_eq!(ListNode::middle_node(head), expected);
    }
}
