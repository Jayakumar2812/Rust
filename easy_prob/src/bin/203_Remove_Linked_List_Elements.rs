// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

// copied from dicussion forum

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut walker = &mut head;
        loop {
            match walker {
                None => break,
                Some(node) if node.val == val => {
                    *walker = node.next.take();
                },
                Some(node) => {
                    walker = &mut node.next;
                },
            }
        }
        head
    }
}
