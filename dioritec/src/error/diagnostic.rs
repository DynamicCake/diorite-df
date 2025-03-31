//! Unfinished module for generating pretty errors

use std::sync::Arc;

use ariadne::{Color, Label, Report, ReportKind};
use lasso::{RodeoResolver, Spur};

use crate::{common::span::Span, error::syntax::UnexpectedToken};

use super::{syntax::LexerError, CompilerError};

// Report then file name
pub type CompilerReport<'src> = (Report<'src, (&'src str, Span<usize>)>, Spur);
pub type Diagnostics<'src> = Vec<CompilerReport<'src>>;
pub struct DiagnosticsGenerator {
    resolver: Arc<RodeoResolver>,
}
impl<'src> DiagnosticsGenerator {
    pub fn new(resolver: Arc<RodeoResolver>) -> Self {
        Self { resolver }
    }
    pub fn generate(&'src self, errors: Vec<CompilerError>) -> Diagnostics<'src> {
        let mut diagnostics = Vec::new();
        for err in errors {
            let err = match err {
                CompilerError::Lexer(err) => diagnostics.push(self.lexer_error(err)),
                CompilerError::Parse(err) => {} // self.parse_error(err),
                CompilerError::Eof(_) => {}     //todo!(),
                CompilerError::Semantic(_) => {}
            };
        }
        diagnostics
    }
    fn lexer_error(&'src self, error: LexerError) -> CompilerReport<'src> {
        let error = error.token;
        let file = self.resolver.resolve(&error.file_path);

        (
            Report::build(
                ReportKind::Error,
                "something",
                error.spanned.span.start as usize,
            )
            .with_code(1)
            .with_message("Invalid token".to_string())
            .with_label(
                Label::new((file, {
                    let Span { start, end } = error.spanned.span;
                    (start as usize)..(end as usize)
                }))
                .with_message("This token cannot be used in diorite")
                .with_color(Color::Red),
            )
            .finish(),
            error.file_path,
        )
    }

    fn parse_error(&self, err: UnexpectedToken) -> CompilerReport<'src> {
        // let UnexpectedToken { expected, received, expected_name, file } = err;
        // Report::build(ReportKind::Error, file, offset)
        todo!()
    }
}

pub fn generate_syntax_error<'src>(
    file: Arc<str>,
    error: UnexpectedToken,
) -> Report<'src, (Arc<str>, Span<usize>)> {
    let red = Color::Red;

    let expected = error.expected_print();
    Report::build(ReportKind::Error, file.clone(), 0)
        .with_code(1)
        .with_message("Syntax Error".to_string())
        .with_label(
            Label::new((file, {
                let Span { start, end } = error.received.span;
                (start as usize)..(end as usize)
            }))
            .with_message(format!(
                "Expected {} recieved {}",
                expected,
                error.received.data.expected_print()
            ))
            .with_color(red),
        )
        .finish()
}
