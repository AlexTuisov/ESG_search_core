// use std::cell::RefCell;
// use std::rc::{Rc, Weak};
use crate::search::state::StateTrait;

#[derive(Debug, Clone, PartialEq)]
pub struct Node<S: StateTrait, A: Clone> {
    pub state: S,
    pub parent: Option<usize>,
    pub action: Option<A>,
    pub cost: i64,
}

impl<S: StateTrait, A: Clone> Node<S, A> {
    pub fn new_empty(state: S) -> Self {
        Node {
            state,
            parent: None,
            action: None,
            cost: 0,
        }
    }
}
