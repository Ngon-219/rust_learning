use serde::{Serialize, Deserialize};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct toDoList {
    pub task: String,
    pub status: bool,
    pub username: String
}