use eframe::egui::{FontDefinitions, FontFamily};
use eframe::{egui, epi};

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
                        ui.centered_and_justified(|ui| {
                            let mut frame = egui::Frame::none();
                            if cmd == &"JOIN" {
                                frame = frame.stroke(egui::Stroke::new(0.25, egui::Color32::WHITE));
                            };

                            frame.show(ui, |ui| {
                                let desc = egui::Label::new(
                                    egui::RichText::new(cmd.to_string()).monospace(),
                                );
                                ui.add(desc);
                            });
                        })
                    });
                }
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                ui.heading("testing");
                ui.heading("code review");
            });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(App {}), options);
}
