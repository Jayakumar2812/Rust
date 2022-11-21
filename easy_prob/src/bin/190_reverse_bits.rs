impl Solution {
pub fn reverse_bits(x: u32) -> u32 {
    let  y =format!("{:b}",x);
    let mut res:u32 =0;
    let mut bit_vec:Vec<_> =  y.chars().collect();

    for  _i in 0..(32- y.len()) {
        bit_vec.insert(0,'0');
    }
    for  (i,v) in bit_vec.iter().enumerate() {
        if v == &'1'{
            res += 2u32.pow(i as u32) * 1 ;
        }
    }

    return res
}
}
