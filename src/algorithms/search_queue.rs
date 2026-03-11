use crate::algorithms::bfs::BfsQueue;
use crate::algorithms::dfs::DfsQueue;
use crate::algorithms::priority_queue::PriorityQueue;
use crate::algorithms::weighted_astar::WeightedAStarQueue;

pub enum SearchQueue {
    AStar(WeightedAStarQueue),
    GBFS(WeightedAStarQueue),
    WeightedAStar(WeightedAStarQueue),
    BFS(BfsQueue),
    DFS(DfsQueue),
}

impl PriorityQueue for SearchQueue {
    fn insert(&mut self, node_index: usize, cost: i64, heuristic_value: f64) {
        match self {
            SearchQueue::AStar(queue) => queue.insert(node_index, cost, heuristic_value),
            SearchQueue::GBFS(queue) => queue.insert(node_index, cost, heuristic_value),
            SearchQueue::WeightedAStar(queue) => queue.insert(node_index, cost, heuristic_value),
            SearchQueue::BFS(queue) => queue.insert(node_index, cost, heuristic_value),
            SearchQueue::DFS(queue) => queue.insert(node_index, cost, heuristic_value),
        }
    }

    fn pop(&mut self) -> Option<usize> {
        match self {
            SearchQueue::AStar(queue) => queue.pop(),
            SearchQueue::GBFS(queue) => queue.pop(),
            SearchQueue::WeightedAStar(queue) => queue.pop(),
            SearchQueue::BFS(queue) => queue.pop(),
            SearchQueue::DFS(queue) => queue.pop(),
        }
    }
}
