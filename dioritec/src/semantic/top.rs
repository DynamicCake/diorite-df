use lasso::Spur;

use crate::{
    error::semantic::{ActionNotFoundError, ActionReference, SemanticError},
    project::{parsed::TreeFile, ProjectFile},
};

use super::{
    AnalyzedFile, Analyzer, AstEvent, AstFuncDef, AstFuncParamDef, AstProcDef, AstRoot,
    AstTopLevel, BlockType, Referenced, TreeFuncParamDef, TreeRoot, TreeTopLevel, Wrapped,
};

impl<'d> Analyzer<'d> {
    #[inline]
    pub(super) async fn resolve_project_file(
        &'d self,
        file: ProjectFile<TreeFile>,
    ) -> ProjectFile<AnalyzedFile<'d>> {
        ProjectFile {
            src: file.src,
            path: file.path,
            hash: file.hash,
            resolution: self.resolve_file(file.resolution.root, file.path).await,
        }
    }
    pub(super) async fn resolve_file(&'d self, root: TreeRoot, file: Spur) -> AnalyzedFile<'d> {
        let mut errs = Vec::new();
        let mut ast_top: Vec<AstTopLevel<'d>> = Vec::with_capacity(root.top_statements.len());
        for top in root.top_statements {
            ast_top.push(match top {
                TreeTopLevel::Event(e) => {
                    let blocktype: BlockType = e.type_tok.data.clone().into();
                    let action = self.dump.search_action_spur(
                        self.resolver,
                        e.name.data.inner,
                        blocktype.caps(),
                    );
                    if action.is_none() {
                        let reference = Referenced::new(
                            e.name.clone().map_inner(|i| {
                                ActionReference::new(BlockType::PlayerEvent, i.inner)
                            }),
                            file,
                        );

                        let suggestions = self.dump.suggest_actions(&reference, self.resolver);

                        errs.push(SemanticError::EventNotFound(ActionNotFoundError {
                            token: reference,
                            suggestions,
                        }))
                    }
                    AstTopLevel::Event(AstEvent {
                        type_tok: e.type_tok,
                        name: e.name,
                        action,
                        statements: self.statements(e.statements, file).await,
                        end_tok: e.end_tok,
                    })
                }
                TreeTopLevel::FuncDef(f) => AstTopLevel::FuncDef(AstFuncDef {
                    type_tok: f.type_tok,
                    name: f.name,
                    params: self.inputs_params(f.params, file).await,
                    statements: self.statements(f.statements, file).await,
                    end_tok: f.end_tok,
                }),
                TreeTopLevel::ProcDef(p) => AstTopLevel::ProcDef(AstProcDef {
                    type_tok: p.type_tok,
                    name: p.name,
                    statements: self.statements(p.statements, file).await,
                    end_tok: p.end_tok,
                }),
                TreeTopLevel::Recovery(_) => panic!("Recovery shouldn't appear in semantic"),
            });
        }

        AnalyzedFile::new(AstRoot {
            top_statements: ast_top,
        })
    }

    pub(super) async fn inputs_params(
        &self,
        params: Wrapped<TreeFuncParamDef>,
        file: Spur,
    ) -> Wrapped<AstFuncParamDef> {
        todo!()
    }
}
