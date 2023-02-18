use tonic::{Request, Status, Response};
use user_model::{model::UserModel, error::Error};
pub use pb::user_service_server::UserServiceServer;

mod pb {
    tonic::include_proto!("user");
}

type Result<T> = std::result::Result<T, Status>;

pub struct UserService {
    user_model: UserModel
}

#[tonic::async_trait]
impl pb::user_service_server::UserService for UserService {
    async fn get_user_by_username(&self, request: Request<pb::UsernameRequest>) -> Result<Response<pb::OneUserResponse>> {
        let username = request.into_inner().username;
        let found_user = self.user_model.get_user_by_username(&username).await;

        match found_user {
            Ok(found_user) => {
                match found_user {
                    Some(user) => {
                        let reply = pb::OneUserResponse {
                            id: user._id.to_string(),
                            username: user.username,
                            password: user.password,
                            fullname: user.fullname
                        };
                        Ok(Response::new(reply))
                    },
                    None => Err(Status::not_found(format!("username '{}' is not found", &username)))
                }
            },
            Err(e) => match e {
                Error::MongoError(e) => {
                    Err(Status::internal(e.to_string()))
                }
            }
        }
    }
}

impl UserService {
    pub fn new(user_model: UserModel) -> Self {
        Self { user_model: user_model }
    }
}
