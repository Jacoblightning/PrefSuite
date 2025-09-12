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

use eframe::egui;
use eframe::egui::RichText;
use log::{debug, error, info, trace};
use os_info::Version;

#[derive(Default)]
pub struct SIPData {
    // Sip bits cache
    bits: Option<u32>,
}

#[cfg(target_os = "macos")]
fn get_sip() -> Result<u32, String> {
    info!("Loading /usr/lib/libSystem.dylib");

    // Load the library libSystem.dylib, where the function to get the current SIP config is stored
    let lib = match unsafe { libloading::Library::new("/usr/lib/libSystem.dylib") } {
        Ok(lib) => lib,
        Err(e) => {
            error!("Failed to load libSystem.dylib: {}", e);
            return Err(e.to_string());
        }
    };
    trace!("Successfully loaded /usr/lib/libSystem.dylib");
    debug!("Loading function csr_get_active_config");

    // The function to get the current SIP (csr) config.
    let func: libloading::Symbol<unsafe extern "C" fn(*mut u32) -> i32> =
        match unsafe { lib.get(b"csr_get_active_config") } {
            Ok(func) => func,
            Err(e) => {
                error!("Failed to load function csr_get_active_config: {}", e);
                return Err(e.to_string());
            }
        };

    trace!("Successfully loaded /usr/lib/libSystem.dylib");

    debug!("Calling sip_active_config function");
    let mut sip_bits: u32 = 0;
    let sip_err = unsafe { func(&raw mut sip_bits) };
    if sip_err != 0 {
        error!("sip_active_config function failed: {}", sip_err);
        return Err(sip_err.to_string());
    }
    trace!("Successfully called sip_active_config function");
    info!("sip bits: {}", sip_bits);

    Ok(sip_bits)
}

#[cfg(not(target_os = "macos"))]
fn get_sip() -> Result<u32, String> {
    Err("sip is not supported on this platform".to_owned())
}

fn is_sip_disabled(bits: u32, version: &Version) -> bool {
    let sierra = Version::Semantic(10, 12, 0);
    let high_sierra = Version::Semantic(10, 13, 0);
    let mojave = Version::Semantic(10, 14, 0);
    let big_sur = Version::Semantic(11, 0, 0);

    // Checking for all El Capitan bits except allow_apple_internal as that can't be set
    if (bits & 239) != 239 {
        return false;
    } else if version < &sierra {
        return true;
    }

    // Check for any_recovery_os
    if (bits & 256) == 0 {
        return false;
    } else if version < &high_sierra {
        return true;
    }

    // check for unapproved_kexts
    if !(bits & 512) == 0 {
        return false;
    } else if version < &mojave {
        return true;
    }

    // check or allow_executable_policy_override
    if (bits & 1024) == 0 {
        return false;
    } else if version < &big_sur {
        return true;
    }

    // check for allow_unauthenticated_root
    (bits & 2048) != 0
}

fn show_sip_bits(ui: &mut egui::Ui, bits: u32, version: &Version) {
    let sierra = Version::Semantic(10, 12, 0);
    let high_sierra = Version::Semantic(10, 13, 0);
    let mojave = Version::Semantic(10, 14, 0);
    let big_sur = Version::Semantic(11, 0, 0);

    ui.label(
        RichText::new(format!(
            "CSR/SIP is: {}",
            if bits == 0 {
                "Fully Enabled"
            } else if is_sip_disabled(bits, version) {
                "Fully Disabled"
            } else {
                "Custom:"
            }
        ))
        .size(32.0),
    );
    ui.label(format!(
        "CSR_ALLOW_UNTRUSTED_KEXTS (Allow unsigned kernel drivers to be installed and loaded): {}",
        if (bits & (1 << 0)) != 0 {
            "Allowed"
        } else {
            "Forbidden"
        }
    ));
    ui.label(format!(
        "CSR_ALLOW_UNRESTRICTED_FS (Allows unrestricted filesystem access): {}",
        if (bits & (1 << 1)) != 0 {
            "Allowed"
        } else {
            "Forbidden"
        }
    ));
    ui.label(format!(
        "CSR_ALLOW_TASK_FOR_PID (Alows tracking processes based off of a provided process ID): {}",
        if (bits & (1 << 2)) != 0 {
            "Allowed"
        } else {
            "Forbidden"
        }
    ));
    ui.label(
        format!(
            "CSR_ALLOW_KERNEL_DEBUGGER (Allows attacking a low level kernel debugger to the system): {}",
            if (bits & (1 << 3)) != 0 {"Allowed"} else {"Forbidden"}
        )
    );
    ui.label(
        format!(
            "CSR_ALLOW_APPLE_INTERNAL (Allows apple internal feature set (primarily for development devices)): {}",
            if (bits & (1 << 4)) != 0 {"Allowed"} else {"Forbidden"}
        )
    );
    ui.label(format!(
        "CSR_ALLOW_UNRESTRICTED_DTRACE (Allows unrestricted dtrace usage): {}",
        if (bits & (1 << 5)) != 0 {
            "Allowed"
        } else {
            "Forbidden"
        }
    ));
    ui.label(format!(
        "CSR_ALLOW_UNRESTRICTED_NVRAM (Allows unrestricted NVRAM write): {}",
        if (bits & (1 << 6)) != 0 {
            "Allowed"
        } else {
            "Forbidden"
        }
    ));
    ui.label(
        format!(
            "CSR_ALLOW_DEVICE_CONFIGURATION (Allows custom device trees (based off of speculation. There is little public info on what this bit does)): {}",
            if (bits & (1 << 7)) != 0 {"Allowed"} else {"Forbidden"}
        )
    );
    // Those were all the EL Capitan bits
    if version < &sierra {
        return;
    }
    ui.label(
        format!(
            "CSR_ALLOW_ANY_RECOVERY_OS (Skip BaseSystem Verification, primarily for custom recoveryOS images): {}",
            if (bits & (1 << 8)) != 0 {"Allowed"} else {"Forbidden"}
        )
    );
    // Only 1 bit was added in Sierra
    if version < &high_sierra {
        return;
    }
    ui.label(format!(
        "CSR_ALLOW_UNAPPROVED_KEXTS (Allows unapproved kernel driver installation/loading): {}",
        if (bits & (1 << 9)) != 0 {
            "Allowed"
        } else {
            "Forbidden"
        }
    ));
    // Same for High Sierra
    if version < &mojave {
        return;
    }
    ui.label(format!(
        "CSR_ALLOW_EXECUTABLE_POLICY_OVERRIDE (Allows override of executable policy): {}",
        if (bits & (1 << 10)) != 0 {
            "Allowed"
        } else {
            "Forbidden"
        }
    ));
    // Same for Mojave
    if version < &big_sur {
        return;
    }
    ui.label(format!(
        "CSR_ALLOW_UNAUTHENTICATED_ROOT (Allows custom APFS snapshots to be booted): {}",
        if (bits & (1 << 11)) != 0 {
            "Allowed"
        } else {
            "Forbidden"
        }
    ));
}

pub fn main(app: &mut MyApp, ctx: &egui::Context) {
    let binding = os_info::get();
    let version = binding.version();

    let el_capitan = Version::Semantic(10, 11, 0);

    egui::CentralPanel::default().show(ctx, |ui| {
        if ui.button(RichText::new("Back")).clicked() {
            app.selected_menu = Menu::Main;
        }
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label(RichText::new("System Integrity Protection Menu:").size(36.0));
        });

        if version < &el_capitan {
            ui.label("Your system does not have System Integrity Protection");
            return;
        }

        if app.sip_data.bits.is_none() {
            match get_sip() {
                Ok(bits) => {
                    app.sip_data.bits = Some(bits);
                }
                Err(e) => {
                    error!("Failed to get SIP: {e}");
                }
            }
        }

        if let Some(bits) = app.sip_data.bits {
            show_sip_bits(ui, bits, version);
        }
    });
}
