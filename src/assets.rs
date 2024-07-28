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
use isahc::{config::Configurable, ReadResponseExt};

pub(crate) fn extract_assets(
    url: &String,
    config: &config::Config,
    filename: Option<&String>,
) -> Result<(Vec<serde_json::Value>, String), crate::errors::AppError> {
    let client = isahc::HttpClient::builder()
        .redirect_policy(isahc::config::RedirectPolicy::Limit(
            config.download_config.max_redirect,
        ))
        .timeout(std::time::Duration::from_secs(
            config.download_config.download_timeout_s,
        ))
        .build()?;
    let mut response = client.get(url)?;
    let page_content = response.text()?;
    let script = match page_content.find(&config.page_config.script_start_marker[..]) {
        Some(start_index) => {
            let json_start = start_index + config.page_config.script_start_marker.len();
            match page_content[json_start..].find(&config.page_config.script_end_marker[..]) {
                Some(end_index) => &page_content[json_start..json_start + end_index],
                None => return Err(crate::errors::AppError::bad_address()),
            }
        }
        None => return Err(crate::errors::AppError::bad_address()),
    };
    let value: serde_json::Value = serde_json::from_str(&script)?;

    let mut nested_drm_tags = config.page_config.drm_nested_tag.to_owned();
    let drm_tag = nested_drm_tags.pop().unwrap();
    let mut drm_asset_values = &value;
    for tag in nested_drm_tags {
        drm_asset_values = match &drm_asset_values.get(tag) {
            Some(assets_values) => assets_values,
            None => return Err(crate::errors::AppError::bad_address()),
        }
    }

    if drm_asset_values
        .get(drm_tag)
        .map_or(false, |drm| drm.as_bool().map_or(false, |drm| drm))
    {
        return Err(crate::errors::AppError::drm_error());
    };

    let mut nested_video_tags = config.page_config.video_nested_tag.to_owned();
    let video_tag = nested_video_tags.pop().unwrap();
    let mut video_asset_values = &value;
    for tag in nested_video_tags {
        video_asset_values = match &video_asset_values.get(tag) {
            Some(assets_values) => assets_values,
            None => return Err(crate::errors::AppError::bad_address()),
        }
    }

    let videos = match video_asset_values[video_tag].as_array() {
        Some(videos) => videos.clone(),
        None => return Err(crate::errors::AppError::bad_address()),
    };

    let mut nested_title_tags = config.page_config.title_nested_tag.to_owned();
    let title_tag = nested_title_tags.pop().unwrap();
    let mut title_asset_values = &value;
    for tag in nested_title_tags {
        title_asset_values = match &title_asset_values.get(tag) {
            Some(assets_values) => assets_values,
            None => return Err(crate::errors::AppError::bad_address()),
        }
    }

    let save_filename = filename
        .map_or(
            &(title_asset_values
                .get(&title_tag)
                .ok_or(crate::errors::AppError::bad_address())?
                .as_str()
                .ok_or(crate::errors::AppError::bad_address())?
                .replace('/', "+")
                .replace('\\', "+")
                + ".mp4"),
            |filename| filename,
        )
        .to_owned();

    Ok((videos, save_filename))
}
