use hyper::body::HttpBody as _;
use hyper::http::{Request, Response};

async fn run() {
    let pat = std::env::var("PAT").expect("must provide $PAT env var");

    let auth = base64::encode(format!("colinwm:{}", pat).into_bytes());

    let client = hyper::Client::new();
    let req = Request::builder()
        .uri(format!(
            "https://api.github.com/notifications?participating=true&per_page=100"
        ))
        .header("Accept", "application/vnd.github.v3+json")
        .header("Authorization", format!("Basic {}", auth))
        .body(hyper::Body::empty())
        .unwrap();
    let mut response = client.request(req).await.unwrap();

    let mut data: Vec<u8> = Vec::new();
    while let Some(chunk) = response.body_mut().data().await {
        data.extend(chunk.unwrap().as_ref());
    }

    let value: serde_json::Value =
        serde_json::from_str(std::str::from_utf8(&data).expect("response was not utf8!"))
            .expect("response was not valid JSON!");

    if let serde_json::Value::Array(arr) = value {
        for notification in arr {
            if let serde_json::Value::String(reason) = &notification["reason"] {
                if reason == "team_mention" || reason == "assign" {
                    continue;
                }
            }
        }
    }
}
