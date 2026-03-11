// Define a trait for the priority queue to be used in the generic search.
pub trait PriorityQueue {
    // Insert a node with its cost or priority.
    fn insert(&mut self, node_index: usize, cost: i64, heuristic_value: f64);

    // Pop the next node based on the queue ordering.
    fn pop(&mut self) -> Option<usize>;
}
