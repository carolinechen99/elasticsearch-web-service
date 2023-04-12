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

## Getting Started
### Prerequisites

1. Pull the Elasticsearch ARM image from the arm64v8/elasticsearch repository. This is the only image that supports ARM64 architecture. The version of Elasticsearch is 8.7.0.
```bash
docker pull arm64v8/elasticsearch:8.7.0 (for arm64)
```
2. Run the docker container with Elasticsearch image.
```bash
docker run -d --name elasticsearch -p 9200:9200 -p 9300:9300 -e "discovery.type=single-node" arm64v8/elasticsearch:8.7.0
```
3. Verify that Elasticsearch is running.
```bash
curl "http://localhost:9200"
```

4. create the index
```bash
curl -X PUT "localhost:9200/my_index?pretty"
```

5. add documents


6. refresh the index.
```bash
curl -X POST "localhost:9200/my_index/_refresh?pretty"
```




### References
  * [Elasticsearch](https://crates.io/crates/elasticsearch)
