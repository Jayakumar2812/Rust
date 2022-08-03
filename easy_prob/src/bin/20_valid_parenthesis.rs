fn main() {
    println!("{}",is_valid("]".to_string()));
}

struct Stack<T> {
    stack: Vec<T>,
  }
  
  impl<T> Stack<T> {
    fn new() -> Self {
      Stack { stack: Vec::new() }
    }
  
    fn length(&self) -> usize {
      self.stack.len()
    }
  
    fn pop(&mut self) -> Option<T> {
      self.stack.pop()
    }
  
    fn push(&mut self, item: T) {
      self.stack.push(item)
    }
  
    fn is_empty(&self) -> bool {
      self.stack.is_empty()
    }
  
    fn peek(&self) -> Option<&T> {
      self.stack.last()
    }
  }

pub fn is_valid(s: String) -> bool {
    
    let mut stack: Stack<char> = Stack::new();

    for (i,v) in s.chars().enumerate(){
        
        if v == '{' || v == '[' || v == '(' {
            stack.push(v);
        }
        else {
            if v == '}' && *stack.peek().unwrap_or(&'0') == '{' {
                stack.pop();
            }
            else if v == ']' && *stack.peek().unwrap_or(&'0') == '[' {
                stack.pop();
            }
            else if v == ')' && *stack.peek().unwrap_or(&'0') == '(' {
                stack.pop();
            }
            else {
                stack.push(v);
                continue;
            }

        }
    }
    if stack.is_empty() == true {
         true 
    }    
    else {
        false
    }
}



