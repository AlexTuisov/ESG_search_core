use crate::algorithms::bfs::BfsQueue;
use crate::algorithms::dfs::DfsQueue;
use crate::algorithms::search_queue::SearchQueue;
use crate::algorithms::weighted_astar::WeightedAStarQueue;
use crate::problems::problem::{Problem, ProblemInput};
use crate::search::action::ActionTrait;
use crate::search::search_tree::SearchTree;
use crate::search::state::StateKey;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SearchStrategy {
    AStar,
    Gbfs,
    Bfs,
    Dfs,
    WeightedAStar { weight: f64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolveError {
    NoSolution,
    InvalidWeightedAStarWeight,
    InvalidSearchStrategy,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Solution<A: ActionTrait + Clone> {
    pub actions: Vec<A>,
    pub total_cost: i64,
}

impl<A: ActionTrait + Clone> Solution<A> {
    pub fn action_names(&self) -> Vec<String> {
        self.actions
            .iter()
            .map(|action| action.name().to_string())
            .collect()
    }
}

impl FromStr for SearchStrategy {
    type Err = SolveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "A*" | "ASTAR" | "AStar" => Ok(SearchStrategy::AStar),
            "GBFS" => Ok(SearchStrategy::Gbfs),
            "BFS" => Ok(SearchStrategy::Bfs),
            "DFS" => Ok(SearchStrategy::Dfs),
            _ => Err(SolveError::InvalidSearchStrategy),
        }
    }
}

pub fn solve_problem<P>(
    problem: &P,
    initial_state: P::State,
    search_strategy: SearchStrategy,
) -> Result<Solution<P::Action>, SolveError>
where
    P: Problem,
    P::State: StateKey,
{
    let mut tree: SearchTree<P::State, P::Action> = SearchTree::new(initial_state);
    let queue = build_queue(search_strategy)?;
    let result = if matches!(
        search_strategy,
        SearchStrategy::AStar | SearchStrategy::WeightedAStar { .. }
    ) {
        crate::search::search::generic_search_with_best_cost(
            &mut tree,
            |state| problem.get_possible_actions(state),
            |state, action| problem.apply_action(state, action),
            |state| problem.is_goal_state(state),
            queue,
            |state| problem.heuristic(state),
        )
    } else {
        crate::search::search::generic_search(
            &mut tree,
            |state| problem.get_possible_actions(state),
            |state, action| problem.apply_action(state, action),
            |state| problem.is_goal_state(state),
            queue,
            |state| problem.heuristic(state),
        )
    };

    process_search_result(result)
}

pub fn solve_problem_with_input<P, I>(
    input: I,
    search_strategy: SearchStrategy,
) -> Result<Solution<P::Action>, SolveError>
where
    P: Problem,
    I: ProblemInput<P>,
    P::State: StateKey,
{
    let (initial_state, problem) = I::load_state(input);
    solve_problem(&problem, initial_state, search_strategy)
}

pub fn solve_problem_with_input_str<P, I>(
    input: I,
    search_strategy: &str,
) -> Result<Solution<P::Action>, SolveError>
where
    P: Problem,
    I: ProblemInput<P>,
    P::State: StateKey,
{
    let parsed_strategy = SearchStrategy::from_str(search_strategy)?;
    solve_problem_with_input::<P, I>(input, parsed_strategy)
}

fn build_queue(search_strategy: SearchStrategy) -> Result<SearchQueue, SolveError> {
    let queue = match search_strategy {
        SearchStrategy::AStar => SearchQueue::AStar(WeightedAStarQueue::new_astar()),
        SearchStrategy::Gbfs => SearchQueue::GBFS(WeightedAStarQueue::new_gbfs()),
        SearchStrategy::Bfs => SearchQueue::BFS(BfsQueue::new()),
        SearchStrategy::Dfs => SearchQueue::DFS(DfsQueue::new()),
        SearchStrategy::WeightedAStar { weight } => {
            if !weight.is_finite() || weight <= 0.0 {
                return Err(SolveError::InvalidWeightedAStarWeight);
            }
            SearchQueue::WeightedAStar(WeightedAStarQueue::new(weight))
        }
    };

    Ok(queue)
}

fn process_search_result<A>(result: Result<Vec<A>, &str>) -> Result<Solution<A>, SolveError>
where
    A: ActionTrait + Clone,
{
    match result {
        Ok(actions) => {
            let total_cost = actions.iter().map(|action| action.cost()).sum();
            Ok(Solution {
                actions,
                total_cost,
            })
        }
        Err(..) => Err(SolveError::NoSolution),
    }
}
