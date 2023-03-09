use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use elasticsearch::http::transport::Transport;
use elasticsearch::{Elasticsearch, SearchParts};

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
        .await.unwrap();

    let results = {
        let response_body = response
            .text()
            .await
            .unwrap();
        let json_response = serde_json::from_str::<serde_json::Value>(&response_body)
            .unwrap();
        json_response["hits"]["hits"]
            .as_array()
            .unwrap()
            .to_owned()
    };

    let names = results
        .iter()
        .map(|result| result["_source"]["name"].as_str().unwrap().to_string())
        .collect::<Vec<_>>();

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

