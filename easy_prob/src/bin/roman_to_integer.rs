use std::collections::HashMap;
fn main() {
   println!("{}",roman_to_int("IVV".to_string()));
}
pub fn roman_to_int(s: String) -> i32 {
     let mut lookup_table = HashMap::new();
     lookup_table.insert("I",1);
     lookup_table.insert("IV",4);
     lookup_table.insert("IX",9);
     lookup_table.insert("V",5);
     lookup_table.insert("X",10);
     lookup_table.insert("XL",40);
     lookup_table.insert("XC",90);
     lookup_table.insert("L",50);
     lookup_table.insert("C",100);
     lookup_table.insert("CD",400);
     lookup_table.insert("CM",900);
     lookup_table.insert("D",500);
     lookup_table.insert("M",1000);

     let my_vec: Vec<char> = s.chars().collect();
     let str_len = s.len() as i32;
     let mut i = 0;
     let mut res = 0;
     let mut next_word:String = "dummy".to_string();
     for mut i in 0..str_len{
        let current_word = my_vec[i as usize].to_string();
        if (i+1 < str_len){
            next_word = my_vec[(i+1) as usize].to_string();
        }
        else {
            next_word = "dummy".to_string();
        }


        if (i +1) < str_len &&   lookup_table.get(&current_word as &str).unwrap_or(&0) < lookup_table.get(&next_word as &str).unwrap_or(&0)  {
            res -= lookup_table.get(&current_word as &str).unwrap_or(&0);
        }  
        else {
            res += lookup_table.get(&current_word as &str).unwrap_or(&0);
        }
     }
     return res;
 }

//  if i == (str_len-1){
//     let mut word = my_vec[i as usize].to_string();
//     res += lookup_table.get(&word as &str).unwrap_or(&0);
//     break;
// }
// else {
//     let mut word1 = my_vec[i as usize].to_string();
//     let mut word3 = my_vec[i as usize].to_string();
//     let mut word2 = my_vec[(i+1) as usize].to_string();
//     word1.push_str(&word2);

//     match lookup_table.get(&word1 as &str) {
//         Some(num) => {
//             res += num;
            
//         } ,
//         None => match lookup_table.get(&word3 as &str) {
//             Some(num2) => res += num2,
//             None => res +=0

//         }
//     }

// }