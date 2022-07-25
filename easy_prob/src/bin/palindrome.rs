fn main() {
     println!("{}",is_palindrome(1));

}
pub fn is_palindrome(x: i32) -> bool {
    if x<0 {
        return false
    }
    let mut y:i32 =0;
    let mut t = x;
    while t>=10  {
            y += t%10;
            y *= 10;
            // println!(" y :{} , t :{}",y,t);
            t = t /10;
        }
    y+=t;
    // println!("{}",y);
    if y == x {return true}
    else {return false}
}