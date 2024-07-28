/*
    pbskids-dl
    Copyright (C) 2024 The pbskids-dl Team

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License along
    with this program; if not, write to the Free Software Foundation, Inc.,
    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
*/

mod args;
mod errors;
mod assets;
mod config;
mod downloader;

fn main() -> std::process::ExitCode {
    let matches = args::cli_args();

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

    if !quiet {
        println!("{}", save_filename);
    }

    if let Err(error) = downloader::download_video(videos, config, save_filename, quiet) {
        if !quiet {
            eprintln!("{}", error);
        };
        return error.into();
    };

    crate::errors::AppError{
        error_code: crate::errors::LinuxExitCodes::SUCCESS as u8,
        error_message: "OK".to_string(),
    }.into()
}
