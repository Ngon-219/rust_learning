use sqlx::PgPool;
use crate::model::toDoList;
use uuid::Uuid;

pub struct ToDoListRepository {
    pool: PgPool,
}

impl ToDoListRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_to_do_list(&self, to_do_list: toDoList) -> Result<(), sqlx::Error> {
        let task_id = Uuid::new_v4();
        let task_id_str = task_id.to_string();
        sqlx::query!(
            "INSERT INTO to_do_list (task_id, task, status, username) VALUES ($1, $2, $3, $4)",
            task_id_str,
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
            "SELECT task_id, task, status, username FROM to_do_list"
        )
        .fetch_all(&self.pool)
        .await?;
    
        Ok(to_do_lists)
    }

    pub async fn update_to_do_list(&self, to_do_list: toDoList) -> Result<(), sqlx::Error> {
        println!("{:#?}", to_do_list);
    
        if to_do_list.task_id.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }
    
        let task_id = to_do_list.task_id.unwrap();
    
        sqlx::query!(
            "UPDATE to_do_list SET task = $1, status = $2, username = $3 WHERE task_id = $4",
            to_do_list.task,
            to_do_list.status,
            to_do_list.username,
            task_id
        )
        .execute(&self.pool)
        .await?;
    
        Ok(())
    }
    
    pub async fn delete_to_do_list(&self, task_id: &str) -> Result<(), sqlx::Error> {
        let check_to_do_list = sqlx::query!(
            "SELECT task_id FROM to_do_list WHERE task_id = $1",
            task_id
        )  
        .fetch_optional(&self.pool)
        .await?;

        if check_to_do_list.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        sqlx::query!(
            "DELETE FROM to_do_list WHERE task_id = $1",
            task_id
        )
        .execute(&self.pool)
        .await?;
    
        Ok(())
    }

    pub async fn update_to_do_list_status(&self, task_id: &str) -> Result<(), sqlx::Error> {
        let result = sqlx::query!(
            "SELECT status FROM to_do_list WHERE task_id = $1",
            task_id
        )
        .fetch_optional(&self.pool)
        .await?;

        if result.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        let status = result.unwrap().status;
        
        let new_status = !status;

        sqlx::query!(
            "UPDATE to_do_list SET status = $1 WHERE task_id = $2",
            new_status,
            task_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
    
    pub async fn get_to_do_list_by_id (&self, task_id: &str) -> Result<toDoList, sqlx::Error> {
        let to_do_list = sqlx::query_as!(
            toDoList,
            "SELECT task_id, task, status, username FROM to_do_list WHERE task_id = $1",
            task_id
        )
        .fetch_one(&self.pool)
        .await?;
    
        Ok(to_do_list)
    }
}
