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

#[derive(Serialize, Deserialize, Debug,Default)]
pub struct TextOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_characters: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_html_tags: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug,Default)]
pub struct HighlightsOptions {
    pub num_sentences: Option<u32>,
    pub highlights_per_url: Option<u32>,
    pub query: Option<String>,
}

#[derive(Serialize, Deserialize, Debug,Default)]
pub struct ContentsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlights: Option<HighlightsOptions>,
}


#[derive(Serialize, Deserialize, Debug,Default)]
 pub struct CommonRequestOptions {
     num_results: Option<u32>,
     include_domains: Option<Vec<String>>,
     exclude_domains: Option<Vec<String>>,
     start_crawl_date: Option<String>,
     end_crawl_date: Option<String>,
     start_published_date: Option<String>,
     end_published_date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SearchParams {
    query: String,
    use_autoprompt: Option<bool>,
    #[serde(flatten)]
    common: CommonRequestOptions,
    #[serde(skip_serializing_if = "Option::is_none")]
    contents: Option<ContentsRequest>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    search_type: Option<String>,
}


// fix the text 


impl SearchParams {
    pub fn new(query: &str) -> Self {
        SearchParams {
            query: query.to_string(),
            ..Default::default()
        }
    }

    pub fn use_autoprompt(mut self, value: bool) -> Self {
        self.use_autoprompt = Some(value);
        self
    }

    pub fn search_type(mut self, search_type: &str) -> Self {
        self.search_type = Some(search_type.to_string());
        self
    }

    pub fn num_results(mut self, value: u32) -> Self {
        self.common.num_results = Some(value);
        self
    }

    pub fn include_domains(mut self, domains: Vec<String>) -> Self {
        self.common.include_domains = Some(domains);
        self
    }

    pub fn exclude_domains(mut self, domains: Vec<String>) -> Self {
        self.common.exclude_domains = Some(domains);
        self
    }

    pub fn start_crawl_date(mut self, date: &str) -> Self {
        self.common.start_crawl_date = Some(date.to_string());
        self
    }

    pub fn end_crawl_date(mut self, date: &str) -> Self {
        self.common.end_crawl_date = Some(date.to_string());
        self
    }

    pub fn start_published_date(mut self, date: &str) -> Self {
        self.common.start_published_date = Some(date.to_string());
        self
    }

    pub fn end_published_date(mut self, date: &str) -> Self {
        self.common.end_published_date = Some(date.to_string());
        self
    }

    pub fn text(mut self, max_characters: Option<u32>, include_html_tags: Option<bool>) -> Self {
        let text = TextOptions {
            max_characters,
            include_html_tags,
        };
        self.contents.get_or_insert_with(Default::default).text = Some(text);
        self
    }

    pub fn highlights(mut self, num_sentences: Option<u32>, highlights_per_url: Option<u32>, query: Option<&str>) -> Self {
        let query_str = query.map(|q| q.to_string());
        let highlights = HighlightsOptions {
            num_sentences,
            highlights_per_url,
            query: query_str,
        };
        self.contents.get_or_insert_with(Default::default).highlights = Some(highlights);
        self
    }

  
}

// make each of these option::is_none
// findsimilar  
#[derive( Serialize,Deserialize, Debug,Default)]
pub struct FindSimilarParams {
     url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
     exclude_source_domain: Option<bool>,
    #[serde(flatten)]
     common: CommonRequestOptions,
    #[serde(skip_serializing_if = "Option::is_none")]
    contents: Option<ContentsRequest>,


}

impl FindSimilarParams {
    pub fn new(url: &str) -> Self {
        FindSimilarParams {
            url: url.to_string(),
            ..Default::default()
        }
    }

    pub fn exclude_source_domain(mut self, value: bool) -> Self {
        self.exclude_source_domain = Some(value);
        self
    }

    pub fn num_results(mut self, value: u32) -> Self {
        self.common.num_results = Some(value);
        self
    }

    pub fn include_domains(mut self, domains: Vec<String>) -> Self {
        self.common.include_domains = Some(domains);
        self
    }

    pub fn exclude_domains(mut self, domains: Vec<String>) -> Self {
        self.common.exclude_domains = Some(domains);
        self
    }

    pub fn start_crawl_date(mut self, date: &str) -> Self {
        self.common.start_crawl_date = Some(date.to_string());
        self
    }

    pub fn end_crawl_date(mut self, date: &str) -> Self {
        self.common.end_crawl_date = Some(date.to_string());
        self
    }

    pub fn start_published_date(mut self, date: &str) -> Self {
        self.common.start_published_date = Some(date.to_string());
        self
    }

    pub fn end_published_date(mut self, date: &str) -> Self {
        self.common.end_published_date = Some(date.to_string());
        self
    }


    pub fn text(mut self, max_characters: Option<u32>, include_html_tags: Option<bool>) -> Self {
        let text = TextOptions {
            max_characters,
            include_html_tags,
        };
        self.contents.get_or_insert_with(Default::default).text = Some(text);
        self
    }

    pub fn highlights(mut self, num_sentences: Option<u32>, highlights_per_url: Option<u32>, query: Option<&str>) -> Self {
        let query_str = query.map(|q| q.to_string());
        let highlights = HighlightsOptions {
            num_sentences,
            highlights_per_url,
            query: query_str,
        };
        self.contents.get_or_insert_with(Default::default).highlights = Some(highlights);
        self
    }

}



// Answers 

// Answer request parameters
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AnswerParams {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<serde_json::Value>,
}

impl AnswerParams {
    pub fn new(query: &str) -> Self {
        AnswerParams {
            query: query.to_string(),
            ..Default::default()
        }
    }

    pub fn stream(mut self, value: bool) -> Self {
        self.stream = Some(value);
        self
    }

    pub fn text(mut self, value: bool) -> Self {
        self.text = Some(value);
        self
    }

    pub fn system_prompt(mut self, prompt: &str) -> Self {
        self.system_prompt = Some(prompt.to_string());
        self
    }

    pub fn model(mut self, model: &str) -> Self {
        self.model = Some(model.to_string());
        self
    }

    pub fn output_schema(mut self, schema: serde_json::Value) -> Self {
        self.output_schema = Some(schema);
        self
    }
}

// Answer citation result
#[derive(Serialize, Deserialize, Debug)]
pub struct AnswerCitation {
    pub id: String,
    pub url: String,
    pub title: Option<String>,
    pub author: Option<String>,
    pub published_date: Option<String>,
    pub text: Option<String>,
    pub image: Option<String>,
    pub favicon: Option<String>,
}

// Cost breakdown structures
#[derive(Serialize, Deserialize, Debug)]
pub struct CostBreakdown {
    pub search: Option<f64>,
    pub contents: Option<f64>,
    pub breakdown: Option<CostDetails>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CostDetails {
    pub keyword_search: Option<f64>,
    pub neural_search: Option<f64>,
    pub content_text: Option<f64>,
    pub content_highlight: Option<f64>,
    pub content_summary: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PerRequestPrices {
    pub neural_search_1_25_results: Option<f64>,
    pub neural_search_26_100_results: Option<f64>,
    pub neural_search_100_plus_results: Option<f64>,
    pub keyword_search_1_100_results: Option<f64>,
    pub keyword_search_100_plus_results: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PerPagePrices {
    pub content_text: Option<f64>,
    pub content_highlight: Option<f64>,
    pub content_summary: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CostDollars {
    pub total: f64,
    pub break_down: Option<Vec<CostBreakdown>>,
    pub per_request_prices: Option<PerRequestPrices>,
    pub per_page_prices: Option<PerPagePrices>,
}

// Answer response
#[derive(Serialize, Deserialize, Debug)]
pub struct AnswerResponse {
    pub answer: serde_json::Value, // Can be string or structured object
    pub citations: Vec<AnswerCitation>,
    pub cost_dollars: Option<CostDollars>,
}

// Streaming chunk for future streaming implementation
#[derive(Debug)]
pub struct StreamChunk {
    pub content: Option<String>,
    pub citations: Option<Vec<AnswerCitation>>,
}

impl StreamChunk {
    pub fn has_data(&self) -> bool {
        self.content.is_some() || self.citations.is_some()
    }
}



// find contents 
#[derive(Serialize, Deserialize, Debug,Default)]
pub struct ContentsParams {
    pub ids: Vec<String>,
    pub text: Option<TextOptions>,
    pub highlights: Option<HighlightsOptions>,

}

impl ContentsParams {
    pub fn new(ids: Vec<String>) -> Self {
        ContentsParams {
            ids,
            ..Default::default()
        }
    }

    pub fn text(mut self, max_characters: Option<u32>, include_html_tags: Option<bool>) -> Self {
        let text = TextOptions {
            max_characters,
            include_html_tags,
        };
        self.text = Some(text);
        self
    }

    
    pub fn highlights(mut self, num_sentences: Option<u32>, highlights_per_url: Option<u32>, query: Option<&str>) -> Self {
        let query_str = query.map(|q| q.to_string());
        let highlights = HighlightsOptions {
            num_sentences,
            highlights_per_url,
            query: query_str,
        };
        self.highlights = Some(highlights);
        self
    }
}


//use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseResult {
    pub title: Option<String>,
    pub url: String,
    pub published_date: Option<String>,
    pub author: Option<String>,
    pub id: String,
    pub score: Option<f64>,
    pub text: Option<String>,
    pub highlights: Option<Vec<String>>,
    pub highlight_scores: Option<Vec<f64>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResponse {
    pub results: Vec<ResponseResult>,
    pub autoprompt_string: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentsResponse {
    pub results: Vec<ResponseResult>,
    pub autoprompt_string: Option<String>,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct FindSimilarResponse {
    pub results: Vec<ResponseResult>,
    pub autoprompt_string: Option<String>,

}




pub struct ExaApiClient {
    pub base_url: String,
    pub api_key: String,
    pub client: Client,
}

impl ExaApiClient {
    pub fn new(api_key:  &str) -> Self {
        ExaApiClient {
            base_url: "https://api.exa.ai".to_string(),
            api_key: api_key.to_string(),
            client: Client::new(),
        }
    }

    pub async fn search(&self, params: SearchParams) -> Result<SearchResponse, ExaApiError> {
        let url = format!("{}/search", self.base_url);
        let response = self.client.post(&url)
            .json(&params)
            .header("x-api-key", &self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let search_response = response.json::<SearchResponse>().await?;
            Ok(search_response)
        } else {
            let error_msg = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(ExaApiError::ApiError(error_msg))
        }
    }

    pub async fn find_similar(&self, params: FindSimilarParams) -> Result<FindSimilarResponse, ExaApiError> {
        let url = format!("{}/findSimilar", self.base_url);
        let response = self.client.post(&url)
            .json(&params)
            .header("x-api-key", &self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let find_similar_response = response.json::<FindSimilarResponse>().await?;
            Ok(find_similar_response)
        } else {
            let error_msg: String = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(ExaApiError::ApiError(error_msg))
        }
    }

   
    pub async fn contents(&self, params: ContentsParams) -> Result<ContentsResponse, ExaApiError> {
        let url = format!("{}/contents", self.base_url);
        let response = self.client.post(&url)
            .json(&params)
            .header("x-api-key", &self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let contents_response = response.json::<ContentsResponse>().await?;
            Ok(contents_response)
        } else {
            let error_msg = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(ExaApiError::ApiError(error_msg))
        }
    }

    pub async fn answer(&self, params: AnswerParams) -> Result<AnswerResponse, ExaApiError> {
        let url = format!("{}/answer", self.base_url);
        let response = self.client.post(&url)
            .json(&params)
            .header("x-api-key", &self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let answer_response = response.json::<AnswerResponse>().await?;
            Ok(answer_response)
        } else {
            let error_msg = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(ExaApiError::ApiError(error_msg))
        }
    }


}



