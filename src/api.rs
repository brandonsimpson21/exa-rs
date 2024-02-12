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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_characters: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlights: Option<HighlightsOptions>,
}


#[derive(Serialize, Deserialize, Debug)]
 pub struct CommonRequestOptions {
     num_results: Option<u32>,
     include_domains: Option<Vec<String>>,
     exclude_domains: Option<Vec<String>>,
     start_crawl_date: Option<String>,
     end_crawl_date: Option<String>,
     start_published_date: Option<String>,
    pub end_published_date: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct SearchParams {
     query: String,
     use_autoprompt: Option<bool>,
    #[serde(flatten)]
     common: Option<CommonRequestOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contents: Option<ContentsRequest>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    search_type: Option<String>,  // Include search_type as an optional field


}

// fix the text 

#[derive(Default)]
pub struct SearchParamsBuilder {
    query: String,
    use_autoprompt: Option<bool>,
    num_results: Option<u32>,
    include_domains: Option<Vec<String>>,
    exclude_domains: Option<Vec<String>>,
    start_crawl_date: Option<String>,
    end_crawl_date: Option<String>,
    start_published_date: Option<String>,
    end_published_date: Option<String>,
    text: Option<TextOptions>,
    highlights: Option<HighlightsOptions>,
    search_type: Option<String>,  // Add search_type as an optional field
}

impl SearchParamsBuilder {
    pub fn new(query: &str) -> Self {
        SearchParamsBuilder {
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
        self.num_results = Some(value);
        self
    }

    pub fn include_domains(mut self, domains: Vec<String>) -> Self {
        self.include_domains = Some(domains);
        self
    }

    pub fn exclude_domains(mut self, domains: Vec<String>) -> Self {
        self.exclude_domains = Some(domains);
        self
    }

    pub fn start_crawl_date(mut self, date: &str) -> Self {
        self.start_crawl_date = Some(date.to_string());
        self
    }

    pub fn end_crawl_date(mut self, date: &str) -> Self {
        self.end_crawl_date = Some(date.to_string());
        self
    }

    pub fn start_published_date(mut self, date: &str) -> Self {
        self.start_published_date = Some(date.to_string());
        self
    }

    pub fn end_published_date(mut self, date: &str) -> Self {
        self.end_published_date = Some(date.to_string());
        self
    }

    pub fn text(mut self, options: TextOptions) -> Self {
        self.text = Some(options);
        self
    }

    pub fn highlights(mut self, options: HighlightsOptions) -> Self {
        self.highlights = Some(options);
        self
    }

    pub fn build(self) -> SearchParams {
        SearchParams {
            query: self.query,
            use_autoprompt: self.use_autoprompt,
            common: Some(CommonRequestOptions {
                num_results: self.num_results,
                include_domains: self.include_domains,
                exclude_domains: self.exclude_domains,
                start_crawl_date: self.start_crawl_date,
                end_crawl_date: self.end_crawl_date,
                start_published_date: self.start_published_date,
                end_published_date: self.end_published_date,
            }),
            contents: Some(ContentsRequest{text: self.text,
            highlights: self.highlights,}),
            search_type: self.search_type.or(Some("neural".to_string())),  // Default to "neural" if not set

        }
    }
}

// make each of these option::is_none
#[derive(Serialize, Deserialize, Debug)]
pub struct FindSimilarParams {
     url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
     exclude_source_domain: Option<bool>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
     common: Option<CommonRequestOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contents: Option<ContentsRequest>,


}


// find similar 
#[derive(Default)]
pub struct FindSimilarParamsBuilder {
    url: String,
    exclude_source_domain: Option<bool>,
    num_results: Option<u32>,
    include_domains: Option<Vec<String>>,
    exclude_domains: Option<Vec<String>>,
    start_crawl_date: Option<String>,
    end_crawl_date: Option<String>,
    start_published_date: Option<String>,
    end_published_date: Option<String>,
    text: Option<TextOptions>,
    highlights: Option<HighlightsOptions>,
}

impl FindSimilarParamsBuilder {
    pub fn new(url: &str) -> Self {
        FindSimilarParamsBuilder {
            url: url.to_string(),
            ..Default::default()
        }
    }

    pub fn exclude_source_domain(mut self, value: bool) -> Self {
        self.exclude_source_domain = Some(value);
        self
    }

    pub fn num_results(mut self, value: u32) -> Self {
        self.num_results = Some(value);
        self
    }

    pub fn include_domains(mut self, domains: Vec<String>) -> Self {
        self.include_domains = Some(domains);
        self
    }

    pub fn exclude_domains(mut self, domains: Vec<String>) -> Self {
        self.exclude_domains = Some(domains);
        self
    }

    pub fn start_crawl_date(mut self, date: &str) -> Self {
        self.start_crawl_date = Some(date.to_string());
        self
    }

    pub fn end_crawl_date(mut self, date: &str) -> Self {
        self.end_crawl_date = Some(date.to_string());
        self
    }

    pub fn start_published_date(mut self, date: &str) -> Self {
        self.start_published_date = Some(date.to_string());
        self
    }

    pub fn end_published_date(mut self, date: &str) -> Self {
        self.end_published_date = Some(date.to_string());
        self
    }


    pub fn text(mut self, options: TextOptions) -> Self {
        self.text = Some(options);
        self
    }

    pub fn highlights(mut self, options: HighlightsOptions) -> Self {
        self.highlights = Some(options);
        self
    }

    pub fn build(self) -> FindSimilarParams {
        FindSimilarParams {
            url: self.url,
            exclude_source_domain: self.exclude_source_domain,
            common: Some(CommonRequestOptions {
                num_results: self.num_results,
                include_domains: self.include_domains,
                exclude_domains: self.exclude_domains,
                start_crawl_date: self.start_crawl_date,
                end_crawl_date: self.end_crawl_date,
                start_published_date: self.start_published_date,
                end_published_date: self.end_published_date,
            }),
           contents: Some(ContentsRequest{ text: self.text,
            highlights: self.highlights,}),
        }
    }
}



// find contents 
#[derive(Serialize, Deserialize, Debug)]
pub struct ContentsParams {
    pub ids: Vec<String>,
    pub text: Option<TextOptions>,
    pub highlights: Option<HighlightsOptions>,

}

#[derive(Default)]
pub struct ContentsParamsBuilder {
    ids: Vec<String>,
    text: Option<TextOptions>,
    highlights: Option<HighlightsOptions>,
}

impl ContentsParamsBuilder {
    pub fn new(ids: Vec<String>) -> Self {
        ContentsParamsBuilder {
            ids,
            ..Default::default()
        }
    }

    pub fn text(mut self, text_options: TextOptions) -> Self {
        self.text = Some(text_options);
        self
    }

    pub fn highlights(mut self, highlights_options: HighlightsOptions) -> Self {
        self.highlights = Some(highlights_options);
        self
    }

    pub fn build(self) -> ContentsParams {
        ContentsParams {
            ids: self.ids,
            text: self.text,
            highlights: self.highlights,
        }
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


