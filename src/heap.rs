pub struct MinHeap<T> {
    pub length: usize,
    pub data: Vec<T>,
}

impl<T> MinHeap<T>
where
    T: Copy + PartialOrd,
{
    pub fn new(&mut self) {
        self.length = 0;
        self.data = Vec::new();
    }

    pub fn insert(&mut self, value: T) {
        self.data.insert(self.length, value);
        self.heapify_up(self.length);
        self.length += 1;
    }

    pub fn delete(&mut self) -> T {
        let val = self.data[0];

        if self.length == 1 {
            self.data = Vec::new();
            self.length = 0;
            return val;
        }

        self.data[0] = self.data[self.length - 1];
        self.length -= 1;
        self.heapify_down(0);
        return val;
    }

    fn heapify_up(&mut self, idx: usize) {
        if idx == 0 {
            return;
        }
        let parent_idx = self.parent(idx);
        let parent_val = self.data[parent_idx];
        let curr_val = self.data[idx];

        if parent_val > curr_val {
            self.data[parent_idx] = curr_val;
            self.data[idx] = parent_val;
            self.heapify_up(parent_idx);
        }
    }

    fn heapify_down(&mut self, idx: usize) {
        if idx >= self.length {
            return;
        }

        let left_idx = self.left_child(idx);
        let right_idx = self.right_child(idx);

        if left_idx > self.length || right_idx > self.length {
            return;
        }

        let right_val = self.data[right_idx];
        let left_val = self.data[left_idx];
        let curr_val = self.data[idx];

        let (min_val, min_idx) = if left_val < right_val {
            (left_val, left_idx)
        } else {
            (right_val, right_idx)
        };
        if min_val < curr_val {
            self.data[min_idx] = curr_val;
            self.data[idx] = min_val;
            self.heapify_down(min_idx);
        }
    }

    fn parent(&self, idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn left_child(&self, idx: usize) -> usize {
        idx * 2 + 1
    }

    fn right_child(&self, idx: usize) -> usize {
        idx * 2 + 2
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_heap() {
        let mut heap = MinHeap::<i64> {
            length: 0,
            data: Vec::new(),
        };
        heap.insert(3);
        heap.insert(2);
        heap.insert(1);

        assert_eq!(heap.data[0], 1);
        assert_eq!(heap.data[1], 3);
        assert_eq!(heap.data[2], 2);

        assert_eq!(heap.delete(), 1);
        assert_eq!(heap.data[0], 2);
        assert_eq!(heap.data[1], 3);
        assert_eq!(heap.length, 2);
        assert_eq!(heap.delete(), 2);
        assert_eq!(heap.data[0], 3);
        assert_eq!(heap.length, 1);

        heap.insert(1);
        assert_eq!(heap.data[0], 1);
        assert_eq!(heap.data[1], 3);
        assert_eq!(heap.length, 2);

        heap.delete();
        heap.delete();

        assert_eq!(heap.length, 0);
        assert_eq!(heap.data.len(), 0);
    }

    #[test]
    #[should_panic]
    fn test_empty_delete_panics() {
        let mut heap = MinHeap::<i64> {
            length: 0,
            data: Vec::new(),
        };
        heap.delete();
    }
}
