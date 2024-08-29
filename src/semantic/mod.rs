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

#[derive(Debug, Clone)]
pub struct Analyzer<'d> {
    dump: Arc<ActionDump>,
    resolver: Arc<RodeoResolver>,
}

pub struct AnalysisResult {
    errors: Vec<SemanticError>,
    program: Project<CheckedProjectFiles>,
    starters: StarterSet,
}

impl Analyzer {
    pub async fn verify(
        resolver: Arc<RodeoResolver>,
        program: Project<ParsedProjectFiles>,
        dump: Arc<ActionDump>,
    ) -> Option<AnalysisResult> {
        let errs = &program.files;
        if errs.eof_errs.is_empty() || errs.parse_errs.is_empty() || errs.lex_errs.is_empty() {
            return None;
        }
        if let Some(n) = Self::new(resolver, dump) {
            return Some(n.resolve_self(program).await);
        }
        None
    }

    fn new(resolver: Arc<RodeoResolver>, dump: Arc<ActionDump>) -> Option<Self> {
        Some(Self { resolver, dump })
    }

    async fn resolve_self(self, program: Project<ParsedProjectFiles>) -> AnalysisResult {
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
            .map(|file| self.clone().resolve_file(file.resolution.root, file.path));

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

    async fn resolve_file(&self, root: TreeRoot, file: Spur) -> AstRoot {
        let mut ast_top: Vec<AstTopLevel> = Vec::with_capacity(root.top_statements.len());
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
                        statements: self.clone().statements(e.statements, file).await,
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

    async fn statements(self, top: TreeStatements, file: Spur) -> AstStatements {
        todo!()
    }
}
