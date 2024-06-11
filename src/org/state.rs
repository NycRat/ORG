#[derive(Debug)]
pub struct State {}

#[derive(Debug, Clone)]
pub struct Goal {
    pub id: String,
    pub name: String,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub name: String,
}

pub type GoalOrder = Vec<String>; // order of goal ids
