use sqlx::PgPool;
use crate::model::toDoList;

pub struct ToDoListRepository {
    pool: PgPool,
}

impl ToDoListRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_to_do_list(&self, to_do_list: toDoList) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO to_do_list (task, status, username) VALUES ($1, $2, $3)",
            to_do_list.task,
            to_do_list.status,
            to_do_list.username
        )
        .execute(&self.pool)
        .await?;
    
        Ok(())
    }

    pub async fn get_all_to_do_list(&self) -> Result<Vec<toDoList>, sqlx::Error> {
        let to_do_lists = sqlx::query_as!(
            toDoList,
            "SELECT task, status, username FROM to_do_list"
        )
        .fetch_all(&self.pool)
        .await?;
    
        Ok(to_do_lists)
    }

    pub async fn get_to_do_list_by_username(&self, username: &str) -> Result<Vec<toDoList>, sqlx::Error> {
        let to_do_lists = sqlx::query_as!(
            toDoList,
            "SELECT task, status, username FROM to_do_list WHERE username = $1",
            username
        )
        .fetch_all(&self.pool)
        .await?;
    
        Ok(to_do_lists)
    }
    pub async fn update_to_do_list(&self, to_do_list: toDoList) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE to_do_list SET task = $1, status = $2 WHERE username = $3",
            to_do_list.task,
            to_do_list.status,
            to_do_list.username
        )
        .execute(&self.pool)
        .await?;
    
        Ok(())
    }
}

