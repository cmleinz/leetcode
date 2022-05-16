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

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => None,
        (Some(l1), None) => Some(l1),
        (None, Some(l2)) => Some(l2),
        (Some(l1), Some(l2)) => {
            let sum = l1.val + l2.val;
            if sum < 10 {
                Some(Box::new(ListNode {
                    val: sum,
                    next: add_two_numbers(l1.next, l2.next),
                }))
            } else {
                Some(Box::new(ListNode {
                    val: sum - 10,
                    next: add_two_numbers(Some(Box::new(ListNode::new(1))), l1.next),
                }))
            }
        }
    }
}

mod test {
    #[test]
    fn add_two_numbers1() {
        assert_eq!(add_two_numbers(l1, l2), l3)
    }
}
