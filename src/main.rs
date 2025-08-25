// SPDX-FileCopyrightText: Copyright © 2025 Muhammad Alfi Syahrin
//
// SPDX-License-Identifier: MPL-2.0

use std::process::Command;

use slint::ComponentHandle;
slint::include_modules!();

fn main() {
    let app = MainWindow::new().unwrap();

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

    app.run().unwrap();
}
