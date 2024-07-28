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
        .get_matches()
}
