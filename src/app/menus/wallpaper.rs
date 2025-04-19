use std::path::PathBuf;
use crate::app::{MyApp, Menu};

use rusqlite::{Connection};

use eframe::egui;
use eframe::egui::RichText;
use rusqlite::fallible_iterator::FallibleIterator;

#[derive(Default)]
pub struct WallpaperData {
    new_path: Option<String>,
    // Whether a file is selected
    noselect: bool,
    // If there was a database error
    dberror: bool,
}

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

// Modify the plist file
fn change_wallpaper_pre_mavericks(new_path: &str) -> Result<(), String> {Ok(())}

/// This is only possible thanks to the amazing reverse engineering work done over here. Give them a star.
/// https://github.com/tech-otaku/macos-desktop
fn change_wallpaper_mavericks_to_sonoma(new_path: &str) -> Result<(), String> {
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

    // Delete old data
    conn.execute("DELETE FROM data;",        ()).unwrap();
    conn.execute("DELETE FROM preferences;", ()).unwrap();

    // Add new data
    conn.execute("INSERT INTO data(rowid,value) VALUES (1,?1)", [new_path]).unwrap();

    conn.execute("INSERT INTO preferences(rowid,key,data_id,picture_id) VALUES (1,1,1,3)", ()).unwrap();
    conn.execute("INSERT INTO preferences(rowid,key,data_id,picture_id) VALUES (2,1,1,4)", ()).unwrap();
    conn.execute("INSERT INTO preferences(rowid,key,data_id,picture_id) VALUES (3,1,1,2)", ()).unwrap();
    conn.execute("INSERT INTO preferences(rowid,key,data_id,picture_id) VALUES (4,1,1,1)", ()).unwrap();

    conn.close().unwrap();


    Ok(())
}

// TODO: Use applescript
fn change_wallpaper_sonoma_plus(new_path: &str) -> Result<(), String> {Ok(())}


fn change_wallpaper(new_path: &str) -> Result<(), String> {
    let binding = os_info::get();
    let version = binding.version();
    Err(version.to_string())
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

        let wallpaper_path;

        match current_wallpaper {
            Ok(wallpaper) => {
                ui.label(RichText::new(format!("Current Wallpaper: {wallpaper}")).size(20.0));
                wallpaper_path = wallpaper;
                app.wallpaper_data.dberror = false;
            }
            Err(e) => {
                ui.label(RichText::new(format!("Failed to get current Wallpaper: {e}")).size(20.0));
                wallpaper_path = "".into();
                app.wallpaper_data.dberror = true;
            }
        };

        if !wallpaper_path.is_empty() && !app.wallpaper_data.dberror {
            let wallpaper_path = PathBuf::from(wallpaper_path);
            if wallpaper_path.exists() {
                ui.image(String::from("file://") + wallpaper_path.to_str().unwrap());
            } else {
                ui.label(RichText::new("Wallpaper does not exist.").size(20.0));
            }
        }

        if !app.wallpaper_data.dberror {
            if ui.button("Change Wallpaper").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("image", &["png", "jpg", "jpeg", "webp"])
                    .pick_file() {
                    app.wallpaper_data.new_path = Some(path.display().to_string());
                }
            }

            if let Some(picked_path) = &app.wallpaper_data.new_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }

            if ui.button("Change").clicked() {
                if let Some(new_path) = &app.wallpaper_data.new_path {
                    app.wallpaper_data.noselect = false;
                    match change_wallpaper(new_path) {
                        Ok(_) => {}
                        Err(e) => {
                            ui.label(RichText::new(format!("Failed to set Wallpaper: {e}")).size(20.0));
                        }
                    }
                } else {
                    app.wallpaper_data.noselect = true;
                }
            }

            if app.wallpaper_data.noselect {
                ui.label("You have to select an image.");
            }
        }
    });
}