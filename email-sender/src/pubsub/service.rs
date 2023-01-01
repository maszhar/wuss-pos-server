use serde::Deserialize;

#[derive(Deserialize)]
pub struct Command {
    command: String,
    arg: String,
}

#[derive(Debug)]
pub enum DecodeError {
    JsonError(serde_json::Error),
    Base64Error(base64::DecodeError),
    StringError(std::string::FromUtf8Error)
}

#[derive(Deserialize)]
struct PubsubData {
    message: PubsubMessage
}

#[derive(Deserialize)]
struct PubsubMessage {
    data: String
}

pub struct PubsubService {}

impl PubsubService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn decode_message(self: &Self, payload: &String) -> Result<Command, DecodeError> {
        let pubsub_data: serde_json::Result<PubsubData> = serde_json::from_str(payload.as_str());
        match pubsub_data{
            Ok(pubsub_data) => {
                match base64::decode(pubsub_data.message.data) {
                    Ok(data) => {
                        match String::from_utf8(data) {
                            Ok(data) => {
                                let command: serde_json::Result<Command> = serde_json::from_str(data.as_str());
                                match command {
                                    Ok(command) => Ok(command),
                                    Err(e) => Err(DecodeError::JsonError(e))
                                }
                            },
                            Err(e) => Err(DecodeError::StringError(e))
                        }
                    },
                    Err(e) => Err(DecodeError::Base64Error(e))
                }
            },
            Err(e) => Err(DecodeError::JsonError(e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_message_with_invalid_payload() {
        let service = PubsubService::new();
        let payload = "{\"data\": \"Wrong payload\"}".to_string();

        let result = service.decode_message(&payload);

        let test_result = match result {
            Err(DecodeError::JsonError(_)) => true,
            _ => false
        };
        assert_eq!(test_result, true);
    }

    #[test]
    fn test_decode_message_with_invalid_data() {
        let service = PubsubService::new();
        let payload = "{\"message\": {\"data\": \"Wrong payload\"}}".to_string();

        let result = service.decode_message(&payload);

        let test_result = match result {
            Err(DecodeError::Base64Error(_)) => true,
            _ => false
        };
        assert_eq!(test_result, true);
    }

    #[test]
    fn test_decode_message_with_invalid_command() {
        let service = PubsubService::new();
        let payload = "{\"message\": {\"data\": \"eyJ0ZXN0IjogImludmFsaWQifQo=\"}}".to_string();

        let result = service.decode_message(&payload);

        let test_result = match result {
            Err(DecodeError::JsonError(_)) => true,
            _ => false
        };
        assert_eq!(test_result, true);
    }

    #[test]
    fn test_decode_message_with_valid_payload() {
        let service = PubsubService::new();
        let payload = "{\"message\": {\"data\": \"eyJjb21tYW5kIjogIlNFTkRfRU1BSUwiLCAiYXJnIjogInJlZ2lzdHJhdGlvbiJ9Cg==\"}}".to_string();

        let result = service.decode_message(&payload).unwrap();

        assert_eq!(result.command, "SEND_EMAIL");
        assert_eq!(result.arg, "registration");
    }
}