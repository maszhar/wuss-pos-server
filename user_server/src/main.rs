mod service;

use env::load_env_variables;
use mongodb::Client;
use service::UserService;
use tonic::transport::Server;
use user_model::model::UserModel;

#[tokio::main]
async fn main() ->Result<(), Box<dyn std::error::Error>> {
    load_env_variables().unwrap();

    // Listening address
    let port = str::parse::<u16>(std::env::var("PORT").unwrap_or("40040".into()).as_str())
        .unwrap_or(40040);
    let addr = format!("[::]:{}", port).parse().unwrap();

    // Database init
    let mongodb_uri = std::env::var("MONGODB_URI").unwrap();
    let client = Client::with_uri_str(mongodb_uri).await.unwrap();
    let db_instance = client.database("accounts");

    // Init service
    let user_model = UserModel::new(db_instance);
    let user_service = UserService::new(user_model);

    // Start gRPC
    Server::builder()
        .add_service(service::UserServiceServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}
