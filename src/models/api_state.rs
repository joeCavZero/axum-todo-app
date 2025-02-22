use std::sync::Arc;

use tokio::sync::Mutex;

use super::Task;

#[derive(Clone)]
pub struct ApiState {
    pub tasks: Arc<Mutex<Vec<Task>>>,
    pub id_counter: Arc<Mutex<usize>>,
}
impl ApiState {
    pub fn new() -> Self {
        ApiState {
            tasks: Arc::new( Mutex::new( Vec::new() )),
            id_counter: Arc::new(Mutex::new(0)),
        }
    }
}