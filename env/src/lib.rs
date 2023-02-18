
pub fn load_env_variables() -> Result<(), String> {
    let env_location = std::env::var("ENV_LOCATION").unwrap_or("local".into());
    match env_location.as_str() {
        "local" => {}
        "dotenv" => {
            let dotenv_path = std::env::var("DOTENV_PATH");
            let result = match dotenv_path {
                Ok(dotenv_path) => {
                    dotenv::from_filename(dotenv_path)
                }
                Err(_) => {
                    dotenv::dotenv()
                }
            };
            match result {
                Ok(_) => {}
                Err(error) => {
                    return Err(error.to_string())
                }
            }
        }
        _ => {
            return Err(format!("Cannot load '{}' environment", env_location))
        }
    }
    Ok(())
}

/**
 * The testing threads must be 1 to prevent race condition 
 * when accessing environment variables
 */
#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn test_load_local_env() {
        let test_env_1 = "local environment 1";
        let test_env_2 = "local environment 2";

        env::set_var("ENV_1", test_env_1);
        env::set_var("ENV_2", test_env_2);

        load_env_variables().unwrap();

        let env1 = env::var("ENV_1").unwrap();
        assert_eq!(env1, test_env_1);

        let env2 = env::var("ENV_2").unwrap();
        assert_eq!(env2, test_env_2);
    }

    #[test]
    fn test_load_dotenv() {
        let test_env_1 = "dotenv environment 1";
        let test_env_2 = "dotenv environment 2";

        env::set_var("ENV_LOCATION", "dotenv");
        env::set_var("DOTENV_PATH", ".env.example");

        load_env_variables().unwrap();

        let env1 = env::var("ENV_1").unwrap();
        assert_eq!(env1, test_env_1);

        let env2 = env::var("ENV_2").unwrap();
        assert_eq!(env2, test_env_2);
    }
}