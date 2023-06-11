use std::{cell::RefCell, rc::Rc};

pub fn binary_search(arr: &[i64], target: i64, mut low: usize, mut high: usize) -> Option<usize> {
    while low < high {
        let mid_idx = (low + (high - low) / 2) as usize;
        let mid_val = arr[mid_idx];
        if mid_val == target {
            return Some(mid_idx);
        } else if mid_val > target {
            high = mid_idx;
        } else {
            low = mid_idx + 1;
        }
    }
    return None;
}

pub fn two_crystal_balls(arr: &[bool]) -> Option<usize> {
    let jump_by = f64::sqrt(arr.len() as f64) as usize;
    let mut idx = jump_by;

    while idx < arr.len() {
        if arr[idx] {
            break;
        }
        idx += jump_by;
    }

    idx -= jump_by;
    for i in idx..arr.len() {
        if arr[i] {
            return Some(i);
        }
    }

    return None;
}

pub fn bubble_sort(arr: &mut [i64]) {
    for i in 0..arr.len() {
        let stop = arr.len() - i;
        for i in 0..stop - 1 {
            if arr[i] > arr[i + 1] {
                let curr_val = arr[i];
                arr[i] = arr[i + 1];
                arr[i + 1] = curr_val;
            }
        }
    }
}

pub fn quicksort(arr: &mut [i64], low: i64, high: i64) {
    if low >= high {
        return;
    }

    let pivotIdx = partition(arr, low, high);
    quicksort(arr, low, pivotIdx - 1);
    quicksort(arr, pivotIdx + 1, high);
}

pub fn partition(arr: &mut [i64], low: i64, high: i64) -> i64 {
    let pivot = arr[high as usize];
    let mut switch = low - 1;
    for i in low..=high {
        if arr[i as usize] <= pivot {
            switch += 1;
            let temp = arr[i as usize];
            arr[i as usize] = arr[switch as usize];
            arr[switch as usize] = temp;
        }
    }

    return switch;
}

pub struct RingBuffer<T> {
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
    buffer: Vec<Rc<RefCell<T>>>,
}

impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        return Self {
            head: 0,
            tail: 0,
            size: 0,
            capacity,
            buffer: Vec::<Rc<RefCell<T>>>::with_capacity(capacity),
        };
    }

    pub fn enque(&mut self, item: T) {
        if self.size == 0 {
            self.buffer.push(Rc::new(RefCell::new(item)));
        } else {
            let mut new_tail = (self.tail + 1) % self.capacity;
            if new_tail == self.head {
                self.resize();
                new_tail = (self.tail + 1) % self.capacity;
            }
            if new_tail > self.head {
                self.buffer.push(Rc::new(RefCell::new(item)));
            } else {
                self.buffer[new_tail] = Rc::new(RefCell::new(item));
            }
            self.tail = new_tail;
        }
        self.size += 1;
    }

    pub fn deque(&mut self) -> Rc<RefCell<T>> {
        if self.size == 0 {
            panic!("Cannot deque from empty buffer");
        }
        let first_item = self.buffer[self.head].clone();
        self.head = (self.head + 1) % self.capacity;
        self.size -= 1;
        return first_item;
    }

    fn resize(&mut self) {
        let new_capacity = self.capacity * 2;
        let mut new_buffer = Vec::<Rc<RefCell<T>>>::with_capacity(new_capacity);
        for i in 0..self.size {
            let idx = (self.head + i) % self.capacity;
            new_buffer.push(self.buffer[idx].clone());
        }
        self.head = 0;
        self.tail = self.size - 1;
        self.capacity = new_capacity;
        self.buffer = new_buffer;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search() {
        // test found
        let arr = [1, 2, 3, 4, 5];
        let target = 3;
        let low = 0;
        let high = arr.len();
        assert_eq!(binary_search(&arr, target, low, high), Some(2));

        let target = 5;
        assert_eq!(binary_search(&arr, target, low, high), Some(4));

        let target = 1;
        assert_eq!(binary_search(&arr, target, low, high), Some(0));

        // test not found
        let arr = [1, 2, 3, 4, 5];
        let target = 6;
        let low = 0;
        let high = arr.len() - 1;
        assert_eq!(binary_search(&arr, target, low, high), None);

        let target = 0;
        assert_eq!(binary_search(&arr, target, low, high), None);

        // test slice
        let arr = [1, 2, 3, 4, 5, 6, 7, 9, 10];
        let target = 5;
        let low = 2;
        let high = 6;
        assert_eq!(binary_search(&arr, target, low, high), Some(4));

        // test empty arr
        let arr = [];
        let target = 5;
        let low = 0;
        let high = arr.len();
        assert_eq!(binary_search(&arr, target, low, high), None);
    }

    #[test]
    fn test_two_crystal_balls() {
        let arr = [
            false, false, false, false, false, false, false, false, false, true,
        ];
        assert_eq!(two_crystal_balls(&arr), Some(9));

        let arr = [
            false, false, false, false, false, false, false, false, false, false,
        ];
        assert_eq!(two_crystal_balls(&arr), None);

        let arr = [
            false, false, false, false, false, true, true, true, true, true, true,
        ];
        assert_eq!(two_crystal_balls(&arr), Some(5));

        let arr = [
            true, true, true, true, true, true, true, true, true, true, true,
        ];
        assert_eq!(two_crystal_balls(&arr), Some(0));

        let arr = [];
        assert_eq!(two_crystal_balls(&arr), None);
    }

    #[test]
    fn test_bubble_sort() {
        // test worst case
        let mut arr = [5, 4, 3, 2, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);

        // test best case
        let mut arr = [1, 2, 3, 4, 5];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);

        // test random
        let mut arr = [100, 5, 1, 6, 278, 53];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 5, 6, 53, 100, 278]);

        // test empty
        let mut arr = [];
        bubble_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_ring_enque() {
        let mut rb = RingBuffer::<i64>::new(2);
        for i in 0..1000 {
            rb.enque(i);
        }
        assert_eq!(rb.size, 1000);
        assert_eq!(rb.capacity, 1024);
        assert_eq!(rb.size, rb.buffer.len());
        assert_eq!(rb.capacity, rb.buffer.capacity());
    }

    #[test]
    #[should_panic(expected = "Cannot deque from empty buffer")]
    fn test_ring_deque() {
        let mut rb = RingBuffer::<i64>::new(2);
        for i in 0..25 {
            rb.enque(i);
        }
        for i in 0..12 {
            let item = rb.deque();
            assert_eq!(*item.borrow(), i);
        }
        for i in 0..12 {
            rb.enque(i);
        }
        let expected_order = [
            12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        for i in expected_order.iter() {
            let item = rb.deque();
            assert_eq!(*item.borrow(), *i);
        }
        for i in 0..100 {
            rb.deque();
        }
    }

    #[test]
    fn test_quicksort() {
        let mut arr = [5, 4, 3, 2, 1];
        quicksort(&mut arr, 0, 4);
        assert_eq!(arr, [1, 2, 3, 4, 5]);

        arr = [1, 2, 3, 4, 5];
        quicksort(&mut arr, 0, 4);
        assert_eq!(arr, [1, 2, 3, 4, 5]);

        arr = [100, 5, 1, 6, 278];
        quicksort(&mut arr, 0, 4);
        assert_eq!(arr, [1, 5, 6, 100, 278]);
    }
}
