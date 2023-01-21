impl Solution {
    
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false
        }
        let mut  arr_1:Vec<char> = s.chars().collect();
        let mut  arr_2:Vec<char> = t.chars().collect();
        arr_1.sort();
        arr_2.sort();
        for i in 0..s.len() {
            if arr_1 [i as usize] != arr_2[i] {
                return false
            }
        }       


        return true
    }
    
}
