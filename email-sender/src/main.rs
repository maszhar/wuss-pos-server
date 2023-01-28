mod pubsub;
mod handler;
mod task;
 
// use pubsub::handle_pubsub;
use warp::Filter;

#[tokio::main]
async fn main() {
    // let handler = warp::post()
    //     .and(warp::body::json())
    //     .map(handle_pubsub);

    // let port = get_port();
    // warp::serve(handler).run(([0; 16], port)).await  
}

fn get_port() -> u16 {
    match std::env::var("PORT") {
        Ok(p) => match p.parse::<u16>() {
            Ok(port) => port,
            Err(_) => 3000,
        },
        Err(_) => 3000,
    }
}