fn main() {
    let vec = vec![1,2];
    let linked_list = to_linked_list(vec);
    println!("{:?}",delete_duplicates(linked_list));
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}


fn to_linked_list(vector: Vec<i32>)-> Option<Box<ListNode>>{
  let mut cur = None;
  for &value in vector.iter().rev(){
      let mut new_node = ListNode::new(value);
      new_node.next = cur;
      cur = Some(Box::new(new_node));
  }
  cur   
}

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  if head == None{
    return head;
  }

  let mut mylist:Vec<i32> =vec![];

  let initial_value = &head.as_ref().unwrap().val;
  let mut previous_num = *initial_value;

  mylist.push(*initial_value); 
  let mut  cur = &head;

  while cur.as_ref().unwrap().next != None {
    
    cur = &cur.as_ref().unwrap().next;
    if cur.as_ref().unwrap().val != previous_num {
      previous_num = cur.as_ref().unwrap().val;
      mylist.push(cur.as_ref().unwrap().val);
    }
  }

  println!("{:?}",mylist);

  return to_linked_list(mylist);
}