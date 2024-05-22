const INDENT: &str = "    ";

pub fn make_indent(level: usize) -> String {
    INDENT.repeat(level)
}
