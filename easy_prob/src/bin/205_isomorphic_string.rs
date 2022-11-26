use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
     let mut char_to_char:HashMap<char,char> = HashMap::with_capacity(s.len());   
     let mut paired:HashSet<char> = HashSet::with_capacity(s.len());
     let my_chars: Vec<char> = s.chars().collect();
     for (i,c) in t.chars().enumerate() {
        match char_to_char.get(&c) {
        Some(charac) => {
           if  my_chars[i] == *charac {
            continue
           }
           else {
            return false
           }
        }
        None =>{
            if paired.contains(&my_chars[i]) {
                return false
            }
            else  {
                char_to_char.insert(c,my_chars[i]);
                paired.insert(my_chars[i]);
            }
        }
       }; 
     }
    return true
    }
}
