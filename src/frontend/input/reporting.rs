use std::fmt::Write;
use std::iter::zip;
use std::ops::Range;
use crate::error::source::SourceError;
use crate::frontend::input::SourceInput;
use crate::frontend::location::{SourceLocation, SourceRange};

struct AnnotationRange {
    lines: Range<usize>,
    slice_idx: Range<usize>,
}

impl SourceInput {

    pub fn create_error_report(&self, err: SourceError) -> String {
        let source_slice_range = self.source_range_to_range(err.err_loc());
        let error_slice = self.annotate_error_slice(source_slice_range, &err);
        format!(r#"
Error at {}:
{}
{}
        "#,
            err.err_loc(),
            error_slice,
            err.msg()
        )
    }

    fn annotate_error_slice(&self, annotation_range: AnnotationRange, error: &SourceError) -> String {
        let source_slice = &self.buffer[annotation_range.slice_idx];
        let mut buffer = String::new();
        let error_lines = error.err_loc().start.line..=error.err_loc().end.line;
        for (line_no, line) in zip(annotation_range.lines, source_slice.split("\n")) {
            let delim_char = if error_lines.contains(&line_no) {
                '>'
            } else {
                '|'
            };
            writeln!(buffer, "{} {} {}", line_no, delim_char, line).unwrap()
        }

        buffer
    }

    fn source_range_to_range(&self, range: SourceRange) -> AnnotationRange {
        let mut counter = self.buffer.chars().enumerate();
        let mut start_idx = 0;
        let mut end_idx = self.buffer.len();
        let mut loc = SourceLocation::default();
        let (start_line, end_line) = (range.start.line - 1, range.end.line + 3);
        loop {
            let Some((idx, ch)) = counter.next() else {
                break;
            };

            // bump the line and column
            loc.update_with_char(ch);

            if loc.line == start_line && loc.col == 0 {
                start_idx = idx;
            }

            if loc.line == end_line && loc.col == 0 {
                end_idx = idx;
                break;
            }
        };

        AnnotationRange {
            lines: start_line..end_line,
            slice_idx: start_idx..end_idx,
        }
    }
}
