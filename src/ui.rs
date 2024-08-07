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

// Build Dependencies
// https://github.com/fltk-rs/fltk-rs?tab=readme-ov-file#build-dependencies

#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(clippy::needless_update)]

include!(concat!(env!("OUT_DIR"), "/ui.rs"));

use fltk::prelude::*;

pub(crate) struct Point {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

pub(crate) fn center() -> Point {
    Point {
        x: (fltk::app::screen_size().0 / 2.0) as i32,
        y: (fltk::app::screen_size().1 / 2.0) as i32,
    }
}

pub(crate) fn save_as_callback(ui: &UserInterface) {
    let mut gui = ui.clone();
    gui.select_file.set_callback(move |_| {
        let mut nfc = fltk::dialog::FileChooser::new(
            ".",
            "*.mp4",
            fltk::dialog::FileChooserType::Create,
            "Save Download File as:",
        );
        nfc.set_position(center().x, center().y);

        nfc.show();
        while nfc.shown() {
            fltk::app::wait();
        }

        if let Some(selected_filename) = nfc.value(1) {
            gui.save_filename.set_value(&selected_filename[..])
        }
    });
}

pub(crate) fn download_callback(ui: &UserInterface, config: crate::config::Config) {
    let mut gui = ui.clone();
    gui.download.set_callback(move |_| {
        let url = gui.url.value();
        if url.is_empty() {
            fltk::dialog::alert_default("URL is required!");
            return;
        };

        // filename entry is optional
        let filename_field = gui.save_filename.value();
        let filename = if filename_field.is_empty() {
            None
        } else {
            Some(&filename_field)
        };

        let (videos, save_filename) = match crate::assets::extract_assets(&url, &config, filename) {
            Ok(value) => value,
            Err(error) => {
                fltk::dialog::alert_default(&error.error_message);
                return;
            }
        };

        let mut progress_bar = gui.progress.clone();
        let config = config.clone();

        let progress_mode =  progress_bar.into() ;
        std::thread::spawn(move || {
            if let Err(error) =
                crate::downloader::download_video(videos, config, save_filename, progress_mode)
            {
                fltk::dialog::alert_default(&error.error_message);
                return;
            };
            fltk::dialog::message_default("Download completed!");
        });
    });
}
