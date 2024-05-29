#[cfg(test)]
mod tests {
    use std::{
        env,
        fmt::Display,
        process::{exit, Command}
    };

    const CALYX_VERILOG_TEST_DIR: &str = "tests/calyx-verilog";

    fn test_runner<S: Display>(name: S) {
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
    fn test_verilator_harnesses() {
        test_runner("twice");
        test_runner("square");
    }
}
