use super::{error::Result, Task};
use mongodb::{
    bson::{self, doc, oid::ObjectId},
    Collection, Database,
};
use std::str::FromStr;

pub struct TaskModel {
    coll: Collection<Task>,
}

impl TaskModel {
    pub fn new(db: Database) -> Self {
        let coll = db.collection::<Task>("Task");

        Self { coll }
    }

    pub async fn get_task(&self, task_id: &String) -> Result<Option<Task>> {
        let id = ObjectId::from_str(task_id.as_str())?;
        let task = self.coll.find_one(doc! {"_id": id}, None).await?;

        Ok(task)
    }

    pub async fn update_task(&self, task_id: &String, update: Task) -> Result<Task> {
        let id = ObjectId::from_str(task_id.as_str())?;
        self.coll
            .update_one(
                doc! {"_id": id},
                doc! {
                    "$set": bson::to_bson(&update)?
                },
                None,
            )
            .await?;
        Ok(update)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use mongodb::Client;

    #[tokio::test]
    #[ignore = "need database instance"]
    async fn test_get_task() {
        /*
            SAMPLE DATA
            _id: 999999999999999999999999
            scope: "EMAIL_SENDER"
            command: "SEND_EMAIL"
            arg0: "admin@example.com"
            arg1: "EMAIL_VERIFICATION"
            handler: null
            failed_count: 0
            created_at: 2023-01-29T00:00:00.000+00:00
            handled_at: null
            done_at: null
        */

        let client = Client::with_uri_str(get_test_mongodb_uri().as_str())
            .await
            .unwrap();
        let db = client.database("test_pos");
        let model = TaskModel::new(db);

        let task = model
            .get_task(&"999999999999999999999999".into())
            .await
            .unwrap();
        assert!(task.is_some());

        let task = task.unwrap();
        assert_eq!(task.scope, "EMAIL_SENDER");
        assert_eq!(task.command, "SEND_EMAIL");
        assert_eq!(task.arg1.unwrap(), "EMAIL_VERIFICATION");
    }

    #[tokio::test]
    #[ignore = "need database instance"]
    async fn test_update_task() {
        let client = Client::with_uri_str(get_test_mongodb_uri().as_str())
            .await
            .unwrap();
        let db = client.database("test_pos");
        let model = TaskModel::new(db);

        let mut task = model
            .get_task(&"999999999999999999999999".into())
            .await
            .unwrap()
            .unwrap();

        task.handler = Some("[EMAIL_SENDER]: 000-00000-00000".into());
        let updated_task = model
            .update_task(&"999999999999999999999999".into(), task)
            .await
            .unwrap();
        assert_eq!(
            updated_task.handler.unwrap(),
            "[EMAIL_SENDER]: 000-00000-00000"
        );

        // Try to get task again
        let task = model
            .get_task(&"999999999999999999999999".into())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(task.handler.unwrap(), "[EMAIL_SENDER]: 000-00000-00000");
    }

    fn get_test_mongodb_uri() -> String {
        dotenv().ok();

        let uri = std::env::var("TEST_MONGODB_URI").unwrap_or("mongodb://localhost:27017".into());
        uri
    }
}
