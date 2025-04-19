/*
    PrefSuite. A Preferences suite for MacOS
    Copyright (C) 2025-Present Jacob (https://github.com/jacoblightning)

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use crate::app::{Menu, MyApp};
use rusqlite::Connection;
use std::path::PathBuf;

use eframe::egui;
use eframe::egui::RichText;

#[derive(Default)]
pub struct WallpaperData {
    new_path: Option<String>,
    // Whether a file is selected
    noselect: bool,
    // If there was a database error
    dberror: bool,
    // If there was an error changing the wallpaper
    changerror: Option<String>,
    // Whether wallpaper data is out of date
    reloadneeded: Option<bool>,
    wpaper: Option<Result<String, String>>,
}

fn kill_dock() {
    let s = sysinfo::System::new_all();

    for process in s.processes().values() {
        if process.name() == "Dock" {
            process.kill();
        }
    }
}

// TODO: This
fn get_current_wallpaper_pre_mavericks() -> Result<String, String> {
    Ok("".into())
}

fn get_current_wallpaper_mavericks_to_sonoma() -> Result<String, String> {
    let homedir = std::env::var("HOME");
    if homedir.is_err() {
        return Err(String::from("HOME not set"));
    }
    let homedir = homedir.unwrap();

    let db: PathBuf = [
        &homedir,
        "Library/Application Support/Dock/desktoppicture.db",
    ]
    .iter()
    .collect();
    if !db.exists() {
        return Err(String::from("Database file not found :("));
    }

    let conn = Connection::open(db);

    if let Err(con) = conn {
        return Err(con.to_string());
    }

    let conn = conn.unwrap();

    let mut stmt = conn
        .prepare("SELECT cast(value as text) from data ORDER BY rowid DESC")
        .unwrap();

    let iter = stmt.query_map([], |row| row.get(0)).unwrap();

    let mut values: Vec<String> = Vec::new();

    for value in iter {
        values.push(value.unwrap());
    }

    if values.len() == 3 {
        Ok(values[1].clone())
    } else if values.len() == 2 {
        Ok(values[0].clone())
    } else if values[0] == "3" {
        Ok("Default Wallpaper".to_string())
    } else {
        Ok(values[0].clone())
    }
}

fn get_current_wallpaper_sonoma_plus() -> Result<String, String> {
    let osascript = std::process::Command::new("osascript")
        .arg("-e")
        .arg("tell app \"finder\" to get posix path of (get desktop picture as alias)")
        .output();

    if let Err(osaerr) = osascript {
        return Err(osaerr.to_string());
    }
    Ok(String::from_utf8(osascript.unwrap().stdout)
        .unwrap()
        .strip_suffix('\n')
        .unwrap()
        .to_string())
}

// TODO: Modify the plist file
#[allow(unused_variables)] fn change_wallpaper_pre_mavericks(new_path: &str) -> Result<(), String> {
    Ok(())
}

/// This is only possible thanks to the amazing reverse engineering work done over here. Give them a star.
/// https://github.com/tech-otaku/macos-desktop
fn change_wallpaper_mavericks_to_sonoma(new_path: &str) -> Result<(), String> {
    let homedir = std::env::var("HOME");
    if homedir.is_err() {
        return Err(String::from("HOME not set"));
    }
    let homedir = homedir.unwrap();

    let db: PathBuf = [
        &homedir,
        "Library/Application Support/Dock/desktoppicture.db",
    ]
    .iter()
    .collect();
    if !db.exists() {
        return Err(String::from("Database file not found :("));
    }

    let conn = Connection::open(db);

    if let Err(con) = conn {
        return Err(con.to_string());
    }

    let conn = conn.unwrap();

    // Delete old data
    conn.execute("DELETE FROM data;", ()).unwrap();
    conn.execute("DELETE FROM preferences;", ()).unwrap();

    // Add new data
    conn.execute("INSERT INTO data(rowid,value) VALUES (1,?1)", [new_path])
        .unwrap();

    conn.execute(
        "INSERT INTO preferences(rowid,key,data_id,picture_id) VALUES (1,1,1,3)",
        (),
    )
    .unwrap();
    conn.execute(
        "INSERT INTO preferences(rowid,key,data_id,picture_id) VALUES (2,1,1,4)",
        (),
    )
    .unwrap();
    conn.execute(
        "INSERT INTO preferences(rowid,key,data_id,picture_id) VALUES (3,1,1,2)",
        (),
    )
    .unwrap();
    conn.execute(
        "INSERT INTO preferences(rowid,key,data_id,picture_id) VALUES (4,1,1,1)",
        (),
    )
    .unwrap();

    conn.close().unwrap();

    Ok(())
}

fn change_wallpaper_sonoma_plus(new_path: &str) -> Result<(), String> {
    match std::process::Command::new("osascript")
        .arg("-e")
        .arg(format!("tell application \"System Events\" to tell every desktop to set picture to \"{new_path}\" as POSIX file"))
        .spawn()
        .unwrap()
        .wait() {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}

fn change_wallpaper(new_path: &str) -> Result<(), String> {
    let binding = os_info::get();
    let version = binding.version();

    let sonoma = os_info::Version::Semantic(14, 0, 0);
    let mavericks = os_info::Version::Semantic(10, 9, 0);

    let res = if version >= &sonoma {
        change_wallpaper_sonoma_plus(new_path)
    } else if version >= &mavericks {
        change_wallpaper_mavericks_to_sonoma(new_path)
    } else {
        change_wallpaper_pre_mavericks(new_path)
    };
    kill_dock();
    res
}

fn get_current_wallpaper() -> Result<String, String> {
    let binding = os_info::get();
    let version = binding.version();

    let sonoma = os_info::Version::Semantic(14, 0, 0);
    let mavericks = os_info::Version::Semantic(10, 9, 0);

    if version >= &sonoma {
        get_current_wallpaper_sonoma_plus()
    } else if version >= &mavericks {
        get_current_wallpaper_mavericks_to_sonoma()
    } else {
        get_current_wallpaper_pre_mavericks()
    }
}

pub fn main(app: &mut MyApp, ctx: &egui::Context) {
    if app.wallpaper_data.reloadneeded.is_none() {
        app.wallpaper_data.reloadneeded = Some(true);
    }

    let current_wallpaper = if app.wallpaper_data.reloadneeded.unwrap() {
        app.wallpaper_data.reloadneeded = Some(false);
        let w = get_current_wallpaper();
        app.wallpaper_data.wpaper = Some(w.clone());
        w
    } else {
        app.wallpaper_data.wpaper.clone().unwrap()
    };

    egui::CentralPanel::default().show(ctx, |ui| {
        if ui.button(RichText::new("Back")).clicked() {
            app.selected_menu = Menu::Main;
        }
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label(RichText::new("Wallpaper Menu:").size(36.0));
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
                ui.collapsing("Wallpaper:", |ui| {
                    ui.image(String::from("file://") + wallpaper_path.to_str().unwrap());
                });
            } else {
                ui.label(RichText::new("Wallpaper does not exist.").size(20.0));
            }
        }

        if !app.wallpaper_data.dberror {
            if ui.button("Change Wallpaper").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    //.add_filter("image", &["png", "jpg", "jpeg", "webp", "heic", "heif"])
                    .pick_file()
                {
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
                        Ok(_) => {
                            app.wallpaper_data.changerror = None;
                            app.wallpaper_data.reloadneeded = Some(true);
                        }
                        Err(e) => {
                            app.wallpaper_data.changerror = Some(e.to_string());
                        }
                    }
                } else {
                    app.wallpaper_data.noselect = true;
                }
            }

            if app.wallpaper_data.noselect {
                ui.label("You have to select an image.");
            }

            if let Some(error) = app.wallpaper_data.changerror.clone() {
                ui.label(RichText::new(format!("Failed to set Wallpaper: {error}")).size(20.0));
            }
        }
    });
}
