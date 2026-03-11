use crate::algorithms::priority_queue::PriorityQueue;
use crate::search::action::ActionTrait;
use crate::search::search_tree::SearchTree;
use crate::search::state::{StateKey, StateTrait};
use std::collections::{HashMap, HashSet};

// Generic search with closed-list dedupe on generation.
// Used by GBFS/BFS/DFS to preserve existing behavior.
pub fn generic_search<F, G, H, Q, I, S, A>(
    tree: &mut SearchTree<S, A>,
    get_possible_actions: F,
    apply_action: G,
    is_goal: H,
    mut queue: Q,
    heuristic: I,
) -> Result<Vec<A>, &'static str>
where
    F: Fn(&S) -> Vec<A>,
    G: Fn(&S, &A) -> S,
    H: Fn(&S) -> bool,
    Q: PriorityQueue,
    I: Fn(&S) -> f64,
    S: StateTrait + StateKey,
    A: ActionTrait + Clone,
{
    let root_key = if let Some(root_node) = tree.get_node(0) {
        if is_goal(&root_node.state) {
            return Ok(Vec::new());
        }
        let root_heuristic = heuristic(&root_node.state);
        queue.insert(0, 0, root_heuristic);
        root_node.state.state_key()
    } else {
        return Err("Search tree has no root node");
    };

    let mut closed_list: HashSet<S::Key> = HashSet::new();
    closed_list.insert(root_key);

    while let Some(current_index) = queue.pop() {
        let Some(current_node) = tree.get_node(current_index) else {
            continue;
        };
        let current_state = current_node.state.clone();
        let current_cost = current_node.cost;
        let actions = get_possible_actions(&current_state);

        for action in actions {
            let successor_cost = current_cost + action.cost();
            let successor_state = apply_action(&current_state, &action);
            let key = successor_state.state_key();

            // Check if the state is already in the closed list.
            if !closed_list.insert(key) {
                continue;
            }

            let is_goal_state = is_goal(&successor_state);
            let heuristic_value = heuristic(&successor_state);
            let successor_index = tree.add_successor_node(current_index, successor_state, action);

            if is_goal_state {
                return Ok(tree.trace_actions(successor_index));
            }

            queue.insert(successor_index, successor_cost, heuristic_value);
        }
    }

    Err("No solution found")
}

// Cost-aware search for A*/weighted A*.
// Uses best-g relaxation and goal check on pop for correctness.
pub fn generic_search_with_best_cost<F, G, H, Q, I, S, A>(
    tree: &mut SearchTree<S, A>,
    get_possible_actions: F,
    apply_action: G,
    is_goal: H,
    mut queue: Q,
    heuristic: I,
) -> Result<Vec<A>, &'static str>
where
    F: Fn(&S) -> Vec<A>,
    G: Fn(&S, &A) -> S,
    H: Fn(&S) -> bool,
    Q: PriorityQueue,
    I: Fn(&S) -> f64,
    S: StateTrait + StateKey,
    A: ActionTrait + Clone,
{
    let root_key = if let Some(root_node) = tree.get_node(0) {
        if is_goal(&root_node.state) {
            return Ok(Vec::new());
        }
        let root_heuristic = heuristic(&root_node.state);
        queue.insert(0, 0, root_heuristic);
        root_node.state.state_key()
    } else {
        return Err("Search tree has no root node");
    };

    let mut best_cost: HashMap<S::Key, i64> = HashMap::new();
    best_cost.insert(root_key, 0);

    while let Some(current_index) = queue.pop() {
        let Some(current_node) = tree.get_node(current_index) else {
            continue;
        };
        let current_state = current_node.state.clone();
        let current_cost = current_node.cost;
        let current_key = current_state.state_key();

        // Skip stale queue entries that are no longer the best path to the state.
        let Some(best_known_cost) = best_cost.get(&current_key) else {
            continue;
        };
        if current_cost > *best_known_cost {
            continue;
        }

        if is_goal(&current_state) {
            return Ok(tree.trace_actions(current_index));
        }

        let actions = get_possible_actions(&current_state);
        for action in actions {
            let successor_cost = current_cost + action.cost();
            let successor_state = apply_action(&current_state, &action);
            let successor_key = successor_state.state_key();

            let should_relax = match best_cost.get(&successor_key) {
                Some(existing_cost) => successor_cost < *existing_cost,
                None => true,
            };

            if !should_relax {
                continue;
            }

            best_cost.insert(successor_key, successor_cost);
            let heuristic_value = heuristic(&successor_state);
            let successor_index = tree.add_successor_node(current_index, successor_state, action);
            queue.insert(successor_index, successor_cost, heuristic_value);
        }
    }

    Err("No solution found")
}
