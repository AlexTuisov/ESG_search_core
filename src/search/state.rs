use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;

pub trait StateTrait: Debug + Clone + Serialize + for<'de> Deserialize<'de> + Eq {}

impl<T> StateTrait for T where T: Debug + Clone + Serialize + for<'de> Deserialize<'de> + Eq {}

pub trait StateKey {
    type Key: Eq + Hash;

    fn state_key(&self) -> Self::Key;
}
