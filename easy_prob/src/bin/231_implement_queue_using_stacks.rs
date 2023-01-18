struct MyQueue {
    queue:Vec<i32>,
    queue_len:i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        MyQueue {queue:vec![],queue_len:0}
    }
    
    fn push(&mut self, x: i32) {
        self.queue_len +=1;
        self.queue.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        if self.queue_len != 0 {
            self.queue_len -=1;
            self.queue.remove(0)
        }
        else {
            -1
        }
    }
    
    fn peek(&self) -> i32 {
        self.queue[0 ]
    }
    
    fn empty(&self) -> bool {
        if self.queue_len == 0 { true} else { false}
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */