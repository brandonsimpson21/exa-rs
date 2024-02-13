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

}


