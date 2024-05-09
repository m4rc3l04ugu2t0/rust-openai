use async_openai::types::{AssistantTools, AssistantToolsRetrieval, CreateAssistantRequest};
use derive_more::{Deref, Display, From};

use crate::Result;

use crate::ais::OaClient;

pub struct CreateConfig {
    pub name: String,
    pub model: String,
}

#[derive(Debug, From, Deref, Display)]
pub struct AsstId(String);

#[derive(Debug, From, Deref, Display)]
pub struct ThreadId(String);

#[derive(Debug, From, Deref, Display)]
pub struct FileId(String);

pub async fn create(oac: &OaClient, config: CreateConfig) -> Result<AsstId> {
    let ao_assts = oac.assistants();

    let asst_obj = ao_assts
        .create(CreateAssistantRequest {
            model: config.model,
            name: Some(config.name),
            tools: Some(vec![AssistantToolsRetrieval::default().into()]),
            ..Default::default()
        })
        .await?;

    Ok(asst_obj.id.into())
}
