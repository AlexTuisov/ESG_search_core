use crate::search::action::ActionTrait;
use crate::search::state::StateTrait;

pub trait Problem {
    type State: StateTrait; // Associated type for State
    type Action: ActionTrait + Clone;

    fn get_possible_actions(&self, state: &Self::State) -> Vec<Self::Action>;
    fn apply_action(&self, state: &Self::State, action: &Self::Action) -> Self::State;
    fn is_goal_state(&self, state: &Self::State) -> bool;
    fn heuristic(&self, _state: &Self::State) -> f64 {
        0.0
    }
    // fn load_state_from_json(json_path: &str) -> (Self::State, Self);
}

pub trait ProblemInput<P: Problem> {
    fn load_state(input: Self) -> (P::State, P);
}
