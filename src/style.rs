pub const ZOOM_COLOR: egui::Color32 = egui::Color32::from_rgb(0x2D, 0x8C, 0xFF);
pub const BG: egui::Color32 = egui::Color32::BLACK;
pub const FG: egui::Color32 = egui::Color32::GRAY;
pub const FG_MUTED: egui::Color32 = egui::Color32::from_rgb(80, 80, 80);
pub const STROKE: f32 = 1.0;

fn rounding_div(a: i64, b: i64) -> i64 {
    (a as f64 / b as f64).round() as i64
}

pub fn eta(time: i64) -> String {
    let now = chrono::prelude::Utc::now().timestamp();
    let eta = now - time;

    let sign = if eta.signum() == 1 { "+" } else { "-" };
    let eta = eta.abs();

    // ETA > 12 hours, show at least 1 day
    const MINUTE: i64 = 60;
    const HOUR: i64 = 60 * MINUTE;
    if eta > 24 * HOUR {
        return format!("{}{:>2}d", sign, rounding_div(eta, 24 * HOUR));
    } else if eta > 12 * HOUR {
        return format!("{} 1d", sign);
    } else if eta > HOUR {
        return format!("{}{:>2}h", sign, rounding_div(eta, HOUR));
    } else {
        return format!("{}{:>2}m", sign, rounding_div(eta, MINUTE));
    }
}
