fn main() {
    println!("{}",length_of_last_word("luffy is still joyboy".to_string()));
}


pub fn length_of_last_word(s: String) -> i32 {
    
    let mut c =0;
    let mut num =0;
    for i in s.chars().rev() {
        if i != ' '{
            num = 1;
            c+=1
        }
        else if num ==1 {
            break
        }
    }

    return c;
}


 