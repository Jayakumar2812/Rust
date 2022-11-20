impl Solution {
pub fn title_to_number(column_title: String) -> i32 {
    let vec :Vec<char> = column_title.chars().collect();  
    let length = vec.len();
    let  mut iter_count = 0;
    let mut res = 0;
    for i in (0..length).rev() {
        let mut cur_val = 0;
        cur_val = (vec[i] as u32)as i32 -64;
        if iter_count == 0 {
            res +=  cur_val;
            iter_count +=1;
        }
        else {
            res += cur_val * 26_i32.pow(iter_count);
            // println!("{}",res);
            iter_count +=1;
        }
    }


    return res
}

}
