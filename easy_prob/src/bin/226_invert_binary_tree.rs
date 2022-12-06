// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use std::mem;
type TreeLink = Option<Rc<RefCell<TreeNode>>>;
// copied solution
impl Solution {
    fn invert_tree(root: TreeLink) -> TreeLink {
        let mut stack: Vec<TreeLink> = vec!(root.clone());
        while let Some(ele) = stack.pop() {
            if let Some(n) = ele {
                {
                    let TreeNode { left, right, .. } = &mut *n.borrow_mut();
                    mem::swap( right,  left);
                }
                stack.push(n.borrow_mut().right.clone());
                stack.push(n.borrow_mut().left.clone());
            }
        }
        root
    }
}
