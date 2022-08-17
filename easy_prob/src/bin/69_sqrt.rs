use std::result;

fn main() {
    println!("{}",my_sqrt(273999));
}



// pub fn my_sqrt(x: i32) -> i32 {
//     let mut r = x;
//     let mut l = 1;
//     while r>10{
//         r= r/10;
//         l+=1;
//     }

//     return l;
// }

pub fn my_sqrt(x: i32) -> i32 {

    let y = x as i128;
    let mut  half = y/2 ;
    let mut  half_sq = half*half;
    if half_sq ==y {
        return half as i32; 
    }

    while half_sq >= y  {
        println!("1st while {}",half);
        if half_sq == y {
            break;
        }
        half = half/2;
        half_sq = half * half;
    }
    while half_sq<=y {
        println!("2nd while {}",half);
        if half_sq == y {
            break;
        }
        half +=1;
        half_sq = half * half;
        if half_sq > y {
            half -=1;
            break;
        }
    }
    return half as i32;
}

// let mut  half = x/2;
//     let mut  half_sq = half*half;
//     if half_sq ==x {
//         return half; 
//     }

//     while half_sq >= x  {
//         println!("1st while {}",half);
//         if half_sq == x {
//             break;
//         }
//         half = half/2;
//         half_sq = half * half;
//     }
//     while half_sq<=x {
//         println!("2nd while {}",half);
//         if half_sq == x {
//             break;
//         }
//         half +=1;
//         half_sq = half * half;
//         if half_sq > x {
//             half -=1;
//             break;
//         }
//     }

    // let y:i128 = x.into();
    // if y==0 {
    //     return 0;
    // }
    // let mut r= y;
    // let mut l =1;
    // let mut  res = 0;
    // while l<=r {
    //    let mid = l + (r-l)/2;
    //    if mid *mid <= y {
    //     l =mid +1;
    //      res = mid ;
    //    }
    //    else {
    //        r = mid -1
    //    }
    // }
    // let result  = res as i32;
    // return  result;