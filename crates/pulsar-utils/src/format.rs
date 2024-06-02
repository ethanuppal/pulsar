// Copyright (C) 2024 Ethan Uppal. All rights reserved.
const INDENT: &str = "    ";

pub fn make_indent(level: usize) -> String {
    INDENT.repeat(level)
}
