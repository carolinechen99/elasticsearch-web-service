# elasticsearch-web-service

## Overview
This project builds a web microservice that uses Rust and Elasticsearch to index and searches large amounts of structured and unstructured data.

This project is a work in progress.

User can index a document in Elasticsearch.
```bash
POST /my_index/_doc
{
    "title": "My Document",
    "body": "This is some text that I want to search for."
}
```
Then we let the user search for a document in Elasticsearch.

```bash
GET /my_index/_search
{
    "query": {
        "match": {
            "body": "text"
        }
    }
}
```





### References
  * [Elasticsearch](https://crates.io/crates/elasticsearch)
