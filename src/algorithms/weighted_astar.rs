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

/// Weighted A* priority queue.
///
/// Priority is computed as: f(n) = g(n) + w * h(n)
///
/// Aliases:
/// - A* == weighted A* with w = 1.0
/// - GBFS is approximated with a large w (see `new_gbfs`).
pub struct WeightedAStarQueue {
    heap: BinaryHeap<Reverse<QueueEntry>>,
    w: f64,
}

impl WeightedAStarQueue {
    pub fn new(w: f64) -> Self {
        Self {
            heap: BinaryHeap::new(),
            w,
        }
    }

    pub fn new_astar() -> Self {
        Self::new(1.0)
    }

    pub fn new_gbfs() -> Self {
        // Not a mathematically exact GBFS (which ignores g), but a practical alias
        // that heavily favors h.
        Self::new(1.0e3)
    }
}

impl PriorityQueue for WeightedAStarQueue {
    fn insert(&mut self, node_index: usize, cost: i64, heuristic: f64) {
        let priority = cost as f64 + self.w * heuristic;
        self.heap.push(Reverse(QueueEntry {
            priority,
            node_index,
        }));
    }

    fn pop(&mut self) -> Option<usize> {
        self.heap.pop().map(|Reverse(entry)| entry.node_index)
    }
}
