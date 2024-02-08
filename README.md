# Exa-rs

![Crates.io Version](https://img.shields.io/crates/v/exa_api_client)
![Crates.io Total Downloads](https://img.shields.io/crates/d/exa_api_client)
![Crates.io License](https://img.shields.io/crates/l/exa_api_client)



A rust client for the [Exa](https://exa.ai/search) / metaphor systems search. 


It was made using the following the API documentation https://docs.exa.ai/reference/contents


## Installation 

### Add exa_api_client to your Cargo.toml file 

```
[dependencies]
exa_api_client = "0.1.0" 
```

### To use you need an api key from https://docs.exa.ai/reference/getting-an-api-key 
 
 ```
use crate::exa_api_client::{ExaApiClient,SearchParams,CommonRequestOptions,FindSimilarParams,ContentsRequest,ContentsParams,TextOptions,HighlightsOptions};

#[tokio::main]
async fn main() {
    let api_key = "your_exa_api_key".to_string();
    let client: ExaApiClient = ExaApiClient::new(api_key);
}
 ```




## using search, find similar and contents 

```
#[tokio::main]
async fn main() {
    let api_key = "".to_string();
    let client: ExaApiClient = ExaApiClient::new(api_key);


 // Using the search method without ContentsOptions
 let search_params = SearchParams {
    query: "Articles on tamino the singer".to_string(),
    use_autoprompt: Some(true),
    common: Some(CommonRequestOptions {
        num_results: Some(10),
        include_domains: Some(vec!["example.com".to_string()]),
        exclude_domains: None,
        start_crawl_date: None,
        end_crawl_date: None,
        start_published_date: None,
        end_published_date: None,
    }),
    contents: None
};

match client.search(search_params).await {
    Ok(json_string) => println!("{}", json_string),
    Err(e) => eprintln!("Error: {:?}", e),
}
```

### Find similar 
```

// Find similar example using contents object 
let find_similar_params = FindSimilarParams {
    url: "https://www.taminomusic.com/".to_string(),
    exclude_source_domain: Some(true),
    common: None, 
    contents: Some(ContentsRequest {
        text: Some(TextOptions {
            max_characters: Some(1000),
            include_html_tags: Some(false),
        }),
        highlights: Some(HighlightsOptions {
            num_sentences: Some(5),
            highlights_per_url: Some(1),
            query: Some("specific query".to_string()),
        }),
    }),
};

match client.find_similar(find_similar_params).await {
    Ok(json_string) => {
        println!("Find Similar and Contents Response: {}", json_string);
    },
    Err(e) => eprintln!("Error occurred: {:?}", e),
}
```

### Get contents
```
// The get contents api endpoint
let contents_params = ContentsParams {
    ids: vec!["kOYHjR-2wEIOZc9Nv4bUHQ".to_string()], 
    text: Some(TextOptions {
        max_characters: Some(1000),
        include_html_tags: Some(false), 
    }),
    highlights: Some(HighlightsOptions {
        num_sentences: Some(5),
        highlights_per_url: Some(1),
        query: Some("specific query".to_string()), 
    }),
};

match client.contents(contents_params).await {
    Ok(json_string) => {
        println!("Contents Response: {}", json_string);
    },
    Err(e) => eprintln!("Error occurred: {:?}", e),
}

}
```



