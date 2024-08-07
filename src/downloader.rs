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

use crate::config;
use indicatif;
use isahc::{prelude::Configurable, ResponseExt};
use serde_json;
use std::io::{Read, Write};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ProgressBarMode {
    Quiet,
    Console,
    Graphic { progress_bar: fltk::misc::Progress },
}

impl From<fltk::misc::Progress> for ProgressBarMode {
    fn from(progress_bar: fltk::misc::Progress) -> Self {
        Self::Graphic { progress_bar }
    }
}

pub(crate) fn download_video(
    videos: Vec<serde_json::Value>,
    config: config::Config,
    save_filename: String,
    mut progress_mode: ProgressBarMode,
) -> Result<(), crate::errors::AppError> {
    for video in videos.iter() {
        if video.get(&config.page_config.video_profile).unwrap()
            == &config.page_config.selected_video_profile[..]
        {
            let real_vid = match video["url"].as_str() {
                Some(real_vid) => real_vid,
                None => return Err(crate::errors::AppError::bad_address()),
            };
            println!("Downloading Video...");
            println!("{}", real_vid);

            let request = isahc::Request::get(real_vid)
                .redirect_policy(isahc::config::RedirectPolicy::Limit(
                    config.download_config.max_redirect,
                ))
                .timeout(std::time::Duration::from_secs(
                    config.download_config.download_timeout_s,
                ))
                .metrics(true)
                .body(())?;
            let mut response = isahc::HttpClient::new().unwrap().send(request)?;

            let total_size = response.metrics().unwrap().download_progress().1;

            let progress_bar = indicatif::ProgressBar::new(total_size);

            match progress_mode {
                ProgressBarMode::Console => {
                    progress_bar.set_style(
                        indicatif::ProgressStyle::default_bar()
                            .progress_chars(&config.download_config.progress_chars)
                            .template(&config.download_config.progress_template)?,
                    );
                }
                ProgressBarMode::Graphic {
                    ref mut progress_bar,
                } => progress_bar.set_maximum(total_size as f64),
                ProgressBarMode::Quiet => (),
            }

            let file = std::fs::File::create(save_filename)?;
            let mut writer = std::io::BufWriter::new(file);

            let mut downloaded = 0;
            let mut buffer = [0u8; 8192]; //8k buffer

            loop {
                let bytes_read: usize = response.body_mut().read(&mut buffer).unwrap();
                if bytes_read == 0 {
                    break;
                }

                writer.write_all(&buffer[..bytes_read])?;
                downloaded += bytes_read as u64;
                match progress_mode {
                    ProgressBarMode::Console => progress_bar.set_position(downloaded as u64),
                    ProgressBarMode::Graphic {
                        ref mut progress_bar,
                    } => progress_bar.set_value(downloaded as f64),
                    ProgressBarMode::Quiet => (),
                }
            }
            break;
        }
    }
    Ok(())
}
