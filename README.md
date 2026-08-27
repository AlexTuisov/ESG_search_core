# ESG Search Core

`esg-search-core` is a generic Rust search engine crate for state-space problems. ESG stands for Explicit Successor Generator.

It lets users plug in their own:
- state type
- action type
- transition logic
- goal logic
- optional heuristic

Then solve with `A*`, `GBFS`, `BFS`, `DFS`, or weighted A*.

## Features

- Typed actions via `Problem::Action`.
- Generic solver API for direct state+problem solving.
- Optional input loader abstraction (`ProblemInput`).
- Optional heuristic (`0.0` by default).
- Public prelude exports for fast setup.

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
esg-search-core = "0.1.0"
serde = { version = "1", features = ["derive"] }
```

For local workspace usage:

```toml
[dependencies]
esg-search-core = { path = "../ESG_search_core" }
serde = { version = "1", features = ["derive"] }
```

## Quick Start (No `ProblemInput`)

This example uses `SimpleAction` and omits `heuristic`, so the default `0.0` is used.

```rust
use esg_search_core::prelude::*;
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
    type Action = SimpleAction;

    fn get_possible_actions(&self, state: &Self::State) -> Vec<Self::Action> {
        if state.value < self.goal {
            vec![Action::without_parameters("inc".to_string(), 1)]
        } else if state.value > self.goal {
            vec![Action::without_parameters("dec".to_string(), 1)]
        } else {
            Vec::new()
        }
    }

    fn apply_action(&self, state: &Self::State, action: &Self::Action) -> Self::State {
        match action.name.as_str() {
            "inc" => CounterState { value: state.value + 1 },
            "dec" => CounterState { value: state.value - 1 },
            _ => state.clone(),
        }
    }

    fn is_goal_state(&self, state: &Self::State) -> bool {
        state.value == self.goal
    }

    // heuristic is optional:
    // fn heuristic(&self, state: &Self::State) -> f64 { ... }
}

fn main() {
    let problem = CounterProblem { goal: 3 };
    let initial_state = CounterState { value: 0 };

    let solution = solve_problem(&problem, initial_state, SearchStrategy::AStar)
        .expect("solution should exist");

    assert_eq!(solution.action_names(), vec!["inc", "inc", "inc"]);
    assert_eq!(solution.total_cost, 3);
}
```

## With `ProblemInput`

Use `ProblemInput` when you want input parsing/loading separated from solving:

```rust
use esg_search_core::prelude::*;
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

struct CounterInput {
    start: i32,
    goal: i32,
}

impl Problem for CounterProblem {
    type State = CounterState;
    type Action = SimpleAction;

    fn get_possible_actions(&self, state: &Self::State) -> Vec<Self::Action> {
        if state.value < self.goal {
            vec![Action::without_parameters("inc".to_string(), 1)]
        } else {
            Vec::new()
        }
    }

    fn apply_action(&self, state: &Self::State, _action: &Self::Action) -> Self::State {
        CounterState { value: state.value + 1 }
    }

    fn is_goal_state(&self, state: &Self::State) -> bool {
        state.value == self.goal
    }
}

impl ProblemInput<CounterProblem> for CounterInput {
    fn load_state(input: Self) -> (CounterState, CounterProblem) {
        (
            CounterState { value: input.start },
            CounterProblem { goal: input.goal },
        )
    }
}

fn main() {
    let input = CounterInput { start: 0, goal: 2 };
    let solution = solve_problem_with_input::<CounterProblem, _>(input, SearchStrategy::Bfs)
        .expect("solution should exist");
    assert_eq!(solution.action_names(), vec!["inc", "inc"]);
}
```

## Required vs Optional

Required:
- `State`: implements `StateKey` and serde/debug/eq/clone bounds.
- `Problem` impl with:
  - `type State`
  - `type Action`
  - `get_possible_actions`
  - `apply_action`
  - `is_goal_state`

Optional:
- `heuristic` override (defaults to `0.0`).
- `ProblemInput` trait impl (only needed for `solve_problem_with_input*` helpers).
- custom action struct; you can also use `Action<P>` or `SimpleAction`.

## Public API

- `solve_problem(&problem, initial_state, SearchStrategy)`
- `solve_problem_with_input::<P, I>(input, SearchStrategy)`
- `solve_problem_with_input_str::<P, I>(input, "A*" | "GBFS" | "BFS" | "DFS")`
- `SearchStrategy::WeightedAStar { weight }` for custom weighted A*.

Returns:
- `Result<Solution<P::Action>, SolveError>`

`Solution` includes:
- `actions: Vec<P::Action>`
- `total_cost: i64`
- `action_names() -> Vec<String>`

## Notes

- If the initial state is already a goal state, the solver returns an empty action plan.
- `InvalidWeightedAStarWeight` is returned for non-finite or non-positive weights.
- `NoSolution` is returned when the search space is exhausted without a goal.
