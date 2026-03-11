use crate::search::action::ActionTrait;
use crate::search::node::Node;
use crate::search::state::StateTrait;

pub struct SearchTree<S: StateTrait, A: ActionTrait + Clone> {
    pub nodes: Vec<Node<S, A>>, // A vector to store all nodes
}

impl<S: StateTrait, A: ActionTrait + Clone> SearchTree<S, A> {
    // Create a new empty tree with an initial node
    pub fn new(initial_state: S) -> Self {
        let root = Node {
            state: initial_state,
            parent: None,
            action: None,
            cost: 0,
        };

        SearchTree {
            nodes: vec![root], // Add the root node to the nodes vector
        }
    }

    pub fn add_successor_node(&mut self, parent_index: usize, new_state: S, action: A) -> usize {
        let parent_node = &self.nodes[parent_index];
        let new_cost = parent_node.cost + action.cost();

        let new_node = Node {
            state: new_state,
            parent: Some(parent_index),
            action: Some(action),
            cost: new_cost,
        };

        let new_node_index = self.nodes.len();
        self.nodes.push(new_node);
        new_node_index
    }

    // Get the node by its index
    pub fn get_node(&self, index: usize) -> Option<&Node<S, A>> {
        self.nodes.get(index)
    }

    pub fn trace_actions(&self, node_index: usize) -> Vec<A> {
        let mut actions = Vec::new();
        let mut current_index = Some(node_index);
        while let Some(index) = current_index {
            if let Some(node) = self.get_node(index) {
                if let Some(action) = &node.action {
                    actions.push(action.clone());
                }
                current_index = node.parent;
            } else {
                break;
            }
        }
        actions.reverse();
        actions
    }
}
