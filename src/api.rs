use reqwest::{Client, Error as ReqwestError};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use serde_json::json;



//implement keyword search
#[derive(Debug, Error)]
pub enum ExaApiError {
    #[error("network error: {0}")]
    NetworkError(#[from] ReqwestError),
    #[error("API error: {0}")]
    ApiError(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextOptions {
    pub max_characters: Option<u32>,
    pub include_html_tags: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HighlightsOptions {
    pub num_sentences: Option<u32>,
    pub highlights_per_url: Option<u32>,
    pub query: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentsRequest {
    pub text: Option<TextOptions>,
    pub highlights: Option<HighlightsOptions>,
}


#[derive(Serialize, Deserialize, Debug)]
 pub struct CommonRequestOptions {
    pub num_results: Option<u32>,
    pub include_domains: Option<Vec<String>>,
    pub exclude_domains: Option<Vec<String>>,
    pub start_crawl_date: Option<String>,
    pub end_crawl_date: Option<String>,
    pub start_published_date: Option<String>,
    pub end_published_date: Option<String>,
}

//right now search is only neural 
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchParams {
    pub query: String,
    pub use_autoprompt: Option<bool>,
    #[serde(flatten)]
    pub common: Option<CommonRequestOptions>,
    pub contents: Option<ContentsRequest>,  

}

#[derive(Serialize, Deserialize, Debug)]
pub struct FindSimilarParams {
    pub url: String,
    pub exclude_source_domain: Option<bool>,
    #[serde(flatten)]
    pub common: Option<CommonRequestOptions>,
    pub contents: Option<ContentsRequest>, 

}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentsParams {
    pub ids: Vec<String>,
    pub text: Option<TextOptions>,
    pub highlights: Option<HighlightsOptions>,

}

pub struct ExaApiClient {
    pub base_url: String,
    pub api_key: String,
    pub client: Client,
}

impl ExaApiClient {
    pub fn new(api_key: String) -> Self {
        ExaApiClient {
            base_url: "https://api.exa.ai".to_string(),
            api_key,
            client: Client::new(),
        }
    }

    pub async fn search(&self, params: SearchParams) -> Result<String, ExaApiError> {
        let url = format!("{}/search", self.base_url);
        let response = self.client.post(&url)
            .json(&params)
            .header("x-api-key", &self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let json_string = response.text().await.map_err(|e| ExaApiError::NetworkError(e))?;
            Ok(json_string)
        } else {
            let error_msg = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(ExaApiError::ApiError(error_msg))
        }
    }

    pub async fn find_similar(&self, params: FindSimilarParams) -> Result<String, ExaApiError> {
        let url = format!("{}/findSimilar", self.base_url);
        let response = self.client.post(&url)
            .json(&params)
            .header("x-api-key", &self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let json_string = response.text().await.map_err(|e| ExaApiError::NetworkError(e))?;
            Ok(json_string)
        } else {
            let error_msg: String = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(ExaApiError::ApiError(error_msg))
        }
    }

    pub async fn contents(&self, params: ContentsParams) -> Result<String, ExaApiError> {
        let url = format!("{}/contents", self.base_url);
        let response = self.client.post(&url)
            .json(&params)
            .header("x-api-key", &self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let json_string = response.text().await.map_err(|e| ExaApiError::NetworkError(e))?;
            Ok(json_string)
        } else {
            let error_msg = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(ExaApiError::ApiError(error_msg))
        }
    }

}


