use hyper::body::HttpBody as _;
use hyper::http::Request;

pub use crate::{AppState, GitHubNotification};
use std::sync::{Arc, Mutex};

pub async fn run(data: Arc<Mutex<AppState>>) {
    let pat = std::env::var("PAT").expect("must provide $PAT env var");

    let auth = base64::encode(format!("colinwm:{}", pat).into_bytes());

    let https = hyper_rustls::HttpsConnector::with_native_roots();
    let client = hyper::Client::builder().build(https);

    let mut interval = tokio::time::interval(std::time::Duration::from_secs(120));
    loop {
        let req = Request::builder()
            .uri("https://api.github.com/notifications?participating=true&per_page=100")
            .header("Accept", "application/vnd.github.v3+json")
            .header("Authorization", format!("Basic {}", auth))
            .header("User-Agent", "colinwm")
            .body(hyper::Body::empty())
            .unwrap();
        let mut response = client.request(req).await.unwrap();

        let mut bytes: Vec<u8> = Vec::new();
        while let Some(chunk) = response.body_mut().data().await {
            bytes.extend(chunk.unwrap().as_ref());
        }

        let data_str = std::str::from_utf8(&bytes).expect("response was not utf8!");
        let value: serde_json::Value =
            serde_json::from_str(data_str).expect("response was not valid JSON!");

        let mut notifications = Vec::new();

        if let serde_json::Value::Array(arr) = value {
            for notification in arr {
                if let serde_json::Value::String(reason) = &notification["reason"] {
                    if reason == "state_change" || reason == "team_mention" || reason == "assign" {
                        continue;
                    }
                }

                let repository = notification["repository"]
                    .get("name")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string();

                let time = chrono::DateTime::parse_from_rfc3339(
                    notification["updated_at"].as_str().unwrap(),
                )
                .unwrap()
                .timestamp();

                notifications.push(GitHubNotification {
                    title: notification["subject"]
                        .get("title")
                        .unwrap()
                        .as_str()
                        .unwrap_or("")
                        .to_string(),
                    action: notification["reason"].as_str().unwrap().to_string(),
                    repository,
                    time,
                });
            }
        }

        data.lock().unwrap().notifications = notifications;

        interval.tick().await;
    }
}
