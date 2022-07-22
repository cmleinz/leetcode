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

type BoxNode = Option<Box<ListNode>>;

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    merge_nodes(l1, l2, 0, ListNode::new(-1))
}

pub fn merge_nodes(mut l1: BoxNode, mut l2: BoxNode, mut val: i32, mut out: ListNode) -> BoxNode {
    if l1.is_none() && l2.is_none() && val == 10 {
        return None;
    }
    if let Some(l1_node) = l1 {
        val += l1_node.val;
        l1 = l1_node.next 
    }
    if let Some(l2_node) = l2 {
        val += l2_node.val;
        l2 = l2_node.next 
    } 
    out.val = val % 10;
    out.next = merge_nodes(l1, l2, val / 10, ListNode::new(-1));
    Some(Box::new(out))
}