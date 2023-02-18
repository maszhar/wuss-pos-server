use mongodb::{bson::doc, Collection, Database};

use crate::{error::Result, types::User};

pub struct UserModel {
    collection: Collection<User>,
}

impl UserModel {
    pub fn new(db_instance: Database) -> UserModel {
        UserModel {
            collection: db_instance.collection("users"),
        }
    }

    pub async fn get_user_by_username(&self, username: String) -> Result<Option<User>> {
        Ok(self
            .collection
            .find_one(doc! {"username": username}, None)
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::{Client, Database};

    async fn init_db() -> Database {
        let client = Client::with_uri_str("mongodb://localhost:27017")
            .await
            .unwrap();
        client.database("test_wuss_pos")
    }

    #[tokio::test]
    #[ignore = "need database instance"]
    async fn test_get_user_by_username() {
        let username = "juan_arely";
        let db_instance = init_db().await;
        let model = UserModel::new(db_instance);

        let found_user = model.get_user_by_username(username.into()).await.unwrap();

        assert!(found_user.is_some());
    }
}
