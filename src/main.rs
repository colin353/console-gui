use eframe::egui::{FontDefinitions, FontFamily};
use eframe::{egui, epi};

const ZOOM_COLOR: egui::Color32 = egui::Color32::from_rgb(0x2D, 0x8C, 0xFF);

struct App {}

impl epi::App for App {
    fn name(&self) -> &str {
        "console GUI"
    }

    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &epi::Frame,
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
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        egui::SidePanel::right("right_panel")
            .frame(egui::Frame::none())
            .default_width(30.0)
            .max_width(30.0)
            .min_width(30.0)
            .show(ctx, |ui| {
                let spacing = ui.available_height() / 4.0;

                for cmd in &["JOIN", "PRS", "CAL", "SHCT"] {
                    ui.allocate_ui(egui::Vec2::new(20.0, spacing), |ui| {
                        let padding = 10.0;
                        let text_space = cmd.len() as f32 * 25.0 + 2.0 * padding;

                        ui.add_space((spacing - text_space) / 2.0);
                        let mut frame = egui::Frame::none();
                        frame.margin = egui::Vec2::new(5.0, padding);
                        if cmd == &"JOIN" {
                            frame = frame.stroke(egui::Stroke::new(0.25, egui::Color32::WHITE));
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
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                ui.allocate_ui(
                    egui::Vec2::new(ui.available_width(), ui.available_height() - 37.0),
                    |ui| {
                        shortcut(ui);
                        ui.add_space(ui.available_height());
                    },
                );

                footer(ui);
            });
    }
}

fn home_pane(ui: &mut egui::Ui) {
    let mut frame = egui::Frame::none();
    frame.margin = egui::Vec2::new(20.0, 20.0);
    frame.show(ui, |ui| {
        ui.set_clip_rect(ui.max_rect());
        ui.horizontal(|ui| {
            let mut frame = egui::Frame::none();
            frame.margin = egui::Vec2::new(5.0, 5.0);
            frame = frame.stroke(egui::Stroke::new(0.25, egui::Color32::WHITE));
            frame.show(ui, |ui| {
                let desc = egui::Label::new(egui::RichText::new("-30M").monospace());
                ui.add(desc);
            });

            ui.add_space(20.0);
            ui.heading("Search Sync Extremely Long String");
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            let mut frame = egui::Frame::none();
            frame.margin = egui::Vec2::new(5.0, 5.0);
            frame = frame.stroke(egui::Stroke::new(0.25, ZOOM_COLOR));
            frame.fill = ZOOM_COLOR;
            frame.show(ui, |ui| {
                let desc = egui::Label::new(
                    egui::RichText::new("ZOOM")
                        .monospace()
                        .color(egui::Color32::BLACK),
                );
                ui.add(desc);
            });

            ui.add_space(20.0);
            ui.heading("2:00pm - 3:00pm");
        });

        ui.add_space(40.0);

        ui.horizontal(|ui| {
            let mut frame = egui::Frame::none();
            frame.margin = egui::Vec2::new(5.0, 5.0);
            frame = frame.stroke(egui::Stroke::new(0.25, egui::Color32::WHITE));
            frame.show(ui, |ui| {
                let desc = egui::Label::new(egui::RichText::new("+ 2H").monospace());
                ui.add(desc);
            });

            ui.add_space(20.0);
            ui.add(egui::Label::new(
                egui::RichText::new("@look commented").heading(),
            ));
        });
    });
}

fn shortcut(ui: &mut egui::Ui) {
    let shortcuts = &[&["guvcview", "zoom"]];

    let mut frame = egui::Frame::none();
    frame = frame.stroke(egui::Stroke::new(0.25, egui::Color32::WHITE));
    frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            let mut frame = egui::Frame::none();
            frame = frame.stroke(egui::Stroke::new(0.25, egui::Color32::WHITE));
            frame.fill = egui::Color32::WHITE;
            frame.show(ui, |ui| {
                let desc = egui::Label::new(
                    egui::RichText::new(" 1 ")
                        .monospace()
                        .color(egui::Color32::BLACK),
                );
                ui.add(desc);
            });
        });
        ui.add_space(ui.available_height());
    });
}

fn footer(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        let mut frame = egui::Frame::none();
        frame.margin = egui::Vec2::new(5.0, 5.0);
        frame = frame.stroke(egui::Stroke::new(0.25, egui::Color32::WHITE));
        frame.fill = egui::Color32::WHITE;
        frame.show(ui, |ui| {
            let desc = egui::Label::new(
                egui::RichText::new(" Feb 17  2:35PM ")
                    .monospace()
                    .color(egui::Color32::BLACK),
            );
            ui.add(desc);
        });
    });
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(App {}), options);
}
