//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use std::io::stdout;

use anyhow::anyhow;
use pulsar_utils::error::ErrorManager;

pub trait OptionCheckError<T> {
    fn check_errors(
        self, error_manager: &mut ErrorManager
    ) -> anyhow::Result<T>;
}

impl<T> OptionCheckError<T> for Option<T> {
    fn check_errors(
        self, error_manager: &mut ErrorManager
    ) -> anyhow::Result<T> {
        if let Some(result) = self {
            Ok(result)
        } else {
            if error_manager.has_errors() {
                error_manager.consume_and_write(&mut stdout())?;
            }
            Err(anyhow!("Exiting due to errors"))
        }
    }
}
