// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }

// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::add_two(&l1, &l2, 0)
    }

    fn add_two(l1: &Option<Box<ListNode>>, l2: &Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => { 
                if carry == 0 {
                    return None;
                }
                Some(Box::new(ListNode::new(carry)))
            }
            (None, Some(n2)) => Self::add_two(l2, l1, carry),
            (Some(n1), None) => {
                let mut l1 = n1.clone();
                let mut sum = carry + l1.val; 
                l1.val = sum % 10; 
                l1.next = Self::add_two(&n1.next, &None, sum / 10); 
                Some(l1)
            }
            (Some(n1), Some(n2)) => {
                let mut l1 = n1.clone();
                let mut l2 = n2.clone();
                let mut sum = carry + l1.val + l2.val; 
                l1.val = sum % 10; 
                l1.next = Self::add_two(&l1.next, &l2.next, sum / 10); 
                Some(l1)
            }
        }
    }
}