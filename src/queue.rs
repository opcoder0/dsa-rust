pub struct Queue<T> {
    backend: Vec<T>,
    head: usize,
    tail: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum QueueError {
    Overflow,
    Underflow,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            backend: vec![],
            head: 0,
            tail: 0,
        }
    }

    pub fn insert(&mut self, element: T) {
        self.backend.insert(self.tail, element);
        self.tail += 1;
    }

    pub fn remove(&mut self) -> Option<T> {
        if self.head == self.tail {
            return None;
        }
        let v = self.backend.remove(self.head);
        self.head += 1;
        Some(v)
    }

    pub fn peek(&self) -> Option<&T> {
        if self.head == self.tail {
            return None;
        }
        let v = &self.backend[self.head];
        Some(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_insert_is_ok() {
        let mut q = Queue::new();
        q.insert(1);
        assert_eq!(q.backend[q.head], 1);
    }

    #[test]
    fn queue_peek_is_ok() {
        let mut q = Queue::new();
        q.insert(1);
        q.insert(2);
        assert_eq!(q.peek(), Some(&1));
    }

    #[test]
    fn queue_remove_is_ok() {
        let mut q = Queue::new();
        q.insert(1);
        q.insert(2);
        let v = q.remove();
        assert_eq!(v, Some(1));
    }

    #[test]
    fn queue_remove_from_empty_queue_returns_none() {
        let mut q: Queue<i32> = Queue::new();
        let v = q.remove();
        assert_eq!(v, None);
    }

    #[test]
    fn queue_peek_on_empty_queue_returns_none() {
        let q: Queue<i32> = Queue::new();
        let v = q.peek();
        assert_eq!(v, None);
    }
}
