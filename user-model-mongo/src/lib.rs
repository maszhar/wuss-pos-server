
pub mod types;
pub mod model;

use model::UserModel;
use mongodb::Database;
pub use types::User;


static mut MODEL_INSTANCE: Option<UserModel> = None;

#[no_mangle]
pub unsafe fn init(db_instance: Database) {
    MODEL_INSTANCE = Some(UserModel::new(db_instance));
}

#[no_mangle]
pub async unsafe fn get_user_by_username(username: String) -> Option<User> {
    let instance = MODEL_INSTANCE.clone();
    instance.unwrap().get_user_by_username(username).await
}

#[no_mangle]
pub unsafe fn destroy() {
    MODEL_INSTANCE = None;
}