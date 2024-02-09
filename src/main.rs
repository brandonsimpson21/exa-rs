mod api;

use crate::api::{ExaApiClient,SearchParamsBuilder,CommonRequestOptions,FindSimilarParamsBuilder,ContentsRequest,ContentsParamsBuilder,TextOptions,HighlightsOptions};


#[tokio::main]
async fn main() {
    let api_key = "b1d8f567-ab4e-44cc-b9d1-f56ca32a14ae".to_string();
    let client = ExaApiClient::new(api_key);
/* 
    let contents_params = ContentsParamsBuilder::new(vec!["kOYHjR-2wEIOZc9Nv4bUHQ".to_string()])
        .text(TextOptions {
            max_characters: Some(100),  
            include_html_tags: Some(false),  
        })
        .highlights(HighlightsOptions {
            num_sentences: Some(1),  
            highlights_per_url: Some(1), 
            query: Some("specific query".to_string()), 
        })
        .build();

    match client.contents(contents_params).await {
        Ok(response) => {
            let parsed_json: serde_json::Value = serde_json::from_str(&response)
                .expect("Failed to parse JSON");
            let pretty_json = serde_json::to_string_pretty(&parsed_json)
                .expect("Failed to convert to pretty JSON");
            println!("Contents Response: {}", pretty_json);
        },
        Err(e) => eprintln!("Error: {:?}", e),
    }

    // Example SearchParams usage
    let search_params = SearchParamsBuilder::new("Mitski singer")
        //.use_autoprompt(true)
        .num_results(1)
        //.include_domains(vec!["example.com".to_string()])
        //.exclude_domains(vec!["excluded.com".to_string()])
        //.start_crawl_date("2023-01-01")
        //.end_crawl_date("2023-12-31")
        .text(TextOptions { max_characters: Some(100), include_html_tags: Some(false) }) 
        //.highlights(HighlightsOptions { num_sentences: Some(1), highlights_per_url: Some(1), query: Some("highlight query".to_string()) }) 
        .build();


    match client.search(search_params).await {
        Ok(response) => println!("Search Response: {:?}", response),
        Err(e) => eprintln!("Error: {:?}", e),
    }
    */

    // Example FindSimilarParams usage
    let find_similar_params = FindSimilarParamsBuilder::new("https://www.taminomusic.com/")
        .exclude_source_domain(true)
        .num_results(3)
        //.include_domains(vec!["example.com".to_string()])
        //.exclude_domains(vec!["excluded.com".to_string()])
        //.start_crawl_date("2023-01-01")
        //.end_crawl_date("2023-12-31")
        .text((TextOptions { max_characters: Some(100), include_html_tags: Some(false) }))
        .highlights((HighlightsOptions { num_sentences: Some(2), highlights_per_url: Some(1), query: Some("similar content query".to_string()) }))
        .build();

    match client.find_similar(find_similar_params).await {
        Ok(response) => {
            let formatted_response = serde_json::to_string_pretty(&response).unwrap_or_else(|_| "Failed to format response".to_string());
            println!("Find Similar Response: {}", formatted_response);
        },
        Err(e) => eprintln!("Error occurred during find similar: {:?}", e),
    
    }
}
