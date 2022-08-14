fn main() {
    println!("{}",add_binary("1010".to_string(),"1011".to_string()));
}
// 1001
//  101
// 1110

pub fn add_binary(a: String, b: String) -> String {

    let vec1:Vec<char> = a.chars().collect();
    let vec2:Vec<char> = b.chars().collect();

    let mut v1 = vec1.len();
    let mut v2 = vec2.len();
    let mut iter_len = 0;
    let mut final_vec:Vec<String> =Vec::new();
    let mut c = '0';
    if v1 > v2{
        iter_len = v2;
        let a = v1 -v2;
        for i in (0..iter_len).rev() {
            if i <= (v1 -1) && i <=(v2 -1) {
                let ret_str = add_override(vec1[i+a], vec2[i], c);
                if c== '1'{
                    c='0';
                }
                println!("vec1[i] --> {} , vec2[i] --> {}",vec1[i],vec2[i]);
                println!("retstr --> {:?}",&ret_str);
                if ret_str.len() == 1 {
                    final_vec.insert(0, ret_str);
                }
                else {
                    let temp_vec:Vec<char> = ret_str.chars().collect();
                    final_vec.insert(0, temp_vec[1].to_string());
                    c = temp_vec[0];
                }
            } 
    
            println!("final_vec -->{:?}",final_vec);
        }
        for i in (0..a).rev(){
            let ret_str = add_override(vec1[i],'0', c);
            if c== '1'{
                c='0';
            }
            println!("vec1[i] --> {} , vec2[i] --> {}",vec1[i],vec2[i]);
                println!("retstr --> {:?}",&ret_str);
                if ret_str.len() == 1 {
                    final_vec.insert(0, ret_str);
                }
                else {
                    let temp_vec:Vec<char> = ret_str.chars().collect();
                    final_vec.insert(0, temp_vec[1].to_string());
                    c = temp_vec[0];
                }
        }
        if c== '1' {
            final_vec.insert(0, '1'.to_string());
        }
    }
    else{
        println!("else");
        iter_len = v1;
        let a = v2 -v1;
        for i in (0..iter_len).rev() {
            if i <= (v1 -1) && i <=(v2 -1) {
                let ret_str = add_override(vec1[i], vec2[i+a], c);
                if c== '1'{
                    c='0';
                }
                println!("vec1[i] --> {} , vec2[i+a] --> {} , i--> {} , i+a--> {} , c--> {}",vec1[i],vec2[i+a],i,i+a,c);
                println!("retstr --> {:?}",&ret_str);
                if ret_str.len() == 1 {
                    final_vec.insert(0, ret_str);
                }
                else {
                    let temp_vec:Vec<char> = ret_str.chars().collect();
                    final_vec.insert(0, temp_vec[1].to_string());
                    c = temp_vec[0];
                }
            } 
    
            println!("final_vec -->{:?}",final_vec);
        }
        if v2 > v1 {
            for i in (0..a).rev(){
                let ret_str = add_override('0',vec2[i], c);
                if c== '1'{
                    c='0';
                }
                println!("vec1[i] --> {} , vec2[i] --> {} , i--> {}",vec1[i],vec2[i],i);
                    println!("retstr --> {:?}",&ret_str);
                    if ret_str.len() == 1 {
                        final_vec.insert(0, ret_str);
                    }
                    else {
                        let temp_vec:Vec<char> = ret_str.chars().collect();
                        final_vec.insert(0, temp_vec[1].to_string());
                        c = temp_vec[0];
                    }
            }
        }
        
    if c== '1' {
        final_vec.insert(0, '1'.to_string());
    }

    }
    
    
    let s: String = final_vec.into_iter().collect();

    s
}

pub fn add_override(a: char,b: char,c:char) -> String {
    if a == '1' && b== '0' && c == '0'{
        return '1'.to_string(); 
    }
    else if a == '0' && b== '0' && c == '0'{
        return '0'.to_string(); 
    }
    else if a == '0' && b== '1' && c == '0'{
        return '1'.to_string(); 
    }
    else if a == '1' && b== '1' && c == '0'{
        return "10".to_string();
    }
    else if a == '1' && b== '0' && c == '1'{
        return "10".to_string(); 
    }
    else if a == '0' && b== '0' && c == '1'{
        return '1'.to_string(); 
    }
    else if a == '0' && b== '1' && c == '1'{
        return "10".to_string(); 
    }
    else {
        return "11".to_string();
    }
}