use std::sync::Arc;

use ariadne::{Color, Label, Report, ReportKind};

use crate::{parser::error::UnexpectedToken, span::Span};

pub fn generate_syntax_error<'src>(
    file: Arc<str>,
    error: UnexpectedToken,
) -> Report<'src, (Arc<str>, Span)> {
    let red = Color::Red;

    let expected = error.expected_print();
    Report::build(ReportKind::Error, file.clone(), 0)
        .with_code(1)
        .with_message(format!("Syntax Error"))
        .with_label(
            Label::new((file, error.received.span))
                .with_message(format!(
                    "Expected {} recieved {}",
                    expected,
                    error.received.data.expected_print()
                ))
                .with_color(red),
        )
        .finish()
}
