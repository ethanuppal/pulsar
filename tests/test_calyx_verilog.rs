//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

#[cfg(test)]
mod tests {
    use std::{
        env,
        fmt::Display,
        process::{exit, Command}
    };

    const CALYX_VERILOG_TEST_DIR: &str = "tests/calyx-verilog";

    fn run_verilator_test<S: Display>(name: S) {
        let cwd = env::current_dir()
            .expect("Failed to get the current working directory");
        env::set_current_dir(CALYX_VERILOG_TEST_DIR)
            .expect("Failed to change directory");
        let output = Command::new("make")
            .arg(format!("N={}", name))
            .output()
            .expect("Failed to execute command");

        let status = output.status.code().unwrap_or(1);
        if status != 0 {
            exit(status);
        }
        env::set_current_dir(cwd.as_path())
            .expect("Failed to restore the current working directory");
    }

    #[test]
    fn twice() {
        run_verilator_test("twice");
    }

    #[test]
    fn square() {
        run_verilator_test("square");
    }

    #[test]
    fn map_single() {
        run_verilator_test("map_single");
    }

    #[test]
    fn map() {
        run_verilator_test("map");
    }

    #[test]
    fn math() {
        run_verilator_test("math");
    }
}
