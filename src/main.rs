// SPDX-FileCopyrightText: Copyright © 2025 Muhammad Alfi Syahrin
//
// SPDX-License-Identifier: MPL-2.0

use std::env;
use std::process::Command;

use slint::ComponentHandle;
slint::include_modules!();

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let start_page = match args.get(1).map(|s| s.as_str()) {
        Some("--liveiso") => 1,
        Some("--firststart") => 2,
        Some("--donation") => 3,
        Some(other) => {
            eprintln!("Unknown flag {other}, defaulting to First Start page");
            2
        }
        None => 2,
    };

    let app = MainWindow::new().unwrap();
    app.set_current_page(start_page);

    app.global::<Callbacks>().on_close(move || {
        std::process::exit(0);
    });

    app.global::<Callbacks>().on_openUrl(move |url| {
        if let Err(err) = open::that(url.as_str()) {
            eprintln!("Failed to open link: {err}");
        }
    });

    app.global::<Callbacks>().on_startInstaller(|| {
        if let Err(err) = Command::new("calamares-gui").spawn() {
            eprintln!("Failed to launch installer: {err}");
        }
    });

    app.global::<Callbacks>().on_launchUpdater(|| {
        let desktop_env = env::var("XDG_SESSION_DESKTOP").unwrap_or_default();

        let result = match desktop_env.as_str() {
            "gnome" => Command::new("gnome-software").arg("--mode=updates").spawn(),
            "kde" | "xfce" | "budgie" => {
                Command::new("plasma-discover").arg("--mode=update").spawn()
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unsupported desktop environment",
            )),
        };

        if let Err(err) = result {
            eprintln!(
                "Failed to launch software center for {desktop_env:?}: {err}\n\
                  Please update manually via terminal, e.g.:\n  sudo eopkg update"
            );
        }
    });

    app.global::<Callbacks>().on_installDriver(|| {
        let desktop_env = env::var("XDG_SESSION_DESKTOP").unwrap_or_default();

        let result = match desktop_env.as_str() {
            "gnome" => Command::new("gnome-software").arg("--details=nvidia-glx-driver-current").spawn(),
            "kde" | "xfce" | "budgie" => Command::new("plasma-discover")
                .args(["--category", "Hardware Drivers"])
                .spawn(),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unsupported desktop environment",
            )),
        };

        if let Err(err) = result {
            eprintln!(
                "Failed to launch software center for {desktop_env:?}: {err}\n\
                  Please install driver via terminal, e.g.:\n  sudo eopkg it nvidia-glx-driver-current"
            );
        }
    });
    app.run().unwrap();
}
