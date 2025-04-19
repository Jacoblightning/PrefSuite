use std::path::PathBuf;
use crate::app::{MyApp, Menu};

use rusqlite::{Connection};

use eframe::egui;
use eframe::egui::RichText;

fn get_current_wallpaper() -> Result<String, String> {
    let homedir = std::env::var("HOME");
    if homedir.is_err() {
        return Err(String::from("HOME not set"));
    }
    let homedir = homedir.unwrap();

    let db: PathBuf = [&homedir, "Library/Application Support/Dock/desktoppicture.db"].iter().collect();
    if !db.exists() {
        return Err(String::from("Database file not found :("))
    }

    let conn = Connection::open(db);

    if conn.is_err() {
        return Err(conn.unwrap_err().to_string());
    }

    let conn = conn.unwrap();

    let mut stmt = conn.prepare("SELECT value from data WHERE ROWID=2").unwrap();

    let value = stmt.query_row([], |row| row.get(0)).unwrap();

    Ok(value)
}

pub fn main(app: &mut MyApp, ctx: &egui::Context) {
    let current_wallpaper = get_current_wallpaper();

    egui::CentralPanel::default().show(ctx, |ui| {
        if ui.button(RichText::new("Back")).clicked() {
            app.selected_menu = Menu::Main;
        }
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label(RichText::new("Main Menu").size(36.0));
        });

        match current_wallpaper {
            Ok(wallpaper) => {
                ui.label(RichText::new(format!("Current Wallpaper: {wallpaper}")).size(20.0));
            }
            Err(e) => {
                ui.label(RichText::new(format!("Failed to get current Wallpaper: {e}")).size(20.0));
            }
        };
    });
}