//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use anyhow::anyhow;
use pulsar_utils::error::ErrorManager;

pub trait OptionCheckError<T> {
    fn check_errors(
        self, error_manager: &mut ErrorManager
    ) -> anyhow::Result<T>;
}

impl<T> OptionCheckError<T> for Option<T> {
    /// Unwraps this value into an `Ok(...)` if it's `Some(...)`, otherwise,
    /// writes the error messages in `error_manager` to [`std::io::stdout()`]
    /// and returns an `Err(...)`. Non-error messages are always written.
    fn check_errors(
        self, error_manager: &mut ErrorManager
    ) -> anyhow::Result<T> {
        if error_manager.has_errors() {
            let mut buffer = Vec::new();
            error_manager.consume_and_write(&mut buffer)?;
            print!("{}", String::from_utf8_lossy(&buffer));
        }
        if let Some(result) = self {
            Ok(result)
        } else {
            Err(anyhow!("Exiting due to errors"))
        }
    }
}
