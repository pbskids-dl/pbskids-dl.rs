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

//https://mariadb.com/kb/en/operating-system-error-codes/
#[repr(u8)]
pub(crate) enum LinuxExitCodes {
    SUCCESS = 0,   //no Error
    EIO = 5,       //I/O error
    EFAULT = 14,   //Bad address
    EINVAL = 22,   //Invalid argument
    ESPIPE = 29,   //Illegal seek
    EMLINK = 31,   //Too many links
    ENOMSG = 42,   //No message of desired type
    EBADE = 52,    //Invalid exchange
    ETIME = 62,    //Timer expired
    ENONET = 64,   //Machine is not on the network
    UNKNOWN = 255, //Error with no code has happened
}

#[derive(Debug)]
pub(crate) struct AppError {
    pub(crate) error_code: u8,
    pub(crate) error_message: String,
}

impl AppError {
    pub(crate) fn sucess() -> Self {
        AppError {
            error_code: LinuxExitCodes::SUCCESS as u8,
            error_message: format!("OK"),
        }
    }

    pub(crate) fn bad_address() -> Self {
        AppError {
            error_code: LinuxExitCodes::EFAULT as u8,
            error_message: format!(
                "ERROR: The video was not found! Is the link a PBS Kids Video link?"
            ),
        }
    }

    pub(crate) fn drm_error() -> Self {
        AppError {
            error_code: LinuxExitCodes::ESPIPE as u8,
            error_message: format!("DRM Content is not available in {}.", clap::crate_name!()),
        }
    }
}

impl From<isahc::Error> for AppError {
    fn from(error: isahc::Error) -> Self {
        let error_kind = match error.kind() {
            isahc::error::ErrorKind::BadClientCertificate
            | isahc::error::ErrorKind::BadServerCertificate
            | isahc::error::ErrorKind::InvalidContentEncoding
            | isahc::error::ErrorKind::InvalidCredentials
            | isahc::error::ErrorKind::InvalidRequest
            | isahc::error::ErrorKind::RequestBodyNotRewindable
            | isahc::error::ErrorKind::TlsEngine => LinuxExitCodes::EBADE,
            isahc::error::ErrorKind::ClientInitialization => LinuxExitCodes::EINVAL,
            isahc::error::ErrorKind::ConnectionFailed | isahc::error::ErrorKind::NameResolution => {
                LinuxExitCodes::ENONET
            }
            isahc::error::ErrorKind::Io => LinuxExitCodes::EIO,
            isahc::error::ErrorKind::Timeout => LinuxExitCodes::ETIME,
            isahc::error::ErrorKind::TooManyRedirects => LinuxExitCodes::EMLINK,
            _ => LinuxExitCodes::UNKNOWN,
        };
        AppError {
            error_code: error_kind as u8,
            error_message: format!(
                "Failed to access the web site! Error: {}",
                error.to_string()
            ),
        }
    }
}

impl From<isahc::http::Error> for AppError {
    fn from(error: isahc::http::Error) -> Self {
        AppError {
            error_code: LinuxExitCodes::ENOMSG as u8,
            error_message: format!(
                "Failed to access the web site! Error: {}",
                error.to_string()
            ),
        }
    }
}

impl From<indicatif::style::TemplateError> for AppError {
    fn from(error: indicatif::style::TemplateError) -> Self {
        AppError {
            error_code: LinuxExitCodes::EIO as u8,
            error_message: format!(
                "Failed to show the progress bar! Error: {}",
                error.to_string()
            ),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError {
            error_code: error
                .raw_os_error()
                .map_or(LinuxExitCodes::UNKNOWN as u8, |err| err as u8),
            error_message: format!("Failed to open. Error: {}", error.to_string()),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        AppError {
            error_code: error
                .io_error_kind()
                .map_or(LinuxExitCodes::UNKNOWN as u8, |err| err as u8),
            error_message: format!("Failed to parse. Error: {}", error.to_string()),
        }
    }
}

impl From<AppError> for std::process::ExitCode {
    fn from(error: AppError) -> Self {
        std::process::ExitCode::from(error.error_code)
    }
}

impl core::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.error_message)
    }
}
