use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct toDoList {
    pub task_id: Option<String>,
    pub task: String,
    pub status: bool,
    pub username: String
}