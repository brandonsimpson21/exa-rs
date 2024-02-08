mod api;

use crate::api::{ExaApiClient,SearchParams,CommonRequestOptions,FindSimilarParams,ContentsRequest,TextOptions,HighlightsOptions};
#[tokio::main]
async fn main() {
    let api_key = "".to_string();
    let client: ExaApiClient = ExaApiClient::new(api_key);


 /*/ Using the search method with ContentsOptions
 let search_params = SearchParams {
    query: "Articles on tamino the singer".to_string(),
    use_autoprompt: Some(true),
    common: None,
    contents: None
};

match client.search(search_params).await {
    Ok(json_string) => println!("{}", json_string),
    Err(e) => eprintln!("Error: {:?}", e),
}



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
*/

}