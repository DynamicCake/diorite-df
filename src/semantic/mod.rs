//! Crates a new parse tree using the [crate::ast] module

use core::panic;
use std::option::IntoIter;
use std::sync::Arc;

use crate::ast::prelude::*;
use crate::codegen::block;
use crate::common::prelude::*;
use crate::dump::Action;
use crate::error::semantic::{
    ActionNotFoundError, ActionReference, DuplicateLineStarter, SelectorNotFound,
};
use crate::project::{CheckedProjectFiles, ParsedProjectFiles, Project, ProjectFile, TreeFile};
use crate::tree::prelude::*;
use crate::{dump::ActionDump, error::semantic::SemanticError};

use futures::{stream, StreamExt};
use lasso::{Resolver, RodeoResolver, Spur};

pub mod stmt;
pub mod top;

#[derive(Debug)]
pub struct Analyzer<'d> {
    dump: &'d ActionDump,
    resolver: &'d RodeoResolver,
}

pub struct AnalysisResult<'d> {
    errors: Vec<SemanticError<'d>>,
    program: Project<CheckedProjectFiles<'d>>,
    starters: StarterSet,
}

impl<'d> Analyzer<'d> {
    pub async fn verify(
        &'d self,
        program: Project<ParsedProjectFiles>,
    ) -> Option<AnalysisResult<'d>> {
        let errs = &program.files;
        if errs.eof_errs.is_empty() || errs.parse_errs.is_empty() || errs.lex_errs.is_empty() {
            return None;
        }
        let res = self.resolve_self(program).await;
        return Some(res);
    }

    fn new(resolver: &'d RodeoResolver, dump: &'d ActionDump) -> Self {
        Self { resolver, dump }
    }

    async fn resolve_self(&'d self, program: Project<ParsedProjectFiles>) -> AnalysisResult<'d> {
        let mut starters = StarterSet::new();
        let mut starter_collisions = Vec::new();
        let programs_len = program.files.parsed.len();

        // Add all starters to `starters` and get errors early
        program.files.parsed.iter().map(|file| {
            file.resolution.root.top_statements.iter().map(|top| {
                if let Err(err) = top.add_starter(file.path, &mut starters) {
                    starter_collisions.push(err);
                }
            });
        });

        let stream = stream::iter(program.files.parsed)
            .map(|file| self.resolve_file(file.resolution.root, file.path));

        let mut errors = Vec::new();
        let roots: Vec<_> = stream.buffered(programs_len).collect().await;

        AnalysisResult {
            errors,
            starters,
            program: Project {
                resources: program.resources.clone(),
                files: CheckedProjectFiles { programs: roots },
                hash: program.hash,
            },
        }
    }

    pub(crate) fn advance_tree_expr(
        params: &mut IntoIter<TreeExprValue>,
    ) -> Result<TreeExprValue, AdvanceTreeExprError> {
        let a = params.next();
        todo!()
    }
}

pub enum AdvanceTreeExprError {


}

