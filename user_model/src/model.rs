use mongodb::{Database, Collection, bson::doc};

use crate::types::User;

pub struct UserModel {
    collection: Collection<User>
}

impl UserModel {
    pub fn new(db_instance: Database) -> UserModel {
        UserModel{
            collection: db_instance.collection("users")
        }
    }

    pub async fn get_user_by_username(&self, username: String) -> Option<User> {
        self.collection.find_one(
            doc!{"username": username},
            None
        ).await.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use mongodb::{Database, Client};
    use super::*;

    async fn init_db() -> Database {
        let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
        client.database("test_wuss_pos")
    }

    #[tokio::test]
    #[ignore = "need database instance"]
    async fn test_get_user_by_username() {
        let username = "juan_arely";
        let db_instance = init_db().await;
        let model = UserModel::new(db_instance);

        let found_user = model.get_user_by_username(username.into()).await;
        
        assert!(found_user.is_some());
    }
}