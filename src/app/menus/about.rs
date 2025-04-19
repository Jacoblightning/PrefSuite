use crate::app::{MyApp, Menu};


use eframe::egui;
use eframe::egui::{RichText};

pub fn main(app: &mut MyApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        if ui.button(RichText::new("Back")).clicked() {
            app.selected_menu = Menu::Main;
        }
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label(RichText::new("About").size(36.0));
        });
        ui.label("PrefSuite is a WIP app for modifying MacOS system preferences.");
        ui.label("I made this app because I was looking to make MacOS more like Linux. Unlike what everyone else is trying to do.");
        ui.label("Hopefully in the future, it can also be an open-source reference to the settings of MacOS.");
    });
}