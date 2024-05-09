pub mod assit;
use crate::Result;
use async_openai::{
    config::{self, OpenAIConfig},
    Client,
};

const ENV_OPENAI_KEY: &str = "ENV_OPENAI_KEY";

pub type OaClient = Client<OpenAIConfig>;

pub fn new_oa_client() -> Result<OaClient> {
    if let Ok(api_key) = std::env::var(ENV_OPENAI_KEY) {
        let config = OpenAIConfig::new().with_api_key(api_key);
        Ok(Client::with_config(config))
    } else {
        println!("No {ENV_OPENAI_KEY} env variable. Please set it.");
        Err("No openai api key in env".into())
    }
}
