pub struct Stack<T> {
    backend: Vec<T>,
    top: usize,
    size: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StackError {
    Overflow,
    Underflow,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            backend: vec![],
            top: 0,
            size: 2048,
        }
    }

    pub fn push(&mut self, element: T) -> Result<(), (Option<T>, StackError)> {
        if self.top == self.size - 1 {
            return Err((Some(element), StackError::Overflow));
        }
        self.backend.push(element);
        self.top += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<T, StackError> {
        if self.top == 0 {
            return Err(StackError::Underflow);
        }
        let v = self.backend.pop();
        self.top -= 1;
        Ok(v.unwrap())
    }

    pub fn peek(&self) -> Option<&T> {
        if self.top == 0 {
            return None;
        }
        let v = &self.backend[self.top - 1];
        Some(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_is_ok() {
        let mut stack = Stack::new();
        let r = stack.push(1);
        assert_eq!(r.is_ok(), true);
    }

    #[test]
    fn test_pop_is_ok() {
        let mut stack = Stack::new();
        let r = stack.push(1);
        assert_eq!(r.is_ok(), true);
        let r = stack.pop();
        let v = r.expect("pop error");
        assert_eq!(v, 1);
    }

    #[test]
    fn test_peek_is_ok() {
        let mut stack = Stack::new();
        stack.push(1).unwrap();
        stack.push(2).unwrap();
        let r = stack.peek();
        assert_eq!(r.expect("peek error"), &2);
    }

    #[test]
    fn test_peek_on_empty_stack_is_none() {
        let stack: Stack<i32> = Stack::new();
        assert_eq!(stack.peek(), None);
    }

    #[test]
    fn test_pop_empty_stack_results_underflow() {
        let mut stack: Stack<i32> = Stack::new();
        let r = stack.pop();
        assert_eq!(r.expect_err("expecting underflow"), StackError::Underflow);
    }

    #[test]
    fn test_push_over_size_results_in_overflow() {
        let mut stack: Stack<i32> = Stack::new();
        for i in 0..=2048 {
            let r = stack.push(i);
            if i >= 2047 {
                if let Err(v) = r {
                    assert_eq!(v.0, Some(i));
                    assert_eq!(v.1, StackError::Overflow);
                } else {
                    assert!(false, "expecting error");
                }
            } else {
                if let Ok(_) = r {
                    assert!(true, "all ok");
                } else {
                    assert!(false, "received unexpected error");
                }
            }
        }
    }
}
