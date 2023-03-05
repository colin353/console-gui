use hyper::body::HttpBody as _;
use hyper::http::Request;

pub use crate::{AppState, GitHubNotification, PullRequest};
use std::sync::{Arc, Mutex};

pub async fn run(data: Arc<Mutex<AppState>>) {
    let _data = data.clone();
    tokio::spawn(async move { pulls(_data.clone()).await });

    let pat = std::env::var("PAT").expect("must provide $PAT env var");

    let auth = base64::encode(format!("colinwm:{pat}").into_bytes());

    let https = hyper_rustls::HttpsConnector::with_native_roots();
    let client = hyper::Client::builder().build(https);

    let mut interval = tokio::time::interval(std::time::Duration::from_secs(120));
    loop {
        let req = Request::builder()
            .uri("https://api.github.com/notifications?participating=true&per_page=100")
            .header("Accept", "application/vnd.github.v3+json")
            .header("Authorization", format!("Basic {auth}"))
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

                let action = notification["reason"].as_str().unwrap().to_string();
                let url = notification["subject"]
                    .get("url")
                    .map(|v| v.as_str().unwrap_or(""))
                    .unwrap_or("")
                    .to_string();
                let mut url = url.replace("api.github.com/repos", "github.com");

                if url.contains("/pulls/") {
                    url = url.replace("/pulls/", "/pull/");
                }

                if url.is_empty() {
                    continue;
                }

                notifications.push(GitHubNotification {
                    title: notification["subject"]
                        .get("title")
                        .unwrap()
                        .as_str()
                        .unwrap_or("")
                        .to_string(),
                    action,
                    repository,
                    time,
                    url,
                });
            }
        }

        data.lock().unwrap().notifications = notifications;

        interval.tick().await;
    }
}

pub async fn pulls(data: Arc<Mutex<AppState>>) {
    let pat = std::env::var("PAT").expect("must provide $PAT env var");
    let auth = base64::encode(format!("colinwm:{pat}").into_bytes());
    let https = hyper_rustls::HttpsConnector::with_native_roots();
    let client = hyper::Client::builder().build(https);

    let mut interval = tokio::time::interval(std::time::Duration::from_secs(180));
    loop {
        let req = Request::builder()
            .uri("https://api.github.com/search/issues?q=is:pr%20author:colinwm%20is:open")
            .header("Accept", "application/vnd.github.v3+json")
            .header("Authorization", format!("Basic {auth}"))
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

        let mut prs = Vec::new();
        if let Some(serde_json::Value::Array(arr)) = value.get("items") {
            for item in arr {
                prs.push(extract_pr(item));
            }
        }
        data.lock().unwrap().open_prs = prs;

        let req = Request::builder()
            .uri("https://api.github.com/search/issues?q=is:pr%20author:colinwm%20is:closed")
            .header("Accept", "application/vnd.github.v3+json")
            .header("Authorization", format!("Basic {auth}"))
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

        let mut prs = Vec::new();
        if let Some(serde_json::Value::Array(arr)) = value.get("items") {
            for item in arr {
                prs.push(extract_pr(item));
            }
        }
        data.lock().unwrap().closed_prs = prs;

        interval.tick().await;
    }
}

fn extract_pr(item: &serde_json::Value) -> PullRequest {
    let time = chrono::DateTime::parse_from_rfc3339(item["updated_at"].as_str().unwrap())
        .unwrap()
        .timestamp();

    let url = item.get("html_url").unwrap().as_str().unwrap().to_string();
    let re = regex::Regex::new("^https://github.com/([^/]+)/([^/]+)").unwrap();
    let mut repo_name = String::new();
    if let Some(cap) = re.captures_iter(&url).next() {
        repo_name = cap[2].to_string();
    }

    PullRequest {
        title: item.get("title").unwrap().as_str().unwrap().to_string(),
        url: item.get("html_url").unwrap().as_str().unwrap().to_string(),
        updated_at: time,
        repo_name,
    }
}
