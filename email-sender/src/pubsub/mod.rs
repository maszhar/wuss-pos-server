use serde::Deserialize;

#[derive(Deserialize)]
pub struct PubSubMessage {
    data: String,
}

#[derive(Deserialize)]
pub struct PubSubData {
    message: PubSubMessage,
}

pub fn handle_pubsub(data: PubSubData) -> String {
    let data = data.message.data;

    let decoded_data = base64::decode(data.as_str());
    let message = match decoded_data {
        Ok(s) => match String::from_utf8(s) {
            Ok(s) => s,
            Err(_) => data,
        },
        Err(_) => data,
    };
    println!("{}", message);
    
    "".into()
}