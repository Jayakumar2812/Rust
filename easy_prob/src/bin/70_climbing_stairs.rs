use std::collections::HashMap;

fn main() {
    println!("{}",climb_stairs(40));
}


pub fn climb_stairs(n: i32) -> i32 {
    let mut recof:HashMap<i32,i32> = HashMap::new();
    recof.insert(1,1);
    recof.insert(2,2);
    if n== 1{
        return 1;
    }
    if n== 2{
        return 2;
    }
    let mut res  =0; 
    //  recursion(n,& mut recof)
    for i in 3..=n{
        let num_2 =  recof.get(&(i-2));
        let target_2;
           match num_2{
            Some(v) => target_2 = *v,
            None => target_2 =0,
        
        };
        let num_1 =  recof.get(&(i-1));
        let target_1;
           match num_1{
            Some(v) => target_1 = *v,
            None => target_1 =0,
        
        };
        recof.insert(i, target_1+target_2);
        if i == n {
            res = target_1+target_2;
        }
    }
    res
}

pub fn recursion(n:i32,recof:& mut HashMap<i32,i32>)-> i32 {
    if n == 0 {
        return 1
    }
    else if n == -1 {
        return 0 
    }
    else {
       let num1 =  recof.get(&(n-1));
       let mut  num1_af;
       match num1{
        Some(v) => num1_af =*v,

        None =>  num1_af = 0,
    
    };
    let num2 =  recof.get(&(n-2));
    let mut  num2_af;
       match num2{
        Some(v) => num2_af = *v,

        None => num2_af =0,
    
    };
    if num1_af !=0 && num2_af !=0 {
        return num1_af+num2_af;
    }
    else if num1_af !=0 && num2_af ==0 {
        num2_af = recursion(n-2, recof);
        recof.insert(n-2,num2_af);
        return num1_af + num2_af
    }
    else if num1_af ==0 && num2_af !=0 {
        num1_af = recursion(n-1, recof);
        recof.insert(n-1,num1_af);
        return num1_af +num2_af
    }
    else{
        num2_af = recursion(n-2, recof);
        recof.insert(n-2,num2_af);
        num1_af = recursion(n-1, recof);
        recof.insert(n-1,num1_af);
        return recursion(n-1,recof)+recursion(n-2,recof);
    }
    }

}

