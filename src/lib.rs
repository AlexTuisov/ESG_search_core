pub mod algorithms;
pub mod problems;
pub mod search;

pub use problems::problem::{Problem, ProblemInput};
pub use search::action::{Action, ActionTrait, SimpleAction};
pub use search::solve::{
    solve_problem, solve_problem_with_input, solve_problem_with_input_str, SearchStrategy,
    Solution, SolveError,
};
pub use search::state::{StateKey, StateTrait};

pub mod prelude {
    pub use crate::problems::problem::{Problem, ProblemInput};
    pub use crate::search::action::{Action, ActionTrait, SimpleAction};
    pub use crate::search::solve::{
        solve_problem, solve_problem_with_input, solve_problem_with_input_str, SearchStrategy,
        Solution, SolveError,
    };
    pub use crate::search::state::{StateKey, StateTrait};
}
