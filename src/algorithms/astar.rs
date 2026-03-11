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

pub struct AStarQueue {
    heap: BinaryHeap<Reverse<QueueEntry>>,
}

impl AStarQueue {
    pub fn new() -> Self {
        AStarQueue {
            heap: BinaryHeap::new(),
        }
    }
}

impl PriorityQueue for AStarQueue {
    fn insert(&mut self, node_index: usize, cost: i64, heuristic: f64) {
        let priority = cost as f64 + heuristic;
        self.heap.push(Reverse(QueueEntry {
            priority,
            node_index,
        }));
    }

    fn pop(&mut self) -> Option<usize> {
        self.heap.pop().map(|Reverse(entry)| entry.node_index)
    }
}
