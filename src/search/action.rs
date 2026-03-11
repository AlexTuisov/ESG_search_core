#[derive(Debug, Clone, PartialEq)]
pub struct Action<P> {
    pub name: String,
    pub cost: i64,
    pub parameters: P,
}

pub type SimpleAction = Action<()>;

impl<P> Action<P> {
    pub fn new(name: String, cost: i64, parameters: P) -> Self {
        Action {
            name,
            cost,
            parameters,
        }
    }
}

impl Action<()> {
    pub fn without_parameters(name: String, cost: i64) -> Self {
        Action::new(name, cost, ())
    }
}

pub trait ActionTrait {
    fn name(&self) -> &str;
    fn cost(&self) -> i64;
}

impl<P> ActionTrait for Action<P> {
    fn name(&self) -> &str {
        &self.name
    }

    fn cost(&self) -> i64 {
        self.cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_create_action_with_parameters() {
        let mut parameters = HashMap::new();
        parameters.insert("speed".to_string(), 10);
        parameters.insert("fuel".to_string(), 90);

        let action = Action::new("move".to_string(), 5, parameters);

        assert_eq!(action.name, "move");
        assert_eq!(action.cost, 5);
        assert_eq!(action.parameters.get("speed"), Some(&10));
        assert_eq!(action.parameters.get("fuel"), Some(&90));
    }
}
