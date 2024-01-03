
struct SnapshotArray {
    array: Vec<Vec<(i32, i32)>>,
    snap_id: i32,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        SnapshotArray {
            array: vec![vec![(0, 0)]; length as usize],
            snap_id: 0,
        }
    }
    
    fn set(&mut self, index: i32, val: i32) {
        let len = self.array[index as usize].len();
        if len > 0 && self.array[index as usize][len - 1].0 == self.snap_id {
            self.array[index as usize][len - 1].1 = val;
        } else {
            self.array[index as usize].push((self.snap_id, val));
        }
    }
    
    fn snap(&mut self) -> i32 {
        let current_snap_id = self.snap_id;
        self.snap_id += 1;
        current_snap_id

    }
    
    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let snapshots = &self.array[index as usize];
        let len = snapshots.len();
        if len == 0 || snap_id < snapshots[0].0 {
            return 0;
        }
        let mut left = 0;
        let mut right = len - 1;
        while left < right {
            let mid = (left + right + 1) / 2;
            if snapshots[mid].0 <= snap_id {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        snapshots[left].1

    }
}
