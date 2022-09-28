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

pub fn get_decimal_value(mut head: Option<Box<ListNode>>) -> i32 {
    let mut res = 0;

    while let Some(node) = head {
        res *= 2;
        res += node.val;
        head = node.next;
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_binary_num_in_linked_list() {
        let mut node1 = ListNode::new(1);
        let mut node2 = ListNode::new(0);
        let node3 = ListNode::new(1);

        node2.next = Some(Box::new(node3));
        node1.next = Some(Box::new(node2));

        let curr_head = Some(Box::new(node1));

        assert_eq!(get_decimal_value(curr_head), 5);
    }
}
