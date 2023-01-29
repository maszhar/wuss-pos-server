use mongodb::Database;

use super::{
    error::{Error, Result},
    model::{Task, TaskModel},
};

pub struct TaskService {
    task_model: TaskModel,
}

impl TaskService {
    pub fn new(db: Database) -> Self {
        let task_model = TaskModel::new(db);

        Self { task_model }
    }

    pub async fn take_task(&self, task_id: &String) -> Result<Task> {
        let task = self.task_model.get_task(&task_id).await?;
        match task {
            Some(mut task) => {
                task.handler = Some("[EMAIL_SENDER]".into());
                let new_task = self.task_model.update_task(&task_id, task).await?;
                Ok(new_task)
            }
            None => Err(Error::NotFound()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use mongodb::Client;
    use super::super::error::Error;

    #[tokio::test]
    async fn test_take_task() {
        let client = Client::with_uri_str(get_test_mongodb_uri().as_str())
            .await
            .unwrap();
        let db = client.database("test_pos");
        let service = TaskService::new(db);

        let task = service
            .take_task(&"999999999999999999999999".into())
            .await
            .unwrap();

        assert!(task.handler.unwrap().contains("[EMAIL_SENDER]"))
    }

    #[tokio::test]
    async fn test_take_invalid_task() {
        let client = Client::with_uri_str(get_test_mongodb_uri().as_str())
            .await
            .unwrap();
        let db = client.database("test_pos");
        let service = TaskService::new(db);

        let task = service
            .take_task(&"000999999999999999999999".into())
            .await;

        let error_valid = match task {
            Err(e) => {
                match e {
                    Error::NotFound() => true,
                    _ => false
                }
            },
            _ => false
        };

        assert_eq!(error_valid, true);
    }

    fn get_test_mongodb_uri() -> String {
        dotenv().ok();

        let uri = std::env::var("TEST_MONGODB_URI").unwrap_or("mongodb://localhost:27017".into());
        uri
    }
}
