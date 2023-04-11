use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use elasticsearch::http::transport::Transport;
use elasticsearch::{Elasticsearch, SearchParts};
use serde_json::json;
use serde::Deserialize;

#[derive(Deserialize)]
struct SearchQuery {
    query: String,
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello, {}!", name.into_inner())
}

async fn search_handler(query: web::Query<SearchQuery>) -> impl Responder {
    let transport = Transport::single_node("http://localhost:9200").unwrap();
    let client = Elasticsearch::new(transport);

    let response = client
        .search(SearchParts::Index(&["my-index"]))
        .body(json!({
            "query": {
                "match": {
                    "name": query.query
                }
            }
        }))
        .send()
        .await;

    if let Ok(response) = response {
        let results = {
            let response_body = response.text().await.unwrap();
            let json_response = serde_json::from_str::<serde_json::Value>(&response_body).unwrap();
            json_response["hits"]["hits"].as_array().unwrap().to_owned()
        };

        let names = results
            .iter()
            .map(|result| result["_source"]["name"].as_str().unwrap().to_string())
            .collect::<Vec<_>>();

        HttpResponse::Ok().json(names)
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/search", web::get().to(search_handler))
            .service(hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
