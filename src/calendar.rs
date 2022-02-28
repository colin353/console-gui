use google_calendar3::api::Channel;
use google_calendar3::CalendarHub;
use google_calendar3::{Error, Result};

pub use crate::{AppState, CalendarEvent};
use std::sync::{Arc, Mutex};

struct CalendarAPI {
    hub: CalendarHub,
}

pub async fn run(data: Arc<Mutex<AppState>>) {
    let search_start = chrono::prelude::Local::now() - chrono::Duration::hours(2);
    let search_end = search_start + chrono::Duration::days(1);

    let cal = CalendarAPI::new().await;
    let (_, events) = cal
        .hub
        .events()
        .list("primary")
        .single_events(true)
        .time_min(&search_start.to_rfc3339())
        .time_max(&search_end.to_rfc3339())
        .doit()
        .await
        .unwrap();

    let mut output = None;

    for event in events.items.unwrap() {
        // Skip invalid events
        if event.start.is_none() || event.status.as_deref().unwrap_or("") == "cancelled" {
            continue;
        }
        let start =
            chrono::DateTime::parse_from_rfc3339(event.start.unwrap().date_time.as_ref().unwrap())
                .unwrap();
        let end =
            chrono::DateTime::parse_from_rfc3339(event.end.unwrap().date_time.as_ref().unwrap())
                .unwrap();

        output = Some(CalendarEvent {
            title: event.summary.unwrap_or_else(String::new),
            time: format!("{} - {}", start.format("%l:%M%P"), end.format("%l:%M%P")),
            start: start.timestamp() as i64,
            zoom_url: event
                .conference_data
                .as_ref()
                .and_then(|d| d.entry_points.as_ref())
                .and_then(|e| {
                    if !e.is_empty() {
                        e[0].label.clone()
                    } else {
                        None
                    }
                }),
        })
    }

    if output.is_some() {
        data.lock().unwrap().calendar = output;
    }
}

impl CalendarAPI {
    async fn new() -> Self {
        let secret = yup_oauth2::read_application_secret("/home/colinwm/.clientsecret.json")
            .await
            .unwrap();

        let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
            secret,
            yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
        )
        .persist_tokens_to_disk("/home/colinwm/.console_gui_auth.json")
        .build()
        .await
        .unwrap();

        let hub = CalendarHub::new(
            hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()),
            auth,
        );

        Self { hub }
    }
}
