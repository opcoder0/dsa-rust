pub struct Queue<T> {
    backend: Vec<T>,
    head: usize,
    tail: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum QueueError {
    QueueFull,
    QueueEmpty,
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

pub struct CircularQueue<T> {
    backend: Vec<T>,
    head: usize,
    tail: usize,
    size: usize,
}

impl<T> CircularQueue<T> {
    pub fn new(size: usize) -> Self {
        Self {
            backend: vec![],
            head: 0,
            tail: 0,
            size,
        }
    }

    pub fn insert(&mut self, element: T) -> Result<(), QueueError> {
        if self.tail == self.size {
            if self.head == self.tail % self.size {
                return Err(QueueError::QueueFull);
            } else {
                self.backend.insert(self.tail, element);
                self.tail += 1;
            }
        } else {
            self.backend.insert(self.tail, element);
            self.tail += 1;
        }
        Ok(())
    }

    pub fn remove(&mut self) -> Result<Option<T>, QueueError> {
        if self.head == self.tail {
            return Err(QueueError::QueueEmpty);
        }
        let v = self.backend.remove(self.head);
        self.head += 1;
        Ok(Some(v))
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

    #[test]
    fn circular_queue_insert_is_ok() {
        let mut cq = CircularQueue::new(10);
        assert_eq!(cq.insert(1).is_ok(), true);
    }

    #[test]
    fn circular_queue_returns_error_when_full() {
        let mut cq = CircularQueue::new(3);
        cq.insert(1).unwrap();
        cq.insert(2).unwrap();
        cq.insert(3).unwrap();
        if let Err(e) = cq.insert(4) {
            assert_eq!(e, QueueError::QueueFull);
        } else {
            assert!(
                false,
                "logical error: insert must have returned a queue full error"
            );
        }
    }

    #[test]
    fn circular_queue_remove_is_ok() {
        let mut cq = CircularQueue::new(3);
        cq.insert(1).unwrap();
        cq.insert(2).unwrap();
        cq.insert(3).unwrap();
        if let Ok(v) = cq.remove() {
            assert_eq!(v, Some(1));
        } else {
            assert!(false, "remove must have been successful");
        }
    }

    #[test]
    fn circular_queue_remove_returns_error_when_queue_is_empty() {
        let mut cq: CircularQueue<i32> = CircularQueue::new(3);
        if let Err(e) = cq.remove() {
            assert_eq!(e, QueueError::QueueEmpty);
        } else {
            assert!(
                false,
                "attempt to remove element from empty queue must return an error"
            );
        }
    }
}
