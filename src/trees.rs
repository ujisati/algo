use crate::linked_lists::Queue;
use std::{cell::RefCell, rc::Rc};

pub struct BinaryNode<T> {
    value: T,
    left: Option<Rc<RefCell<BinaryNode<T>>>>,
    right: Option<Rc<RefCell<BinaryNode<T>>>>,
}

impl<T: Clone> BinaryNode<T> {
    pub fn new(value: T) -> BinaryNode<T> {
        BinaryNode {
            value,
            left: None,
            right: None,
        }
    }

    pub fn get_value(&self) -> T {
        return self.value.clone();
    }
}

// IN ORDER TRAVERSAL = IN ORDER VALUES. STRONGLY SORTED
impl BinarySearchTree for BinaryNode<i64> {
    fn insert(&mut self, value: i64) {
        let current_value = self.get_value();
        if value <= current_value {
            if self.left.is_some() {
                self.left.as_ref().unwrap().borrow_mut().insert(value);
            }
            self.left = Some(Rc::new(RefCell::new(BinaryNode::new(value))));
        } else {
            if self.right.is_some() {
                self.right.as_ref().unwrap().borrow_mut().insert(value);
            }
            self.right = Some(Rc::new(RefCell::new(BinaryNode::new(value))));
        }
    }

    fn search(&self, value: i64) -> bool {
        // is value == node.value?
        let node_value = self.get_value();
        let node_left = self.left.clone();
        let node_right = self.right.clone();
        if node_value == value {
            return true;
        } else if value <= node_value && node_left.is_some() {
            return node_left.unwrap().borrow().search(value);
        } else if value > node_value && node_right.is_some() {
            return node_right.unwrap().borrow().search(value);
        }
        false
    }

    fn delete(&mut self, value: i64) {
        todo!()
    }
}

trait BinarySearchTree {
    fn insert(&mut self, value: i64);
    fn search(&self, value: i64) -> bool;
    fn delete(&mut self, value: i64);
}

pub fn breadth_first_search(queue: &mut Queue<Rc<RefCell<BinaryNode<i64>>>>, target: i64) -> bool {
    if let Some(node) = queue.peek() {
        if node.borrow().get_value().borrow().get_value() == target {
            return true;
        }
        queue.deque();
        if let Some(left) = node.borrow().get_value().borrow().left.as_ref() {
            queue.enque(left.clone());
        }
        if let Some(right) = node.borrow().get_value().borrow().right.as_ref() {
            queue.enque(right.clone());
        }
        return breadth_first_search(queue, target);
    } else {
        return false;
    }
}

pub fn pre_order_traverse(
    node: Option<Rc<RefCell<BinaryNode<i64>>>>,
    path: &mut Vec<i64>,
) -> &mut Vec<i64> {
    if node.is_none() {
        return path;
    }

    path.push(node.as_ref().unwrap().borrow().value);
    pre_order_traverse(node.as_ref().unwrap().borrow().left.clone(), path);
    pre_order_traverse(node.unwrap().borrow().right.clone(), path);

    return path;
}

pub fn post_order_traverse(
    node: Option<Rc<RefCell<BinaryNode<i64>>>>,
    path: &mut Vec<i64>,
) -> &mut Vec<i64> {
    if node.is_none() {
        return path;
    }

    post_order_traverse(node.as_ref().unwrap().borrow().left.clone(), path);
    post_order_traverse(node.as_ref().unwrap().borrow().right.clone(), path);
    path.push(node.as_ref().unwrap().borrow().value);

    return path;
}

pub fn in_order_traverse(
    node: Option<Rc<RefCell<BinaryNode<i64>>>>,
    path: &mut Vec<i64>,
) -> &mut Vec<i64> {
    if node.is_none() {
        return path;
    }

    in_order_traverse(node.as_ref().unwrap().borrow().left.clone(), path);
    path.push(node.as_ref().unwrap().borrow().value);
    in_order_traverse(node.as_ref().unwrap().borrow().right.clone(), path);

    return path;
}

// Red-black tree, AVL tree for balancing

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traverse() {
        let mut root = BinaryNode::new(1);
        let mut left = BinaryNode::new(2);
        let mut right = BinaryNode::new(3);
        let left_left = BinaryNode::new(4);
        let left_right = BinaryNode::new(5);
        let right_left = BinaryNode::new(6);
        let right_right = BinaryNode::new(7);

        left.left = Some(Rc::new(RefCell::new(left_left)));
        left.right = Some(Rc::new(RefCell::new(left_right)));
        right.left = Some(Rc::new(RefCell::new(right_left)));
        right.right = Some(Rc::new(RefCell::new(right_right)));
        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        let mut path = Vec::<i64>::new();
        let root = Some(Rc::new(RefCell::new(root)));
        pre_order_traverse(root.clone(), &mut path);
        assert_eq!(path, vec![1, 2, 4, 5, 3, 6, 7]);

        path.clear();
        post_order_traverse(root.clone(), &mut path);
        assert_eq!(path, vec![4, 5, 2, 6, 7, 3, 1]);

        path.clear();
        in_order_traverse(root, &mut path);
        assert_eq!(path, vec![4, 2, 5, 1, 6, 3, 7]);
    }

    #[test]
    fn test_bfs() {
        let mut queue = Queue::<Rc<RefCell<BinaryNode<i64>>>>::new();
        let mut root = BinaryNode::new(1);
        let mut left = BinaryNode::new(2);
        let mut right = BinaryNode::new(3);
        let mut left_left = BinaryNode::new(4);
        let mut left_right = BinaryNode::new(5);
        let mut right_left = BinaryNode::new(6);
        let mut right_right = BinaryNode::new(7);
        let left_left_left = BinaryNode::new(8);
        let left_left_right = BinaryNode::new(9);
        let left_right_left = BinaryNode::new(10);
        let left_right_right = BinaryNode::new(11);
        let right_left_left = BinaryNode::new(12);
        let right_left_right = BinaryNode::new(13);
        let right_right_left = BinaryNode::new(14);
        let right_right_right = BinaryNode::new(15);

        left_left.left = Some(Rc::new(RefCell::new(left_left_left)));
        left_left.right = Some(Rc::new(RefCell::new(left_left_right)));
        left_right.left = Some(Rc::new(RefCell::new(left_right_left)));
        left_right.right = Some(Rc::new(RefCell::new(left_right_right)));
        right_left.left = Some(Rc::new(RefCell::new(right_left_left)));
        right_left.right = Some(Rc::new(RefCell::new(right_left_right)));
        right_right.left = Some(Rc::new(RefCell::new(right_right_left)));
        right_right.right = Some(Rc::new(RefCell::new(right_right_right)));
        left.left = Some(Rc::new(RefCell::new(left_left)));
        left.right = Some(Rc::new(RefCell::new(left_right)));
        right.left = Some(Rc::new(RefCell::new(right_left)));
        right.right = Some(Rc::new(RefCell::new(right_right)));
        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        let root = Rc::new(RefCell::new(root));
        queue.enque(root.clone());
        let found = breadth_first_search(&mut queue, 10);
        assert_eq!(found, true);

        let mut queue = Queue::<Rc<RefCell<BinaryNode<i64>>>>::new();
        let found = breadth_first_search(&mut queue, -1);
        assert_eq!(found, false);
    }

    #[test]
    fn test_bst_search() {
        let mut root = BinaryNode::new(20);
        let mut left = BinaryNode::new(10);
        let mut right = BinaryNode::new(30);
        let mut left_left = BinaryNode::new(5);
        let mut left_right = BinaryNode::new(15);
        let mut right_left = BinaryNode::new(25);
        let mut right_right = BinaryNode::new(35);
        let left_left_left = BinaryNode::new(3);
        let left_left_right = BinaryNode::new(7);
        let left_right_left = BinaryNode::new(13);
        let left_right_right = BinaryNode::new(17);
        let right_left_left = BinaryNode::new(23);
        let right_left_right = BinaryNode::new(27);
        let right_right_left = BinaryNode::new(33);
        let right_right_right = BinaryNode::new(37);

        left_left.left = Some(Rc::new(RefCell::new(left_left_left)));
        left_left.right = Some(Rc::new(RefCell::new(left_left_right)));
        left_right.left = Some(Rc::new(RefCell::new(left_right_left)));
        left_right.right = Some(Rc::new(RefCell::new(left_right_right)));
        right_left.left = Some(Rc::new(RefCell::new(right_left_left)));
        right_left.right = Some(Rc::new(RefCell::new(right_left_right)));
        right_right.left = Some(Rc::new(RefCell::new(right_right_left)));
        right_right.right = Some(Rc::new(RefCell::new(right_right_right)));
        left.left = Some(Rc::new(RefCell::new(left_left)));
        left.right = Some(Rc::new(RefCell::new(left_right)));
        right.left = Some(Rc::new(RefCell::new(right_left)));
        right.right = Some(Rc::new(RefCell::new(right_right)));
        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        assert_eq!(root.search(37), true);
        assert_eq!(root.search(38), false);
        assert_eq!(root.search(3), true);
        assert_eq!(root.search(4), false);
        assert_eq!(root.search(20), true);
    }

    #[test]
    fn test_bst_insert() {
        let mut root = BinaryNode::new(20);
        let mut left = BinaryNode::new(10);
        let mut right = BinaryNode::new(30);
        let mut left_left = BinaryNode::new(5);
        let mut left_right = BinaryNode::new(15);
        let mut right_left = BinaryNode::new(25);
        let mut right_right = BinaryNode::new(35);
        let left_left_left = BinaryNode::new(3);
        let left_left_right = BinaryNode::new(7);
        let left_right_left = BinaryNode::new(13);
        let left_right_right = BinaryNode::new(17);
        let right_left_left = BinaryNode::new(23);
        let right_left_right = BinaryNode::new(27);
        let right_right_left = BinaryNode::new(33);
        let right_right_right = BinaryNode::new(37);

        left_left.left = Some(Rc::new(RefCell::new(left_left_left)));
        left_left.right = Some(Rc::new(RefCell::new(left_left_right)));
        left_right.left = Some(Rc::new(RefCell::new(left_right_left)));
        left_right.right = Some(Rc::new(RefCell::new(left_right_right)));
        right_left.left = Some(Rc::new(RefCell::new(right_left_left)));
        right_left.right = Some(Rc::new(RefCell::new(right_left_right)));
        right_right.left = Some(Rc::new(RefCell::new(right_right_left)));
        right_right.right = Some(Rc::new(RefCell::new(right_right_right)));
        left.left = Some(Rc::new(RefCell::new(left_left)));
        left.right = Some(Rc::new(RefCell::new(left_right)));
        right.left = Some(Rc::new(RefCell::new(right_left)));
        right.right = Some(Rc::new(RefCell::new(right_right)));
        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        root.insert(38);
        assert_eq!(root.search(38), true);
    }
}
