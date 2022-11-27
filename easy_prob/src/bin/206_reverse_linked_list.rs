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
fn to_linked_list(vector: Vec<i32>)-> Option<Box<ListNode>>{
    let mut cur = None;
    for &value in vector.iter().rev(){
        let mut new_node = ListNode::new(value);
        new_node.next = cur;
        cur = Some(Box::new(new_node));
    }
    cur
    
}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            
        let mut list_of_values:Vec<i32> = vec![];
        let mut  cur_head = head.clone();

        while (!cur_head.is_none()) {
            list_of_values.push(cur_head.as_ref().unwrap().val);
            cur_head = cur_head.unwrap().next;
        }
        let reversed_values:Vec<i32> = list_of_values.into_iter().rev().collect();
        to_linked_list(reversed_values)
    }
}
