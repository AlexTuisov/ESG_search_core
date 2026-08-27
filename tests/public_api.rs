use esg_search_core::{
    solve_problem, solve_problem_with_input, solve_problem_with_input_str, Action, Problem,
    ProblemInput, SearchStrategy, StateKey,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct CounterState {
    value: i32,
}

impl StateKey for CounterState {
    type Key = i32;

    fn state_key(&self) -> Self::Key {
        self.value
    }
}

struct CounterProblem {
    goal: i32,
}

impl Problem for CounterProblem {
    type State = CounterState;
    type Action = Action<()>;

    fn get_possible_actions(&self, state: &Self::State) -> Vec<Self::Action> {
        if state.value < self.goal {
            vec![Action::new("inc".to_string(), 1, ())]
        } else if state.value > self.goal {
            vec![Action::new("dec".to_string(), 1, ())]
        } else {
            Vec::new()
        }
    }

    fn apply_action(&self, state: &Self::State, action: &Self::Action) -> Self::State {
        match action.name.as_str() {
            "inc" => CounterState {
                value: state.value + 1,
            },
            "dec" => CounterState {
                value: state.value - 1,
            },
            _ => state.clone(),
        }
    }

    fn is_goal_state(&self, state: &Self::State) -> bool {
        state.value == self.goal
    }

    fn heuristic(&self, state: &Self::State) -> f64 {
        (self.goal - state.value).abs() as f64
    }
}

struct CounterInput {
    start: i32,
    goal: i32,
}

impl ProblemInput<CounterProblem> for CounterInput {
    fn load_state(input: Self) -> (CounterState, CounterProblem) {
        (
            CounterState { value: input.start },
            CounterProblem { goal: input.goal },
        )
    }
}

#[test]
fn user_can_solve_with_problem_and_input() {
    let solution = solve_problem_with_input::<CounterProblem, _>(
        CounterInput { start: 0, goal: 3 },
        SearchStrategy::AStar,
    )
    .expect("expected a solution");

    assert_eq!(solution.action_names(), vec!["inc", "inc", "inc"]);
    assert_eq!(solution.total_cost, 3);
}

#[test]
fn user_can_solve_with_problem_and_state() {
    let problem = CounterProblem { goal: 2 };
    let initial_state = CounterState { value: 0 };

    let solution = solve_problem(
        &problem,
        initial_state,
        SearchStrategy::WeightedAStar { weight: 1.5 },
    )
    .expect("expected a solution");

    assert_eq!(solution.action_names(), vec!["inc", "inc"]);
}

#[test]
fn user_can_solve_with_string_strategy() {
    let solution = solve_problem_with_input_str::<CounterProblem, _>(
        CounterInput { start: 0, goal: 1 },
        "BFS",
    )
    .expect("expected a solution");

    assert_eq!(solution.action_names(), vec!["inc"]);
}

#[test]
fn user_can_solve_with_gbfs_strategy() {
    let solution = solve_problem_with_input::<CounterProblem, _>(
        CounterInput { start: 0, goal: 2 },
        SearchStrategy::Gbfs,
    )
    .expect("expected a solution");

    assert_eq!(solution.action_names(), vec!["inc", "inc"]);
}

#[test]
fn already_solved_input_returns_empty_plan() {
    let solution = solve_problem_with_input::<CounterProblem, _>(
        CounterInput { start: 5, goal: 5 },
        SearchStrategy::Dfs,
    )
    .expect("expected a solution");

    assert!(solution.actions.is_empty());
    assert_eq!(solution.total_cost, 0);
}
