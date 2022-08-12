fn main() {
    println!("{:?}",plus_one(vec![9,9,9]));
}


pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    
    let last_index = digits.len() -1 ; 
    let mut copy = digits.clone();
    if &copy[last_index] != &9 {
        copy[last_index]  += 1 ;
        return copy ;
    }
    else {
        recusion(&mut copy,last_index as i32);
    }

    copy
}
pub fn recusion(copy:&mut Vec<i32>,index:i32) -> &mut Vec<i32> {
    if index < 0 {
        copy.insert(0, 1);
        return copy
    }
    copy [index as usize] += 1;
    if (copy[index as usize]) >=10 {
        copy[index as usize] = 0;
        recusion(copy, index -1);
    } 
    copy
}