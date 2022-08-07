fn main() {
    println!("{:?}",str_str("aaa".to_string(),"aaaa".to_string()));
}


pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.len() == 0 {
        return 0 ; 
    }
    let vec:Vec<_> = haystack.chars().collect();
    let to_find:Vec<_> = needle.chars().collect();
    for  (i) in 0..vec.len(){
        if vec[i]== to_find[0] {
            let mut temp = i;
            for j in to_find.iter(){
                if temp == vec.len() {
                    break
                }
                if vec[temp] == *j {
                    temp +=1;
                }
                else {
                    break
                }
            }
            if (i+to_find.len()) == temp{
                return  i as i32 ;
            }
        }
    }
    return -1 ;  
}