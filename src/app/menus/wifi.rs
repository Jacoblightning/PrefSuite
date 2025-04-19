use crate::app::{MyApp, Menu};


use eframe::egui;
use eframe::egui::{RichText};

pub fn main(app: &mut MyApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        if ui.button(RichText::new("Back")).clicked() {
            app.selected_menu = Menu::Main;
        }
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label(RichText::new("Wi-Fi Menu:").size(36.0));
        });
    });
}