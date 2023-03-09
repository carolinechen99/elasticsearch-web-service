use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use elasticsearch::http::transport::Transport;
use elasticsearch::{Elasticsearch, SearchParts};
use serde_json::json;


#[get("/{name}")]
async fn hello(web::Path(name): web::Path<String>) -> impl Responder {
    format!("Hello, {}!", name)
}

#[get("/search")]
async fn search(query: web::Query<String>) -> impl Responder {
    let transport = Transport::single_node("http://localhost:9200").unwrap();
    let client = Elasticsearch::new(transport);

    let response = client
        .search(SearchParts::Index(&["my-index"]))
        .body(json!({
            "query": {
                "match": {
                    "name": query.into_inner()
                }
            }
        }))
        .send()
        .await
        .unwrap();

    let results = response
        .json::<serde_json::Value>()
        .await
        .unwrap()["hits"]["hits"]
        .as_array()
        .unwrap();

    let names: Vec<String> = results
        .iter()
        .map(|result| result["_source"]["name"].as_str().unwrap().to_string())
        .collect();

    HttpResponse::Ok().json(names)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(search)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
