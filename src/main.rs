use eframe::egui::{FontDefinitions, FontFamily};
use eframe::{egui, epi};

use std::sync::{Arc, Mutex};

mod calendar;
mod github;
mod keyboard;
mod style;

mod home;
mod pull_requests;
mod shortcuts;

#[derive(Clone)]
pub struct App {
    data: Arc<Mutex<AppState>>,
}

pub struct CalendarEvent {
    title: String,
    time: String,
    start: i64,
    zoom_url: Option<String>,
}

pub struct GitHubNotification {
    title: String,
    action: String,
    repository: String,
    time: i64,
}

enum PageState {
    Home,
    Shortcuts { selected: Option<usize> },
    PullRequests,
}

pub struct AppState {
    page: PageState,
    frame: Option<epi::Frame>,
    clock: String,
    calendar: Option<CalendarEvent>,
    notifications: Vec<GitHubNotification>,
}

impl AppState {
    fn new() -> Self {
        Self {
            page: PageState::Home,
            frame: None,
            clock: Self::clock_time(),
            calendar: None,
            notifications: Vec::new(),
        }
    }

    fn clock_time() -> String {
        chrono::prelude::Local::now()
            .format(" %h %d  %l:%M%P ")
            .to_string()
    }

    fn footer(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let mut frame = egui::Frame::none();
            frame.margin = egui::Vec2::new(5.0, 5.0);
            frame = frame.stroke(egui::Stroke::new(style::STROKE, style::FG));
            frame.fill = style::FG;
            frame.show(ui, |ui| {
                let desc = egui::Label::new(
                    egui::RichText::new(&self.clock)
                        .monospace()
                        .color(egui::Color32::BLACK),
                );
                ui.add(desc);
            });
        });
    }

    fn commands(&self) -> &[&str] {
        match self.page {
            PageState::Home => {
                if let Some(cal) = &self.calendar {
                    if cal.zoom_url.is_some() {
                        return &["JOIN", "PRS", "CAL", "SHCT"];
                    }
                }

                &["", "PRS", "CAL", "SHCT"]
            }
            PageState::Shortcuts { .. } => &["1", "2", "3", "BACK"],
            PageState::PullRequests => &["IPR", "SUB", "REV"],
        }
    }
}

impl App {
    fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(AppState::new())),
        }
    }

    async fn start_async(&self) {
        let data = self.data.clone();
        tokio::spawn(async move {
            calendar::run(data).await;
        });
        let data = self.data.clone();
        tokio::spawn(async move {
            github::run(data).await;
        });

        // Timer to refresh UI
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
        loop {
            interval.tick().await;
            let mut _data = self.data.lock().unwrap();
            if let Some(frame) = _data.frame.as_ref() {
                frame.request_repaint();
            }
            _data.clock = AppState::clock_time();
        }
    }

    fn bindkeys(&self) {
        let _self = self.clone();
        std::thread::spawn(move || {
            keyboard::handle_input_events(move |key| {
                let mut state = _self.data.lock().unwrap();
                match state.page {
                    PageState::Home => state.handle_kbd_home(key),
                    PageState::Shortcuts { .. } => state.handle_kbd_shortcuts(key),
                    PageState::PullRequests { .. } => state.handle_kbd_pull_requests(key),
                };

                if let Some(frame) = state.frame.as_ref() {
                    frame.request_repaint();
                }
            });
        });
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "console GUI"
    }

    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Create a font definition object
        let mut font_def = FontDefinitions::default();
        font_def
            .family_and_size
            .insert(egui::TextStyle::Heading, (FontFamily::Proportional, 24.));

        font_def
            .family_and_size
            .insert(egui::TextStyle::Monospace, (FontFamily::Monospace, 24.));
        // Set the size of text styles
        // Load the font using ctx
        ctx.set_fonts(font_def);

        let mut app_data = self.data.lock().unwrap();
        app_data.frame = Some(frame.clone());
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        egui::SidePanel::right("right_panel")
            .frame(egui::Frame::none().fill(style::BG))
            .default_width(30.0)
            .max_width(30.0)
            .min_width(30.0)
            .show(ctx, |ui| {
                let app_data = self.data.lock().unwrap();

                let spacing = ui.available_height() / 4.0;

                for cmd in app_data.commands() {
                    ui.allocate_ui(egui::Vec2::new(20.0, spacing), |ui| {
                        let padding = 10.0;
                        let text_space = cmd.len() as f32 * 25.0 + 2.0 * padding;

                        ui.add_space((spacing - text_space) / 2.0);
                        let mut frame = egui::Frame::none();
                        frame.margin = egui::Vec2::new(5.0, padding);
                        if cmd == &"JOIN" {
                            frame = frame.stroke(egui::Stroke::new(style::STROKE, style::FG));
                        };

                        frame.show(ui, |ui| {
                            let desc =
                                egui::Label::new(egui::RichText::new(cmd.to_string()).monospace());
                            ui.add(desc);
                        });
                        ui.add_space((spacing - text_space) / 2.0);
                    });
                }
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(style::BG))
            .show(ctx, |ui| {
                let app_data = self.data.lock().unwrap();

                let (rect, _) = ui.allocate_exact_size(
                    egui::Vec2::new(ui.available_width(), ui.available_height() - 37.0),
                    egui::Sense::hover(),
                );
                app_data.footer(ui);

                let mut content_ui = ui.child_ui(rect, egui::Layout::top_down(egui::Align::Min));
                match app_data.page {
                    PageState::Home => app_data.render_home(&mut content_ui),
                    PageState::Shortcuts { selected: _ } => {
                        app_data.render_shortcuts(&mut content_ui)
                    }
                    PageState::PullRequests => app_data.render_pull_requests(&mut content_ui),
                }
                content_ui.add_space(content_ui.available_height());
            });
    }
}

#[tokio::main]
async fn main() {
    let options = eframe::NativeOptions::default();

    let app = App::new();
    app.bindkeys();

    let _app = app.clone();
    tokio::spawn(async move { _app.start_async().await });
    eframe::run_native(Box::new(app), options);
}
