use elasticsearch::http::transport::Transport;
use elasticsearch::{Elasticsearch, IndexParts};
use serde_json::json;

fn index_data() -> Result<(), Box<dyn Error>> {
    let transport = Transport::single_node("http://localhost:9200")?;
    let client = Elasticsearch::new(transport);

    let body = json!({
        "name": "Alice",
        "age": 30,
        "hobbies": ["reading", "swimming"]
    });

    let response = client
        .index(IndexParts::IndexId("my-index", "1"), body)
        .send()?;

    println!("Indexed document: {:?}", response);

    Ok(())
}
