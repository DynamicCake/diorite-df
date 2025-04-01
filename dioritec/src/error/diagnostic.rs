//! Unfinished module for generating pretty errors

use std::{
    ops::{Range, RangeBounds},
    sync::Arc,
};

use ariadne::{Color, Label, Report, ReportKind};
use lasso::{RodeoResolver, Spur};

use crate::{common::span::Span, error::syntax::UnexpectedToken};

use super::{semantic::SemanticError, syntax::{LexerError, UnexpectedEOF}, CompilerError};

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
    fn convert_span(span: Span) -> Range<usize> {
        (span.start as usize)..(span.end as usize)
    }
    pub fn generate(&'src self, errors: Vec<CompilerError>) -> Diagnostics<'src> {
        errors
            .into_iter()
            .map(|err| match err {
                CompilerError::Lexer(err) => self.lexer_error(err),
                CompilerError::Parse(err) => self.parse_error(err),
                CompilerError::Eof(err) => self.eof_error(err), //todo!(),
                CompilerError::Semantic(err) => self.semantic_error(err),
            })
            .collect()
    }
    fn lexer_error(&'src self, error: LexerError) -> CompilerReport<'src> {
        let error = error.token;
        let file = self.resolver.resolve(&error.file_path);

        (
            Report::build(
                ReportKind::Error,
                "lexer",
                error.spanned.span.start as usize,
            )
            .with_code(-1)
            .with_message("Invalid token")
            .with_label(
                Label::new((file, Self::convert_span(error.spanned.span)))
                    .with_message("This token cannot be used in diorite")
                    .with_color(Color::Red),
            )
            .finish(),
            error.file_path,
        )
    }

    fn parse_error(&'src self, error: UnexpectedToken) -> CompilerReport<'src> {
        let file = self.resolver.resolve(&error.file);
        let print = error.expected_print();
        (
            Report::build(
                ReportKind::Error,
                "parse",
                error.received.span.start as usize,
            )
            .with_message("Unexpected token")
            .with_label(
                Label::new((file, Self::convert_span(error.received.span)))
                    .with_message(format!("Expected one of {}", print))
                    .with_color(Color::Red),
            )
            .finish(),
            error.file,
        )
    }

    fn eof_error(&'src self, error: UnexpectedEOF) -> CompilerReport<'src> {
        let file = self.resolver.resolve(&error.path);
        (
            Report::build(
                ReportKind::Error,
                "parse",
                (error.len) as usize,
            )
            .with_message("Unexpected EOF")
            .with_label(
                Label::new((file, ((error.len - 1) as usize)..(error.len as usize)))
                    .with_message(format!("Expected one of {:?}", error.expected))
                    .with_color(Color::Red),
            )
            .finish(),
            error.path,
        )
    }

    fn semantic_error(&'src self, error: SemanticError) -> CompilerReport<'src> {
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
