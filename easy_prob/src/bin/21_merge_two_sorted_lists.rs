use std::option::Option;
fn main() {

    let p1 = to_linked_list(vec![]);
    let p2 = to_linked_list(vec![0]);
    println!("{:?}",merge_two_lists(p1,p2));
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct ListNode {
    val:i32,
    next:Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val:i32) -> ListNode {
        ListNode{val, next: None}
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

pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut merged_vec = Vec::new();
    let mut i = list1;
    let mut j = list2;
    while (!i.as_ref().is_none() || !j.as_ref().is_none()){

        if i.as_ref().is_none() == true {
            merged_vec.push(j.as_ref().unwrap().val);
            j = j.unwrap().next;
        }

        else if j.as_ref().is_none() == true {
            merged_vec.push(i.as_ref().unwrap().val);
            i = i.unwrap().next;
        }

        else if i.as_ref().unwrap().val == j.as_ref().unwrap().val {
            merged_vec.push(i.as_ref().unwrap().val);
            merged_vec.push(j.as_ref().unwrap().val);
            i = i.unwrap().next;
            j = j.unwrap().next;
        } 
        else if i.as_ref().unwrap().val < j.as_ref().unwrap().val{
            merged_vec.push(i.as_ref().unwrap().val);
            i = i.unwrap().next;

        }
        else {
            // println!("{}:?",&j.as_ref().unwrap().val);
            merged_vec.push(j.as_ref().unwrap().val);
            j = j.unwrap().next;
        }
    }

    let m = to_linked_list(merged_vec);
    m
}



