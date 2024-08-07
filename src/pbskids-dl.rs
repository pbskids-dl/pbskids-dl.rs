/*
    pbskids-dl
    Copyright (C) 2024 The pbskids-dl Team

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
*/

mod args;
mod assets;
mod config;
mod downloader;
mod errors;
mod ui;

fn main() -> std::process::ExitCode {
    let matches = args::cli_args();

    match matches.get_flag("gui") {
        true => run_gui(),
        false => run_cli(matches),
    }
}

fn run_gui() -> std::process::ExitCode {
    let app = fltk::app::App::default().with_scheme(fltk::app::Scheme::Gtk);
    let gui = ui::UserInterface::open_gui();

    // load config file from default location
    let config = match crate::config::load_config(Some(&"config.json".to_string())) {
        Ok(config) => config,
        Err(error) => {
            fltk::dialog::alert_default(&error.error_message);
            return error.into();
        }
    };
    ui::save_as_callback(&gui);
    ui::download_callback(&gui, config);

    // Draw the GUI
    match app.run() {
        Ok(_) => crate::errors::AppError::sucess().into(),
        Err(error) => crate::errors::AppError {
            error_code: crate::errors::LinuxExitCodes::UNKNOWN as u8,
            error_message: error.to_string(),
        }
        .into(),
    }
}

fn run_cli(matches: clap::ArgMatches) -> std::process::ExitCode {
    // safe to unwrap url entry b/c required and verified by clap
    let url = matches.get_one::<String>("url").unwrap();

    // filename entry is optional
    let filename = matches.get_one::<String>("filename");

    // config file either default or user provided
    let config_file_path = matches.get_one::<String>("config");

    let quiet = matches.get_flag("quiet");

    let config = match config::load_config(config_file_path) {
        Ok(config) => config,
        Err(error) => {
            if !quiet {
                eprintln!("{}", error);
            };
            return error.into();
        }
    };

    let (videos, save_filename) = match assets::extract_assets(url, &config, filename) {
        Ok(value) => value,
        Err(error) => {
            if !quiet {
                eprintln!("{}", error);
            };
            return error.into();
        }
    };

    let progress_mode = if !quiet {
        println!("{}", save_filename);
        crate::downloader::ProgressBarMode::Console
    } else {
        crate::downloader::ProgressBarMode::Quiet
    };

    match downloader::download_video(videos, config, save_filename, progress_mode) {
        Ok(_) => crate::errors::AppError::sucess().into(),
        Err(error) => {
            if !quiet {
                eprintln!("{}", error);
            };
            error.into()
        }
    }
}
