use std::{cell::RefCell, rc::Rc};

pub struct Queue<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: usize,
}

pub struct Stack<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    size: usize,
}

pub struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone> Node<T> {
    pub fn get_value(&self) -> T {
        return self.value.clone();
    }
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        return Self {
            head: None,
            tail: None,
            size: 0,
        };
    }

    pub fn enque(&mut self, item: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value: item,
            next: None,
            prev: None,
        }));
        if self.size == 0 {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node.clone());
        } else {
            let old_tail = self.tail.clone().unwrap();
            old_tail.borrow_mut().next = Some(new_node.clone());
            new_node.borrow_mut().prev = Some(old_tail.clone());
            self.tail = Some(new_node.clone());
        }
        self.size += 1;
    }

    pub fn deque(&mut self) -> Rc<RefCell<Node<T>>> {
        if self.size == 0 {
            panic!("Cannot deque from empty queue");
        } else {
            let old_head = self.head.clone().unwrap();
            self.size -= 1;
            if self.size == 0 {
                self.head = None;
                self.tail = None;
            } else {
                let new_head = old_head.borrow().next.clone();
                self.head = new_head;
            }
            return old_head;
        }
    }

    pub fn peek(&self) -> Option<Rc<RefCell<Node<T>>>> {
        return self.head.clone();
    }

    pub fn size(&self) -> usize {
        return self.size;
    }
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        return Self {
            head: None,
            size: 0,
        };
    }

    pub fn push(&mut self, item: T) {
        if self.size == 0 {
            self.head = Some(Rc::new(RefCell::new(Node {
                value: item,
                next: None,
                prev: None,
            })));
        } else {
            let old_head = self.head.clone().unwrap();
            let new_head = Rc::new(RefCell::new(Node {
                value: item,
                prev: Some(old_head.clone()),
                next: None,
            }));
            self.head = Some(new_head.clone());
        }
        self.size += 1;
    }

    pub fn pop(&mut self, item: T) -> Option<Rc<RefCell<Node<T>>>> {
        if self.size == 0 {
            return None;
        } else {
            self.size -= 1;
            let old_head = self.head.clone().unwrap();
            if self.size == 0 {
                self.head = None;
                return Some(old_head);
            } else {
                let new_head = old_head.borrow().prev.clone();
                self.head = new_head;
                return Some(old_head);
            }
        }
    }

    pub fn peek(&self) -> Option<Rc<RefCell<Node<T>>>> {
        return self.head.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enque() {
        let mut queue = Queue::new();
        queue.enque(1);
        queue.enque(2);
        queue.enque(3);
        queue.enque(4);
        queue.enque(5);
        assert_eq!(queue.size, 5);
        assert_eq!(queue.head.unwrap().borrow().value, 1);
        assert_eq!(queue.tail.unwrap().borrow().value, 5);
    }

    #[test]
    #[should_panic(expected = "Cannot deque from empty queue")]
    fn test_deque() {
        let mut queue = Queue::new();
        queue.enque(1);
        queue.enque(2);
        queue.enque(3);
        queue.enque(4);
        queue.enque(5);
        let node = queue.deque();
        assert_eq!(queue.size, 4);
        assert_eq!(node.borrow().value, (1));
        queue.deque();
        queue.deque();
        let node = queue.deque();
        assert_eq!(queue.size, 1);
        assert_eq!(node.borrow().value, (4));
        assert_eq!(queue.head.clone().unwrap().borrow().value, (5));
        assert_eq!(queue.tail.clone().unwrap().borrow().value, (5));

        queue.deque();

        assert_eq!(queue.size, 0);
        assert_eq!(queue.head.is_none(), true);
        queue.deque();
    }

    #[test]
    fn test_queue_peek() {
        let mut queue = Queue::new();
        queue.enque(1);
        queue.enque(2);
        assert_eq!(queue.peek().clone().unwrap().borrow().value, (1));
    }

    #[test]
    fn test_push() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.size, 3);
        assert_eq!(stack.head.clone().unwrap().borrow().value, (3));
    }

    #[test]
    fn test_pop() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        let node = stack.pop(3);
        assert_eq!(stack.size, 2);
        assert_eq!(node.unwrap().borrow().value, (3));
        assert_eq!(stack.head.clone().unwrap().borrow().value, (2));
    }

    #[test]
    fn test_stack_peek() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.peek().unwrap().borrow().value, (2));
    }
}
