//! Crates a new parse tree using the [crate::ast] module

use std::iter;
use std::option::IntoIter;
use std::slice::ChunksMut;
use std::sync::Arc;

use crate::ast::prelude::*;
use crate::error::semantic::AnalysisResult;
use crate::project::parsed::{ParsedProjectFiles, TreeFile};
use crate::project::{Project, ProjectFile};
use crate::tree::prelude::*;
use crate::{dump::ActionDump, error::semantic::SemanticError};

use futures::{stream, SinkExt, StreamExt};
use lasso::RodeoResolver;
use tokio::sync::Mutex;

pub mod stmt;
pub mod top;

#[derive(Debug, PartialEq)]
pub struct Analyzer {
    resolver: Arc<RodeoResolver>,
    dump: Arc<ActionDump>,
}

/// TODO: Find a better name
#[derive(Debug, PartialEq)]
pub struct AnalyzedResult {
    pub errors: Vec<SemanticError>,
    pub files: Vec<ProjectFile<AnalyzedFile>>,
}

#[derive(Debug, PartialEq)]
pub struct AnalyzedFile {
    pub root: AstRoot,
}

impl AnalyzedFile {
    pub fn new(root: AstRoot) -> Self {
        Self { root }
    }
}

impl Analyzer {
    // #[allow(unused)]
    // async fn verify(&'d self, program: Project<ParsedProjectFiles>) -> Option<AnalysisResult<'d>> {
    //     let errs = &program.files;
    //     // Make sure that there are no errors in the parsing stage
    //     // Maybe this restriction can be lifted later
    //     if !errs.eof_errs.is_empty() || !errs.parse_errs.is_empty() || !errs.lex_errs.is_empty() {
    //         return None;
    //     }
    //     Some(self.resolve_self(program.files.parsed).await)
    // }
    //

    pub fn new(resolver: Arc<RodeoResolver>, dump: Arc<ActionDump>) -> Self {
        Self { resolver, dump }
    }

    pub async fn resolve<'a>(self, program: Vec<ProjectFile<TreeFile>>) -> AnalyzedResult {
        let mut starters = StarterSet::new();
        let mut errors = Vec::new();

        // Add all starters to `starters` and get errors early
        program.iter().for_each(|file| {
            file.resolution.root.top_statements.iter().for_each(|top| {
                if let Err(err) = top.add_starter(file.path, &mut starters) {
                    errors.push(SemanticError::DuplicateLineStarter(err));
                }
            });
        });

        let program_len = program.len();
        let files: Vec<_> = stream::iter(program)
            .map(|file| self.resolve_project_file(file))
            .buffered(program_len)
            .collect()
            .await;
        let files: Vec<_> = files
            .into_iter()
            .map(|mut f| {
                errors.append(&mut f.error);
                f.data
            })
            .collect();

        AnalyzedResult { errors, files }
    }

    pub(crate) fn advance_tree_expr(
        params: &mut IntoIter<TreeExprValue>,
    ) -> Result<TreeExprValue, AdvanceTreeExprError> {
        let a = params.next();
        todo!()
    }
}

pub enum AdvanceTreeExprError {}
