//! Crates a new parse tree using the [crate::ast] module

use std::sync::Arc;

use crate::ast::prelude::*;
use crate::common::prelude::*;
use crate::error::semantic::DuplicateLineStarter;
use crate::project::{CheckedProjectFiles, ParsedProjectFiles, Project, ProjectFile, TreeFile};
use crate::tree::prelude::*;
use crate::{dump::ActionDump, error::semantic::SemanticError};

use futures::{stream, StreamExt};
use lasso::{Resolver, RodeoResolver, Spur};

#[derive(Debug)]
pub struct Analyzer<'d> {
    dump: &'d ActionDump,
    resolver: &'d RodeoResolver,
}

pub struct AnalysisResult<'d> {
    errors: Vec<SemanticError>,
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

        let mut errors: Vec<SemanticError> = Vec::new();
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

    async fn resolve_file(&'d self, root: TreeRoot, file: Spur) -> AstRoot<'d> {
        let mut ast_top: Vec<AstTopLevel<'d>> = Vec::with_capacity(root.top_statements.len());
        for top in root.top_statements {
            ast_top.push(AstTopLevel::Event(match top {
                TreeTopLevel::Event(e) => {
                    let action = self.dump.actions.iter().find(|action| {
                        action.name == self.resolver.resolve(&e.name.data.name)
                            && action.codeblock_name == "PLAYER EVENT"
                    });

                    AstEvent {
                        type_tok: e.type_tok,
                        name: e.name,
                        action,
                        statements: self.statements(e.statements, file).await,
                        end_tok: e.end_tok,
                    }
                }
                TreeTopLevel::FuncDef(_) => todo!(),
                TreeTopLevel::ProcDef(_) => todo!(),
                TreeTopLevel::Recovery(_) => todo!(),
            }));
        }
        todo!()
    }

    async fn statements(&self, top: TreeStatements, file: Spur) -> AstStatements<'d> {
        todo!()
    }
}
