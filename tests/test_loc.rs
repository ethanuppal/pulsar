extern crate pulsar;

#[cfg(test)]
mod tests {
    use pulsar::utils::loc::{Loc, Source};

    #[test]
    fn test_loc_lines() {
        let source = Source::File {
            name: "test".to_string(),
            contents: "line1\nline2\nline3\nline4\nline5".to_string()
        };
        let loc = Loc {
            line: 3,
            col: 1,
            pos: 13, // Position of 'l' in "line3"
            source: &source
        };

        {
            let (lines, pos) = loc.lines(1, 1);

            assert_eq!(lines, vec!["line2", "line3", "line4"]);
            assert_eq!(pos, 1);
        }

        {
            let (lines, pos) = loc.lines(0, 0);

            assert_eq!(lines, vec!["line3"]);
            assert_eq!(pos, 0);
        }

        {
            let (lines, pos) = loc.lines(4, 0);

            assert_eq!(lines, vec!["line1", "line2", "line3"]);
            assert_eq!(pos, 2);
        }
    }
}
