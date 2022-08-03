use std::{str::Chars, fmt::Result, result};
fn main() {
    println!("{:?}",longest_common_prefix(["flower".to_string(),"flower".to_string(),"flower".to_string()].to_vec()));
}


pub fn longest_common_prefix(strs: Vec<String>) -> String {
  
  let mut prefix_count:i32 = strs[0].len() as i32;
  for i in 0..strs.len(){
    let compare_to = strs[i].chars();
    let compare = strs[0].chars().clone();
    let current_count = (compare_to.zip(compare).take_while(|(a,b)| a==b).count() as i32);
    // println!("{}",current_count);
    if  prefix_count > current_count {
      prefix_count = current_count;
    } 

   }
  // println!("{}",prefix_count);
  let result:String = strs[0].chars().collect();
  let result_str:&str = result.as_str();
  // println!("{:?}",result_str);
  String::from(&result_str[..prefix_count as usize])
}


// let mut return_word: Vec<Chars> = Vec::new();
//         let mut word_index = 0;
//         return_word.push(strs[0].chars());
//         for (i,v) in strs.iter().enumerate() {
//           let mut index: Vec<Chars> = Vec::new(); // The compiler knows the type
//           index.push(strs[i].chars());
//           let mut current_word_index = 0;
//           let mut i =0;
//           let length = strs.len();
//           for i in 0..length {

//             // println!("{:?}",)
//             // if return_word[i] == index[i]{
//             //   current_word_index +=1;
//             //   }
//             // else  {
//             //   return_word = return_word[..current_word_index].to_vec();
//             //   break
//             // }

//           }
            

//       }
//     let banana: String = String::from("banana");
//     let bandana:String = String::from("dana");
//     let mid  = bandana.len()/2;
//     let left = &bandana[2..mid];
//     let right = &bandana[&bandana.len() - mid..];
//     let left_chars = left.chars();
//     let right_chars = right.chars().rev();
//     println!("{:?}",left);
//     // println!( "{:?}",banana.min(bandana));
//     let mut res = left_chars.zip(right_chars).filter(|(a,b)| a == b).count(); 
//     println!("{}",res);
//     let hike: String = String::from("banana");
//     hike