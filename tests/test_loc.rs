//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

#[cfg(test)]
mod tests {
    use pulsar_utils::loc::{Loc, Source};

    #[test]
    fn test_loc_lines() {
        let source = Source::file("test", "line1\nline2\nline3\nline4\nline5");
        let loc = Loc {
            line: 3,
            col: 1,
            pos: 13, // Position of 'l' in "line3"
            source
        };

        {
            let (lines, idx) = loc.lines(1, 1);

            assert_eq!(lines, vec!["line2", "line3", "line4"]);
            assert_eq!(idx, 1);
        }

        {
            let (lines, idx) = loc.lines(0, 0);

            assert_eq!(lines, vec!["line3"]);
            assert_eq!(idx, 0);
        }

        {
            let (lines, idx) = loc.lines(4, 0);

            assert_eq!(lines, vec!["line1", "line2", "line3"]);
            assert_eq!(idx, 2);
        }
    }
}
