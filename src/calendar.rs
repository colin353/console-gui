use google_calendar3::CalendarHub;

pub use crate::{AppState, CalendarEvent};
use std::sync::{Arc, Mutex};

struct CalendarAPI {
    hub: CalendarHub,
}

pub async fn run(data: Arc<Mutex<AppState>>) {
    let search_start = chrono::prelude::Local::now() - chrono::Duration::hours(2);
    let search_end = search_start + chrono::Duration::days(2);

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

        let mut best_score = f32::NEG_INFINITY;
        let mut best_start: Option<chrono::DateTime<_>> = None;

        for event in events.items.unwrap() {
            let title = &event.summary.as_deref().unwrap_or("");

            // Skip invalid events
            if event.start.is_none()
                || event.start.as_ref().unwrap().date_time.is_none()
                || event.status.as_deref().unwrap_or("") == "cancelled"
            {
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
                //continue;
            }

            // Score the event. Show the most important upcoming event if there are two
            let mut score = 0.0;

            // Shorter meetings should be prioritized above longer ones
            score -= (end - start).num_minutes() as f32 / 30.0;

            if let Some(attendees) = event.attendees {
                for attendee in attendees {
                    if attendee.self_.is_some() {
                        if let Some(status) = attendee.response_status {
                            if status == "accepted" {
                                score += 5.0;
                            } else if status == "declined" {
                                continue;
                            } else if status == "needsAction" || status == "tentative" {
                                score -= 10.0;
                            }
                        }
                    }
                }
            }

            if best_start.is_some() && best_start.unwrap().timestamp() < start.timestamp() {
                continue;
            }

            // Two events starting at the same time, but one is better
            if best_start.is_some()
                && best_start.unwrap().timestamp() == start.timestamp()
                && best_score > score
            {
                continue;
            }

            best_start = Some(start);
            best_score = score;

            let zoom_url = event
                .conference_data
                .as_ref()
                .and_then(|d| d.entry_points.as_ref())
                .and_then(|e| {
                    if !e.is_empty() {
                        e[0].label.clone()
                    } else {
                        None
                    }
                })
                .and_then(|u| {
                    // Transform in to an xdg-open compatible link
                    // NOTE: xdg-open compatible link looks like this:
                    // zoomus://zoom.us/join?action=join&confno=99917074685&pwd=RWprdkxOOEpUUU84ejRVZ09td1NPUT09
                    let re = regex::Regex::new("/j/(.*?)\\?pwd=(.*?)$").unwrap();
                    if let Some(cap) = re.captures_iter(&u).next() {
                        return Some(format!(
                            "zoomus://zoom.us/join?action=join&confno={}&pwd={}",
                            &cap[1], &cap[2]
                        ));
                    }
                    None
                });

            output = Some(CalendarEvent {
                title: event.summary.unwrap_or_default(),
                time: format!("{} - {}", start.format("%l:%M%P"), end.format("%l:%M%P")),
                start: start.timestamp(),
                zoom_url,
            });
        }

        data.lock().unwrap().calendar = output;

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
