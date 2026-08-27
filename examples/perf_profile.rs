use search_core::algorithms::bfs::BfsQueue;
use search_core::algorithms::priority_queue::PriorityQueue;
use search_core::algorithms::weighted_astar::WeightedAStarQueue;
use search_core::prelude::{ActionTrait, Problem, SearchStrategy, StateKey};
use search_core::search::engine::{generic_search, generic_search_with_best_cost};
use search_core::search::search_tree::SearchTree;
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct GridState {
    x: i32,
    y: i32,
}

impl StateKey for GridState {
    type Key = (i32, i32);

    fn state_key(&self) -> Self::Key {
        (self.x, self.y)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum GridAction {
    Up,
    Down,
    Left,
    Right,
}

impl ActionTrait for GridAction {
    fn name(&self) -> &str {
        match self {
            GridAction::Up => "up",
            GridAction::Down => "down",
            GridAction::Left => "left",
            GridAction::Right => "right",
        }
    }

    fn cost(&self) -> i64 {
        1
    }
}

struct GridProblem {
    width: i32,
    height: i32,
    goal: GridState,
}

impl Problem for GridProblem {
    type State = GridState;
    type Action = GridAction;

    fn get_possible_actions(&self, state: &Self::State) -> Vec<Self::Action> {
        let mut actions = Vec::with_capacity(4);
        if state.y > 0 {
            actions.push(GridAction::Up);
        }
        if state.y + 1 < self.height {
            actions.push(GridAction::Down);
        }
        if state.x > 0 {
            actions.push(GridAction::Left);
        }
        if state.x + 1 < self.width {
            actions.push(GridAction::Right);
        }
        actions
    }

    fn apply_action(&self, state: &Self::State, action: &Self::Action) -> Self::State {
        match action {
            GridAction::Up => GridState {
                x: state.x,
                y: state.y - 1,
            },
            GridAction::Down => GridState {
                x: state.x,
                y: state.y + 1,
            },
            GridAction::Left => GridState {
                x: state.x - 1,
                y: state.y,
            },
            GridAction::Right => GridState {
                x: state.x + 1,
                y: state.y,
            },
        }
    }

    fn is_goal_state(&self, state: &Self::State) -> bool {
        state == &self.goal
    }

    fn heuristic(&self, state: &Self::State) -> f64 {
        ((self.goal.x - state.x).abs() + (self.goal.y - state.y).abs()) as f64
    }
}

struct RunStats {
    label: &'static str,
    elapsed_ms: u128,
    path_len: usize,
    tree_nodes: usize,
    unique_states: usize,
    duplicate_nodes: usize,
    get_actions_ms: u128,
    apply_action_ms: u128,
    goal_check_ms: u128,
    heuristic_ms: u128,
}

fn run_profile<Q: PriorityQueue>(
    label: &'static str,
    problem: &GridProblem,
    initial_state: GridState,
    queue: Q,
    use_best_cost_search: bool,
) -> RunStats {
    let mut tree: SearchTree<GridState, GridAction> = SearchTree::new(initial_state);

    let get_actions_ns = Cell::new(0_u128);
    let apply_action_ns = Cell::new(0_u128);
    let goal_check_ns = Cell::new(0_u128);
    let heuristic_ns = Cell::new(0_u128);

    let start = Instant::now();
    let result = if use_best_cost_search {
        generic_search_with_best_cost(
            &mut tree,
            |state| {
                let t0 = Instant::now();
                let actions = problem.get_possible_actions(state);
                get_actions_ns.set(get_actions_ns.get() + t0.elapsed().as_nanos());
                actions
            },
            |state, action| {
                let t0 = Instant::now();
                let next = problem.apply_action(state, action);
                apply_action_ns.set(apply_action_ns.get() + t0.elapsed().as_nanos());
                next
            },
            |state| {
                let t0 = Instant::now();
                let is_goal = problem.is_goal_state(state);
                goal_check_ns.set(goal_check_ns.get() + t0.elapsed().as_nanos());
                is_goal
            },
            queue,
            |state| {
                let t0 = Instant::now();
                let h = problem.heuristic(state);
                heuristic_ns.set(heuristic_ns.get() + t0.elapsed().as_nanos());
                h
            },
        )
    } else {
        generic_search(
            &mut tree,
            |state| {
                let t0 = Instant::now();
                let actions = problem.get_possible_actions(state);
                get_actions_ns.set(get_actions_ns.get() + t0.elapsed().as_nanos());
                actions
            },
            |state, action| {
                let t0 = Instant::now();
                let next = problem.apply_action(state, action);
                apply_action_ns.set(apply_action_ns.get() + t0.elapsed().as_nanos());
                next
            },
            |state| {
                let t0 = Instant::now();
                let is_goal = problem.is_goal_state(state);
                goal_check_ns.set(goal_check_ns.get() + t0.elapsed().as_nanos());
                is_goal
            },
            queue,
            |state| {
                let t0 = Instant::now();
                let h = problem.heuristic(state);
                heuristic_ns.set(heuristic_ns.get() + t0.elapsed().as_nanos());
                h
            },
        )
    };
    let elapsed_ms = start.elapsed().as_millis();

    let path_len = match result {
        Ok(actions) => actions.len(),
        Err(err) => {
            eprintln!("[{}] search failed: {}", label, err);
            0
        }
    };

    let mut unique = HashSet::new();
    for node in &tree.nodes {
        unique.insert(node.state.state_key());
    }

    let tree_nodes = tree.nodes.len();
    let unique_states = unique.len();
    let duplicate_nodes = tree_nodes.saturating_sub(unique_states);

    RunStats {
        label,
        elapsed_ms,
        path_len,
        tree_nodes,
        unique_states,
        duplicate_nodes,
        get_actions_ms: get_actions_ns.get() / 1_000_000,
        apply_action_ms: apply_action_ns.get() / 1_000_000,
        goal_check_ms: goal_check_ns.get() / 1_000_000,
        heuristic_ms: heuristic_ns.get() / 1_000_000,
    }
}

fn main() {
    let width = 220;
    let height = 220;
    let problem = GridProblem {
        width,
        height,
        goal: GridState {
            x: width - 1,
            y: height - 1,
        },
    };

    let initial_state = GridState { x: 0, y: 0 };

    let bfs = run_profile(
        "BFS",
        &problem,
        initial_state.clone(),
        BfsQueue::new(),
        false,
    );
    let astar = run_profile(
        "A*",
        &problem,
        initial_state,
        WeightedAStarQueue::new_astar(),
        true,
    );

    println!(
        "Strategy   Time(ms)   PathLen   TreeNodes   UniqueStates   DuplicateNodes   get_actions(ms)   apply_action(ms)   goal_check(ms)   heuristic(ms)"
    );
    for stats in [bfs, astar] {
        println!(
            "{:<8} {:>8} {:>9} {:>11} {:>14} {:>16} {:>17} {:>18} {:>16} {:>14}",
            stats.label,
            stats.elapsed_ms,
            stats.path_len,
            stats.tree_nodes,
            stats.unique_states,
            stats.duplicate_nodes,
            stats.get_actions_ms,
            stats.apply_action_ms,
            stats.goal_check_ms,
            stats.heuristic_ms
        );
    }

    println!("Note: DuplicateNodes should remain near 0 after pre-insertion dedupe.");

    let _ = SearchStrategy::AStar;
}
