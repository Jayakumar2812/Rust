struct MyStack {
    stack:Vec<i32>,
    stack_len:i32
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    fn new() -> Self {
        MyStack { stack: Vec::new(), stack_len: 0}
    }
    
    fn push(&mut self, x: i32) {
        self.stack_len +=1;
        self.stack.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        if self.stack_len != 0 {
            self.stack_len -=1;
            self.stack.pop().unwrap()
        }
        else {
            return -1
        }
    }
    
    fn top(&self) -> i32 {
        self.stack[ (self.stack_len -1) as usize]
    }
    
    fn empty(&self) -> bool {
        if  self.stack_len == 0 {
        return true
       } 
       else {
        return false
       }
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */
