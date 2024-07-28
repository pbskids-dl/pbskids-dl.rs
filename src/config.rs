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

// Default config locations
const DEFAULT_CONFIG_PATHS: [&str; 2] = [
    "~/.config/pbskids-dl/config.json", //user generated has higher priority
    "/usr/share/pbskids-dl/config.json", //package installed location
];

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PageConfig {
    pub(crate) video_nested_tag: Vec<String>,
    pub(crate) title_nested_tag: Vec<String>,
    pub(crate) drm_nested_tag: Vec<String>,
    pub(crate) script_start_marker: String,
    pub(crate) script_end_marker: String,
    pub(crate) video_profile: String,
    pub(crate) selected_video_profile: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DownloadConfig {
    pub(crate) max_redirect: u32,
    pub(crate) download_timeout_s: u64,
    pub(crate) progress_chars: String,
    pub(crate) progress_template: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) page_config: PageConfig,
    pub(crate) download_config: DownloadConfig,
}

pub(crate) fn load_config(
    config_file_path: Option<&String>,
) -> Result<Config, crate::errors::AppError> {
    //user provided config file path is checked with clap
    //clap passes a default config file poiting to base path if there is no user provided
    let mut paths: Vec<&str> = Vec::new();
    paths.push(&config_file_path.unwrap()[..]);
    //generate a vector and combine clap config location with the defaults
    paths.extend_from_slice(&DEFAULT_CONFIG_PATHS);
    //loads config from the 1st path that is valid
    for path in paths {
        if let Ok(contents) = std::fs::read_to_string(path) {
            return Ok(serde_json::from_str(&contents[..])?);
        };
    }

    //if config file was not found in any of the location,
    //then return error
    let error = std::io::Error::new(std::io::ErrorKind::NotFound, "Config file not found!");
    Err(error.into())
}
