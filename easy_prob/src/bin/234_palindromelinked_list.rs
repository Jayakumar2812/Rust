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
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut reverse = 0;
        let mut actual = 0;
        let mut iter_count =0;
        let mut temp_head = head.as_ref();
        while (temp_head != None) {
            actual = actual*10 + temp_head.as_ref().unwrap().val;
            reverse =  reverse + temp_head.as_ref().unwrap().val* 10_i32.pow(iter_count);
            iter_count +=1;
            temp_head =  temp_head.unwrap().next.as_ref();
        }
        return reverse == actual
    
    }
}