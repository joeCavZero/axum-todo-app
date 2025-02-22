#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub content: String,
    pub completed: bool,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct TaskPost {
    pub title: String,
    pub content: String,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct TaskUpdate {
    pub title: Option<String>,
    pub content: Option<String>,
    pub completed: Option<bool>,
}