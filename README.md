# Exa-rs

![Crates.io Version](https://img.shields.io/crates/v/exa_api_client)
![Crates.io Total Downloads](https://img.shields.io/crates/d/exa_api_client)
![Crates.io License](https://img.shields.io/crates/l/exa_api_client)



A rust client for the [Exa](https://exa.ai/search) / metaphor systems search API. 


It was made using the following the API documentation https://docs.exa.ai/reference/contents


## Installation 

### Add exa_api_client to your Cargo.toml file 

```
[dependencies]
exa_api_client = "0.1.3" 
```
### Or use cargo add  

```
cargo add exa_api_client
```

### To use you need an api key from https://docs.exa.ai/reference/getting-an-api-key 

### Initialization
Start by creating an instance of the ExaApiClient with your API key:
 
 ```
use exa_api_client::{ExaApiClient,SearchParamsBuilder,FindSimilarParamsBuilder,ContentsParamsBuilder,TextOptions,HighlightsOptions};

#[tokio::main]
async fn main() {
    let api_key = "your_exa_api_key".to_string();
    let client: ExaApiClient = ExaApiClient::new(api_key);
}
 ```

## Builders Overview

The `ExaApiClient` utilizes builder patterns for constructing request parameters for various API endpoints. Here is an overview of the available builders:

| Function            | Builder                     | Description                                                   |
|---------------------|-----------------------------|---------------------------------------------------------------|
| Search              | `SearchParamsBuilder`       | Used to construct parameters for the search API endpoint.     |
| Find Similar        | `FindSimilarParamsBuilder`  | Used to construct parameters for the find similar API endpoint.|
| Get Contents        | `ContentsParamsBuilder`     | Used to construct parameters for the get contents API endpoint.|



## Using search 

```
#[tokio::main]
async fn main() {
    let api_key = "".to_string();
    let client: ExaApiClient = ExaApiClient::new(api_key);

//search example
 let search_params = SearchParamsBuilder::new("Mitski singer")
        .use_autoprompt(true)
        .num_results(1)
        .include_domains(vec!["example.com".to_string()])
        .exclude_domains(vec!["excluded.com".to_string()])
        .start_crawl_date("2023-01-01")
        .end_crawl_date("2023-12-31")
        .text(TextOptions { max_characters: Some(1000), include_html_tags: Some(false) }) 
        .highlights(HighlightsOptions { num_sentences: None, highlights_per_url: Some(1), query: Some("highlight query".to_string()) }) 
        .build();


    match client.search(search_params).await {
        Ok(response) => println!("Search Response: {:?}", response),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```

## Find similar 
```
 // Example FindSimilarParams usage
    let find_similar_params = FindSimilarParamsBuilder::new("https://www.taminomusic.com/")
        .exclude_source_domain(true)
        .num_results(3)
        .text((TextOptions { max_characters: None, include_html_tags: Some(false) }))
        .highlights((HighlightsOptions { num_sentences: Some(2), highlights_per_url: Some(1), query: Some("similar content query".to_string()) }))
        .build();

    match client.find_similar(find_similar_params).await {
        Ok(response) => {
            let formatted_response = serde_json::to_string_pretty(&response).unwrap_or_else(|_| "Failed to format response".to_string());
            println!("Find Similar Response: {}", formatted_response);
        },
        Err(e) => eprintln!("Error occurred during find similar: {:?}", e),
    }
```

## Get contents
```
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
```


### Example Search response 

```
Search Response: SearchResponse { results: 
[ResponseResult { title: Some("Mitski Miyawaki"), 
url: "https://mitski.fandom.com/wiki/Mitski_Miyawaki", published_date: None, author: None, id: "wx0YdyyQyjKZpim7m9kcFQ", score: Some(0.19030162692070007), text: Some(" \n\t\n\t \nMitski Miyawaki, known professionally as Mitski, born Mitsuki Laycock, was born on September 27,
 1990 in the Mie prefecture of Japan.  [1]   [2]   Before settling in the United States, Mitski lived in thirteen countries, including (but not limited to) China, Malaysia, Turkey, and the Democratic Republic of Congo. She refers to herself as 
 \"half Japanese, half American but not fully either.\"
 While attending Purchase College's Conservatory of Music to study studio composition, Mitski self-released her first two albums, Lush (2012) and Retired from Sad, New Career in Business (2013). Her senior project at Purchase originally involved the creation of these albums.
  Bury Me at Makeout Creek, her third studio album, was made available in 2014 via Double Double Whammy after she graduated. Her critically acclaimed albums Puberty 2 (2016),
  Be the Cowboy (2018), and Laurel Hell (2022), the latter of which reached the top ten in multiple countries, were then published when she signed with De"), 
  highlights: Some([" She then joined the prog-rock band Voice Coils as a vocalist while working on  Bury Me at Makeout Creek , released on Double Double Whammy as her first album on any record label.  She signed to the label Dead Oceans in 2016, and has gone on to release  Puberty 2 
   and Be the Cowboy, both to critical acclaim. In 2019 Mitski announced a hiatus from music and went on to explain on Twitter:  \"Y'all, I'm not quitting music! Me?"]),
   highlight_scores: None }], autoprompt_string: None }

```




## Contributions

We welcome any and all feedback, contributions, and suggestions! Feel free to raise an issue on the <a href="https://github.com/maishathasin/exa-rs/issues">Repo's GitHub Issues</a> or submit a Pull Request. 

 
