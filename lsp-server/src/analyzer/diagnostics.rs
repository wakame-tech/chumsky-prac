use chumsky::prelude::Simple;
use itertools::Itertools;
use tower_lsp::lsp_types::{Diagnostic, Position, Range};

pub fn to_diagnostics(line: usize, err: &Simple<char>) -> Diagnostic {
    let span = err.span();
    let expected = err.expected().filter_map(|s| *s).join(", ");
    let found = err.found().unwrap();

    Diagnostic {
        range: Range {
            start: Position {
                line: line as u32,
                character: span.start as u32,
            },
            end: Position {
                line: line as u32,
                character: span.end as u32,
            },
        },
        severity: None,
        code: None,
        code_description: None,
        source: Some("source".to_string()),
        message: format!("expected {:?} but {:?}", expected, found).to_string(),
        related_information: None,
        tags: None,
        data: None,
    }
}
