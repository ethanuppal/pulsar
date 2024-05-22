use pulsar::utils::{error::*, loc::*};

extern crate pulsar;

pub fn main() {
    let code = "import std\n\nfunc main() {\n    std::print(1)\n}\n";
    let source = Source::File {
        name: "main.pl".into(),
        contents: code.into()
    };
    let loc = Loc {
        line: 3,
        col: 6,
        pos: 17,
        source: &source
    };
    let error = ErrorBuilder::new()
        .of_style(Style::Primary)
        .at_level(Level::Error)
        .at_region(loc, 4)
        .message("Invalid signature for `main`".into())
        .explain("Function declared to return `Unit` here".into())
        .fix("Consider adding `-> Int` after `main(...)`".into())
        .build();
    println!("{}", error.to_string())
}
