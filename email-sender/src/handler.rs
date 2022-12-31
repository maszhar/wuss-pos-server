use warp::{Filter, Rejection};

pub struct EmailSenderHandler {}

impl EmailSenderHandler {
    pub fn new() -> Self {
        EmailSenderHandler {}
    }

    pub fn handle(self: &Self) -> impl Filter<Extract = (String,), Error = Rejection> + Copy {
        warp::post().map(|| String::from(""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handler_return_success() {
        let handler = EmailSenderHandler::new();
        let filter = handler.handle();

        let resp = warp::test::request()
            .path("/")
            .method("POST")
            .reply(&filter)
            .await;

        assert_eq!(resp.status(), 200);
        assert_eq!(resp.body(), "");
    }
}
