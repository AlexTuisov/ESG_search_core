use crate::algorithms::priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy, PartialEq)]
struct QueueEntry {
    priority: f64,
    node_index: usize,
}

impl Eq for QueueEntry {}

impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority
            .total_cmp(&other.priority)
            .then_with(|| self.node_index.cmp(&other.node_index))
    }
}

impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct GBFSQueue {
    heap: BinaryHeap<Reverse<QueueEntry>>,
}

impl GBFSQueue {
    pub fn new() -> Self {
        GBFSQueue {
            heap: BinaryHeap::new(),
        }
    }
}

impl Default for GBFSQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl PriorityQueue for GBFSQueue {
    fn insert(&mut self, node_index: usize, _cost: i64, heuristic: f64) {
        self.heap.push(Reverse(QueueEntry {
            priority: heuristic,
            node_index,
        }));
    }

    fn pop(&mut self) -> Option<usize> {
        self.heap.pop().map(|Reverse(entry)| entry.node_index)
    }
}
