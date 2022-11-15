#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
pub struct  Solution {

}
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ret = Vec::new();
    Self::_preorder_traversal(root.as_ref(), &mut ret);
    ret
}
    fn _preorder_traversal(root: Option<&Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
    if let Some(node) = root {
        let nodeb = node.borrow();
        Self::_preorder_traversal(nodeb.left.as_ref(), ret);
        Self::_preorder_traversal(nodeb.right.as_ref(), ret);
        ret.push(nodeb.val);
    }
}
}


fn main() {
    
}