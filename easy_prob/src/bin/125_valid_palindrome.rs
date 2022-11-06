fn main() {
    let s:String = "a".to_string();
    println!("{}",is_palindrome(s))
} 

pub fn is_palindrome(s: String) -> bool {
    let mut string = vec![]; 
    for  i in s.chars() {
    if i.is_alphanumeric() {
        let j = i.to_lowercase().to_string();
        string.push(j);
    }
    }
    let len = string.len();
    for  i in 0..len/2 {
        if string[i] != string [ len -i -1] {
            return false;
        }
        
    }

    true 
}