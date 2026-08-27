use esg_search_core::algorithms::priority_queue::PriorityQueue;
use esg_search_core::algorithms::weighted_astar::WeightedAStarQueue;
use esg_search_core::prelude::{ActionTrait, Problem, StateKey};
use esg_search_core::search::engine::{generic_search, generic_search_with_best_cost};
use esg_search_core::search::search_tree::SearchTree;
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct VoxelState {
    x: i32,
    y: i32,
    z: i32,
}

impl StateKey for VoxelState {
    type Key = (i32, i32, i32);

    fn state_key(&self) -> Self::Key {
        (self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Move3d {
    dx: i8,
    dy: i8,
    dz: i8,
}

impl ActionTrait for Move3d {
    fn name(&self) -> &str {
        "move3d"
    }

    fn cost(&self) -> i64 {
        1
    }
}

struct VoxelProblem {
    size_x: i32,
    size_y: i32,
    size_z: i32,
    blocked: Vec<bool>,
    goal: VoxelState,
    moves: Vec<Move3d>,
}

impl VoxelProblem {
    fn index(&self, x: i32, y: i32, z: i32) -> usize {
        ((z * self.size_y + y) * self.size_x + x) as usize
    }

    fn in_bounds(&self, x: i32, y: i32, z: i32) -> bool {
        x >= 0 && y >= 0 && z >= 0 && x < self.size_x && y < self.size_y && z < self.size_z
    }

    fn is_blocked(&self, x: i32, y: i32, z: i32) -> bool {
        self.blocked[self.index(x, y, z)]
    }

    fn blocked_count(&self) -> usize {
        self.blocked.iter().filter(|&&b| b).count()
    }
}

impl Problem for VoxelProblem {
    type State = VoxelState;
    type Action = Move3d;

    fn get_possible_actions(&self, state: &Self::State) -> Vec<Self::Action> {
        let mut actions = Vec::with_capacity(26);
        for mv in &self.moves {
            let nx = state.x + mv.dx as i32;
            let ny = state.y + mv.dy as i32;
            let nz = state.z + mv.dz as i32;
            if self.in_bounds(nx, ny, nz) && !self.is_blocked(nx, ny, nz) {
                actions.push(*mv);
            }
        }
        actions
    }

    fn apply_action(&self, state: &Self::State, action: &Self::Action) -> Self::State {
        VoxelState {
            x: state.x + action.dx as i32,
            y: state.y + action.dy as i32,
            z: state.z + action.dz as i32,
        }
    }

    fn is_goal_state(&self, state: &Self::State) -> bool {
        state == &self.goal
    }

    fn heuristic(&self, state: &Self::State) -> f64 {
        let dx = (self.goal.x - state.x).abs();
        let dy = (self.goal.y - state.y).abs();
        let dz = (self.goal.z - state.z).abs();
        dx.max(dy).max(dz) as f64
    }
}

#[derive(Clone, Copy)]
struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    fn new(seed: u64) -> Self {
        let state = if seed == 0 {
            0x9E37_79B9_7F4A_7C15
        } else {
            seed
        };
        Self { state }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    fn next_f64(&mut self) -> f64 {
        let v = self.next_u64() >> 11;
        (v as f64) / ((1_u64 << 53) as f64)
    }
}

fn generate_26_moves() -> Vec<Move3d> {
    let mut moves = Vec::with_capacity(26);
    for dz in -1..=1 {
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 && dz == 0 {
                    continue;
                }
                moves.push(Move3d {
                    dx: dx as i8,
                    dy: dy as i8,
                    dz: dz as i8,
                });
            }
        }
    }
    moves
}

fn build_problem(
    size_x: i32,
    size_y: i32,
    size_z: i32,
    blocked_ratio: f64,
    seed: u64,
) -> (VoxelProblem, VoxelState) {
    let mut rng = XorShift64::new(seed);
    let total = (size_x * size_y * size_z) as usize;
    let mut blocked = vec![false; total];

    for b in &mut blocked {
        *b = rng.next_f64() < blocked_ratio;
    }

    let start = VoxelState { x: 0, y: 0, z: 0 };
    let goal = VoxelState {
        x: size_x - 1,
        y: size_y - 1,
        z: size_z - 1,
    };

    let start_idx = ((start.z * size_y + start.y) * size_x + start.x) as usize;
    let goal_idx = ((goal.z * size_y + goal.y) * size_x + goal.x) as usize;
    blocked[start_idx] = false;
    blocked[goal_idx] = false;

    let problem = VoxelProblem {
        size_x,
        size_y,
        size_z,
        blocked,
        goal,
        moves: generate_26_moves(),
    };
    (problem, start)
}

struct RunStats {
    label: &'static str,
    solved: bool,
    elapsed_us: u128,
    path_len: usize,
    tree_nodes: usize,
    unique_states: usize,
    duplicate_nodes: usize,
    get_actions_us: u128,
    apply_action_us: u128,
    goal_check_us: u128,
    heuristic_us: u128,
}

fn run_profile<Q: PriorityQueue>(
    label: &'static str,
    problem: &VoxelProblem,
    initial_state: VoxelState,
    queue: Q,
    use_best_cost_search: bool,
) -> RunStats {
    let mut tree: SearchTree<VoxelState, Move3d> = SearchTree::new(initial_state);

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
    let elapsed_us = start.elapsed().as_micros();

    let (solved, path_len) = match result {
        Ok(actions) => (true, actions.len()),
        Err(_) => (false, 0),
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
        solved,
        elapsed_us,
        path_len,
        tree_nodes,
        unique_states,
        duplicate_nodes,
        get_actions_us: get_actions_ns.get() / 1_000,
        apply_action_us: apply_action_ns.get() / 1_000,
        goal_check_us: goal_check_ns.get() / 1_000,
        heuristic_us: heuristic_ns.get() / 1_000,
    }
}

fn main() {
    let size_x = 200;
    let size_y = 200;
    let size_z = 200;
    let blocked_ratio = 0.55;
    let seed = 0xD00D_BAAD_F00D_CAFE;

    let (problem, initial_state) = build_problem(size_x, size_y, size_z, blocked_ratio, seed);
    let blocked_count = problem.blocked_count();
    let total_voxels = (size_x * size_y * size_z) as usize;
    let blocked_pct = (blocked_count as f64 / total_voxels as f64) * 100.0;

    println!(
        "3D grid: {}x{}x{}, blocked: {} / {} ({:.2}%), moves: 26",
        size_x, size_y, size_z, blocked_count, total_voxels, blocked_pct
    );

    let astar = run_profile(
        "A*",
        &problem,
        initial_state.clone(),
        WeightedAStarQueue::new_astar(),
        true,
    );
    let gbfs = run_profile(
        "GBFS",
        &problem,
        initial_state,
        WeightedAStarQueue::new_gbfs(),
        false,
    );

    println!(
        "Strategy   Solved   Time(us)   PathLen   TreeNodes   UniqueStates   DuplicateNodes   get_actions(us)   apply_action(us)   goal_check(us)   heuristic(us)"
    );
    for stats in [astar, gbfs] {
        println!(
            "{:<8} {:>8} {:>9} {:>9} {:>11} {:>14} {:>16} {:>17} {:>18} {:>16} {:>14}",
            stats.label,
            if stats.solved { "yes" } else { "no" },
            stats.elapsed_us,
            stats.path_len,
            stats.tree_nodes,
            stats.unique_states,
            stats.duplicate_nodes,
            stats.get_actions_us,
            stats.apply_action_us,
            stats.goal_check_us,
            stats.heuristic_us
        );
    }
}
