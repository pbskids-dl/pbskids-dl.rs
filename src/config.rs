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

// Default config locations
const DEFAULT_CONFIG_PATHS: [&str; 2] = [
    "~/.config/pbskids-dl.rs/config.json", //user generated has higher priority
    "/usr/lib/pbskids-dl.rs/config.json", //package installed location
];

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct PageConfig {
    pub(crate) video_nested_tag: Vec<String>,
    pub(crate) title_nested_tag: Vec<String>,
    pub(crate) drm_nested_tag: Vec<String>,
    pub(crate) script_start_marker: String,
    pub(crate) script_end_marker: String,
    pub(crate) video_profile: String,
    pub(crate) selected_video_profile: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct DownloadConfig {
    pub(crate) max_redirect: u32,
    pub(crate) download_timeout_s: u64,
    pub(crate) progress_chars: String,
    pub(crate) progress_template: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
