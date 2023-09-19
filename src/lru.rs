use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use uuid::Uuid;

// type IntMutNode<T> = Rc<RefCell<Node<T>>>;
//
// struct Node<T> {
//     value: T,
//     next: Option<IntMutNode<T>>,
//     prev: Option<IntMutNode<T>>,
//     uuid: Uuid,
// }
//
// struct LRU<K, V>
// where
//     K: Hash,
// {
//     head: Option<IntMutNode<V>>,
//     tail: Option<IntMutNode<V>>,
//     lookup: HashMap<K, IntMutNode<V>>,
//     reverse_lookup: HashMap<Uuid, K>,
//     capacity: i64,
//     length: i64,
// }
//
// impl<K, V> LRU<K, V>
// where
//     K: Hash,
//     K: Eq,
//     V: Clone
// {
//     fn new(capacity: i64) -> Self {
//         Self {
//             head: None,
//             tail: None,
//             lookup: HashMap::new(),
//             reverse_lookup: HashMap::new(),
//             capacity,
//             length: 0,
//         }
//     }
//
//     fn get(&mut self, key: &K) -> Option<V> {
//         if self.lookup.contains_key(key) {
//             let node = self.lookup[key].clone();
//             self.detach(&node);
//             self.prepend(&node);
//             return Some(node.borrow().value.clone())
//         } else {
//             None
//         }
//     }
//
//     fn detach(&mut self, node: &IntMutNode<V>) {}
//
//     fn prepend(&mut self, node: &IntMutNode<V>) {}
// }
//
struct Node {
    value: i64,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Rc<RefCell<Node>>>,
    uuid: Uuid,
}

struct LRU {
    pub head: Option<Rc<RefCell<Node>>>,
    pub tail: Option<Rc<RefCell<Node>>>,
    lookup: HashMap<i64, Rc<RefCell<Node>>>,
    reverse_lookup: HashMap<String, i64>,
    capacity: i64,
    length: i64,
}

impl LRU {
    fn new(capacity: i64) -> Self {
        Self {
            head: None,
            tail: None,
            lookup: HashMap::new(),
            reverse_lookup: HashMap::new(),
            capacity,
            length: 0,
        }
    }

    fn get(&mut self, key: i64) -> Option<i64> {
        if self.lookup.contains_key(&key) {
            let node = self.lookup[&key].clone();
            self.detach(node.clone());
            self.prepend(node.clone());
            return Some(node.borrow().value);
        } else {
            None
        }
    }

    fn put(&mut self, key: i64, value: i64) {
        if self.lookup.contains_key(&key) {
            let node = self.lookup[&key].clone();
            self.detach(node.clone());
            self.prepend(node);
        } else {
            let uuid = Uuid::new_v4();
            let node = Rc::new(RefCell::new(Node {
                value,
                prev: None,
                next: None,
                uuid: uuid.clone(),
            }));
            self.lookup.insert(key, node.clone());
            self.reverse_lookup.insert(uuid.to_string(), key);
            self.prepend(node);
        }
        self.length += 1;
        self.trim_cache();
    }

    fn detach(&mut self, node: Rc<RefCell<Node>>) {
        if Rc::ptr_eq(&node, &self.head.clone().unwrap()) {
            self.head = node.borrow().next.clone();
        }
        if Rc::ptr_eq(&node, &self.tail.clone().unwrap()) {
            self.tail = node.borrow().prev.clone();
        }
        let mut node = node.borrow_mut();
        if let Some(prev) = node.prev.clone() {
            prev.borrow_mut().next = node.next.clone();
        }
        if let Some(next) = node.next.clone() {
            next.borrow_mut().prev = node.prev.clone();
        }
        node.next = None;
        node.prev = None;
    }

    fn prepend(&mut self, node: Rc<RefCell<Node>>) {
        if let Some(head) = self.head.clone() {
            head.borrow_mut().prev = Some(node.clone());
            node.borrow_mut().next = Some(head);
            node.borrow_mut().prev = None;
            self.head = Some(node);
        } else {
            self.head = Some(node.clone());
            self.tail = Some(node.clone());
        }
    }

    fn trim_cache(&mut self) {
        if self.length > self.capacity {
            let node = self.tail.clone().unwrap();
            self.tail = node.borrow().prev.clone();
            self.detach(node.clone());
            let uuid = &node.clone().borrow().uuid.to_string();
            let key = self.reverse_lookup[uuid];
            self.lookup.remove(&key);
            self.reverse_lookup.remove(uuid);
            self.length -= 1;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_put() {
        let mut cache = LRU::new(10);
        cache.put(1, 1);
        cache.put(2, 2);
        cache.put(3, 3);
        cache.put(4, 4);
        assert_eq!(cache.head.unwrap().borrow().value, 4);
    }

    #[test]
    fn test_get() {
        let mut cache = LRU::new(10);
        cache.put(1, 1);
        cache.put(2, 2);
        cache.put(3, 3);
        cache.put(4, 4);
        assert_eq!(cache.get(1).unwrap(), 1);
        assert_eq!(cache.head.unwrap().borrow().value, 1);
    }

    #[test]
    fn test_overflow() {
        let mut cache = LRU::new(3);
        cache.put(1, 1);
        cache.put(2, 2);
        cache.put(3, 3);
        cache.put(4, 4);
        cache.put(5, 5);
        cache.put(6, 6);
        cache.put(7, 7);
        assert_eq!(cache.tail.unwrap().borrow().value, 5);
    }

    #[test]
    fn test_order() {
        let mut cache = LRU::new(5);
        cache.put(1, 1);
        cache.put(2, 2);
        cache.put(3, 3);
        cache.put(4, 4);
        cache.put(5, 5);
        cache.get(3);
        cache.get(5);
        cache.get(2);
        let order = [2, 5, 3, 4, 1];
        let mut head = cache.head.clone().unwrap();
        for key in order {
            assert_eq!(head.borrow().value, key);
            if key != 1 {
                head = head.clone().borrow().next.clone().unwrap();
            }
        }
    }
}
