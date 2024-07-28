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

use crate::config;
use indicatif;
use isahc::{prelude::Configurable, ResponseExt};
use serde_json;
use std::io::{Read, Write};

pub(crate) fn download_video(
    videos: Vec<serde_json::Value>,
    config: config::Config,
    save_filename: String,
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
            let style = indicatif::ProgressStyle::default_bar()
                .progress_chars(&config.download_config.progress_chars)
                .template(&config.download_config.progress_template)?;
            progress_bar.set_style(style);

            let file = std::fs::File::create(save_filename)?;
            let mut writer = std::io::BufWriter::new(file);

            let mut downloaded: u64 = 0;
            let mut buffer = [0u8; 8192]; //8k buffer

            loop {
                let bytes_read: usize = response.body_mut().read(&mut buffer).unwrap();
                if bytes_read == 0 {
                    break;
                }

                writer.write_all(&buffer[..bytes_read])?;
                downloaded += bytes_read as u64;
                progress_bar.set_position(downloaded);
            }
            break;
        }
    }
    Ok(())
}
