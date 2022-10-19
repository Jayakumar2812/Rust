fn main () {
    println!("{:?}",generate(3)); 
 }
 
 
 pub fn generate(num_rows: i32) -> Vec<i32> {
     if num_rows == 0 {
         let mut res:Vec<Vec<i32>> = vec![vec![1]]; 
         return res[0].clone()
     }
     if num_rows == 1 {
         let mut res:Vec<Vec<i32>> = vec![vec![1],vec![1,1]]; 
         return res[1].clone()
 
     }
     if num_rows == 2 {
        let mut res:Vec<Vec<i32>> = vec![vec![1],vec![1,1],vec![1,2,1]]; 
        return res[2].clone()

    }

     let mut res:Vec<Vec<i32>> = vec![vec![1],vec![1,1],vec![1,2,1]]; 
     let mut prev:Vec<i32> = vec![1,2,1];
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
     return res[(res.len()-1 ) as usize].clone()
 
     }



    // Better solution from comments using formula
    // pub fn get_row(row_index: i32) -> Vec<i32> {
    //     let row_index = row_index as usize;
    //     let mut res = vec![];
    //     res.push(1);
        
    //     for i in 1..row_index + 1 {
    //         let tmp = res[i - 1] as usize * (row_index + 1 - i) / i;
    //         res.push(tmp as i32);
    //     }

    //     res
    // }