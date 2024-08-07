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

pub(crate) fn cli_args() -> clap::ArgMatches {
    clap::Command::new(clap::crate_name!())
        .help_expected(true)
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        .arg(
            clap::Arg::new("url")
                .required(true)
                .num_args(1)
                .help("The page you land on when a video is playing."),
        )
        .arg(
            clap::Arg::new("filename")
                .required(false)
                .short('f')
                .long("filename")
                .num_args(1)
                .help("The file to store the video (optional)."),
        )
        .arg(
            clap::Arg::new("config")
                .required(false)
                .short('c')
                .long("config")
                .help("Config File in JSON format.")
                .num_args(1)
                .default_value("config.json"),
        )
        .arg(
            clap::Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Suppress output.")
                .num_args(0),
        )
        .arg(
            clap::Arg::new("gui")
                .conflicts_with_all(["url", "filename", "quiet"])
                .long("GUI")
                .short('g')
                .help("launches GUI version")
//                .hide(true)
                .num_args(0)
        )
        .get_matches()
}
