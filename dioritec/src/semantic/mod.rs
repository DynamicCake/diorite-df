//! Crates a new parse tree using the [crate::ast] module

use std::option::IntoIter;

use crate::ast::prelude::*;
use crate::project::parsed::{ParsedProjectFiles, TreeFile};
use crate::project::{Project, ProjectFile};
use crate::tree::prelude::*;
use crate::{dump::ActionDump, error::semantic::SemanticError};

use futures::{stream, StreamExt};
use lasso::RodeoResolver;

pub mod stmt;
pub mod top;

#[derive(Debug)]
pub struct Analyzer<'d> {
    dump: &'d ActionDump,
    resolver: &'d RodeoResolver,
}

pub struct AnalysisResult<'d> {
    pub errors: Vec<SemanticError<'d>>,
    pub files: Vec<ProjectFile<AnalyzedFile<'d>>>,
    // Not useful in codegen
    // pub starters: StarterSet,
}

#[derive(Debug, PartialEq)]
pub struct AnalyzedFile<'d> {
    pub root: AstRoot<'d>,
}

impl<'d> AnalyzedFile<'d> {
    pub fn new(root: AstRoot<'d>) -> Self {
        Self { root }
    }
}

impl<'d> Analyzer<'d> {
    #[allow(unused)]
    async fn verify(&'d self, program: Project<ParsedProjectFiles>) -> Option<AnalysisResult<'d>> {
        let errs = &program.files;
        // Make sure that there are no errors in the parsing stage
        // Maybe this restriction can be lifted later
        if !errs.eof_errs.is_empty() || !errs.parse_errs.is_empty() || !errs.lex_errs.is_empty() {
            return None;
        }
        Some(self.resolve_self(program.files.parsed).await)
    }

    pub fn new(resolver: &'d RodeoResolver, dump: &'d ActionDump) -> Self {
        Self { resolver, dump }
    }

    pub async fn resolve_self(&'d self, program: Vec<ProjectFile<TreeFile>>) -> AnalysisResult<'d> {
        let mut starters = StarterSet::new();
        let mut starter_collisions = Vec::new();
        let programs_len = program.len();

        // Add all starters to `starters` and get errors early
        program.iter().for_each(|file| {
            file.resolution.root.top_statements.iter().for_each(|top| {
                if let Err(err) = top.add_starter(file.path, &mut starters) {
                    starter_collisions.push(err);
                }
            });
        });

        let stream =
            stream::iter(program).map(|file| self.resolve_project_file(file));

        let errors = Vec::new();
        let files: Vec<_> = stream.buffered(programs_len).collect().await;

        AnalysisResult { errors, files }
    }

    pub(crate) fn advance_tree_expr(
        params: &mut IntoIter<TreeExprValue>,
    ) -> Result<TreeExprValue, AdvanceTreeExprError> {
        let a = params.next();
        todo!()
    }
}

pub enum AdvanceTreeExprError {}
