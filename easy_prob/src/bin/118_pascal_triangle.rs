fn main () {
   println!("{:?}",generate(7)); 
}


pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    if num_rows == 1 {
        let mut res:Vec<Vec<i32>> = vec![vec![1]]; 
        return res
    }
    if num_rows ==2 {
        let mut res:Vec<Vec<i32>> = vec![vec![1],vec![1,1]]; 
        return res

    }
    let mut res:Vec<Vec<i32>> = vec![vec![1],vec![1,1]]; 
    let mut prev:Vec<i32> = vec![1,1];
    for  i in 3..=num_rows{
        let mut temp:Vec<i32> = Vec::with_capacity(prev.len() + 2 as usize); 
        for k in 0..(prev.len()+1) {
            temp.insert(k,1);
        }
        for j in 1..(prev.len()){
            temp[j]= prev[j-1] + prev[j];  



            }
        res.push(temp.clone());
        prev = temp;
    }
    return res

    }