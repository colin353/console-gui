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

    let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
    loop {
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

        let mut best_score = 0.0;
        let mut best_start = None;

        for event in events.items.unwrap() {
            // Skip invalid events
            if event.start.is_none() || event.status.as_deref().unwrap_or("") == "cancelled" {
                continue;
            }
            let now = chrono::prelude::Local::now();
            let start = chrono::DateTime::parse_from_rfc3339(
                event.start.unwrap().date_time.as_ref().unwrap(),
            )
            .unwrap();

            let end = chrono::DateTime::parse_from_rfc3339(
                event.end.unwrap().date_time.as_ref().unwrap(),
            )
            .unwrap();

            // If the current meeting is >75% over, don't show it
            if now > end - ((end - start) * 3) / 4 {
                continue;
            }

            // If the event is not today, don't show it
            if now.date() != start.date() {
                continue;
            }

            // Show the soonest upcoming event
            if let Some(b) = best_start {
                if start > b {
                    continue;
                }
            }

            // Score the event. Show the most important upcoming event if there are two
            let mut score = 0.0;

            // Shorter meetings should be prioritized above longer ones
            score -= (end - start).num_minutes() as f32 / 30.0;

            let user = std::env::var("USER").expect("must provide $USER env var");

            if let Some(attendees) = event.attendees {
                for attendee in attendees {
                    if let Some(email) = attendee.email {
                        if email.starts_with(&format!("{}@", user)) {
                            if let Some(status) = attendee.response_status {
                                if status == "accepted" {
                                    score += 5.0;
                                } else if status == "declined" {
                                    score -= 100.0;
                                }
                            }
                        }
                    }
                }
            }

            if best_start.is_some() && best_score > score {
                continue;
            }

            best_start = Some(start);
            best_score = score;

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
            });
        }

        if output.is_some() {
            data.lock().unwrap().calendar = output;
        }

        interval.tick().await;
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
