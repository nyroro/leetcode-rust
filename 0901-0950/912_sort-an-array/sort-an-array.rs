
impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut heap = nums.clone();
        let n = heap.len();

        // 建立最大堆

        for i in (0..n / 2).rev() {
            Solution::heapify(&mut heap, n, i);
        }

        // 依次将最大元素放到数组末尾，并重新调整堆

        for i in (1..n).rev() {
            heap.swap(0, i);
            Solution::heapify(&mut heap, i, 0);
        }

        heap

    }

    fn heapify(heap: &mut Vec<i32>, n: usize, i: usize) {
        let mut largest = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        if left < n && heap[left] > heap[largest] {
            largest = left;
        }

        if right < n && heap[right] > heap[largest] {
            largest = right;
        }

        if largest != i {
            heap.swap(i, largest);
            Solution::heapify(heap, n, largest);
        }
    }
}
