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
