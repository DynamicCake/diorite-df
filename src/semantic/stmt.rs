use super::Analyzer;
use core::panic;
use std::sync::Arc;

use crate::ast::prelude::*;
use crate::common::prelude::*;
use crate::dump::Action;
use crate::error::semantic::{
    ActionNotFoundError, SemanticError, TagKeyNotFoundError, TagValueNotFoundError,
};
use crate::error::semantic::{ActionReference, SelectorNotFound};
use crate::tree::prelude::*;

use lasso::Spur;

impl<'d> Analyzer<'d> {
    pub(super) async fn inputs_params(
        &self,
        params: Wrapped<TreeFuncParamDef>,
        file: Spur,
    ) -> Wrapped<AstFuncParamDef> {
        todo!()
    }

    pub(super) fn tags(
        &'d self,
        tags: Option<Spanned<TreeTags>>,
        main_action: Option<&'d Action>,
        file: Spur,
    ) -> Result<Option<Spanned<AstTags<'d>>>, SemanticError<'d>> {
        let main_action = if let Some(it) = main_action {
            it
        } else {
            // Hear me out: If there is no main_action, there shouldn't be another error, it just
            // makes it harder to find the original error
            return Ok(None);
        };
        if let Some(tags) = tags {
            // This is to prevent too much nesting lol
            let Spanned {
                data: outer_tags,
                span: outer_tags_span,
            } = tags;
            let MaybeSpan {
                data: inner_tags,
                span: inner_maybe_span,
            } = outer_tags.inner_tags;

            let mut ast_tags = Vec::new();
            for tag in inner_tags.items {
                let key_str = self.resolver.resolve(&tag.key.data);
                let value_str = self.resolver.resolve(&tag.value.data);
                // Yes this is On^2, however, there realistically shouldn't be more than 5 tags
                let key = if let Some(it) = main_action.tags.iter().find(|key| key.name == key_str)
                {
                    it
                } else {
                    return Err(SemanticError::TagKeyNotFound(TagKeyNotFoundError {
                        action: main_action,
                        token: Referenced::new(tag.key, file),
                        suggestions: main_action.suggest_tags(key_str),
                    }));
                };

                let value =
                    if let Some(it) = key.options.iter().find(|value| value.name == value_str) {
                        it
                    } else {
                        return Err(SemanticError::TagValueNotFound(TagValueNotFoundError {
                            key,
                            token: Referenced::new(tag.key, file),
                            suggestions: key.suggest_tag_values(key_str),
                        }));
                    };

                ast_tags.push(AstIdenPair {
                    key: tag.key,
                    colon: tag.colon,
                    value: tag.value,
                    tag: key,
                    choice: value,
                })
            }

            let ast_tags = AstTags::new(
                outer_tags.open,
                MaybeSpan::new(Parameters::new(ast_tags), inner_maybe_span),
                outer_tags.close,
            );
            Ok(Some(Spanned::new(ast_tags, outer_tags_span)))
        } else {
            Ok(None)
        }
    }

    // Warning: This function's is very messy and the names are horrible, maybe rename later™?
    pub(super) fn selectors(
        &'d self,
        selection: Option<Spanned<TreeSelection>>,
        main_action: Option<&'d Action>,
        file: Spur,
    ) -> Result<Option<Spanned<AstSelection<'d>>>, SemanticError<'d>> {
        if let Some(span_selection) = selection {
            // Destructure spanned for later reconstruction, not using map_inner(f) because of
            // it is easier to return errors without it
            let Spanned {
                data: tree_selection,
                span: span_selection_span,
            } = span_selection;

            // If (parse tree) selection exists, do some modifications, otherwise, return null
            // Equivilent of Option.map()
            let selection = if let Some(selection_some) = &tree_selection.selection {
                let name = self.resolver.resolve(&selection_some.data);
                // Attempt convert to basic
                let mut selection_inner: ActionSelector<'d> = ActionSelector::basic_from_str(name);
                // It selection is ActionSelector::Other if it doesn't have a usual name
                // either being invalid or a subaction
                if let ActionSelector::Other(_inner) = selection_inner {
                    // Is the main action resolved?
                    if let Some(main_action) = main_action {
                        let mut found_subaction = None;
                        // Find action using sub_action_blocks and name
                        for block_name in &main_action.sub_action_blocks {
                            let block_type = BlockType::from_iden(block_name).expect(&format!(
                                "Actiondump does not contain block {}",
                                block_name
                            ));
                            let found_action = if let Some(action) =
                                self.dump.search_action(name, block_type.caps())
                            {
                                found_subaction = Some(action);
                                break;
                            } else {
                                let reference = Referenced::new(
                                    Spanned::new(
                                        ActionReference::new(block_type, selection_some.data),
                                        span_selection_span,
                                    ),
                                    file,
                                );
                                let suggestions =
                                    self.dump.suggest_actions(&reference, self.resolver);

                                return Err(SemanticError::ActionNotFound(ActionNotFoundError {
                                    token: reference,
                                    suggestions,
                                }));
                            };
                        }
                        selection_inner = ActionSelector::Other(found_subaction);
                        if found_subaction.is_none() {
                            return Err(SemanticError::SelectorNotFound(SelectorNotFound {
                                selector: Referenced::new(
                                    Spanned::new(selection_some.data, span_selection_span),
                                    file,
                                ),
                            }));
                        }
                    };
                };
                Some(Spanned::new(selection_inner, span_selection_span.clone()))
            } else {
                None
            };
            let selection = AstSelection {
                open: tree_selection.open,
                selection,
                close: tree_selection.close,
            };
            Ok(Some(Spanned::new(selection, span_selection_span)))
        } else {
            Ok(None)
        }
    }

    pub(super) async fn statements(&self, stmts: TreeStatements, file: Spur) -> AstStatements<'d> {
        let mut ast = Vec::new();
        let mut errs = Vec::new();

        for stmt in stmts.items {
            ast.push(match stmt {
                TreeStatement::Simple(s) => AstStatement::Simple(s.map_inner(|s| {
                    let block_type: BlockType = s.type_tok.data.clone().into();

                    let main_action = self.dump.search_action_spur(
                        &self.resolver,
                        s.action.data.inner,
                        block_type.caps(),
                    );

                    if let None = main_action {
                        let reference = Referenced::new(
                            s.action
                                .clone()
                                .map_inner(|i| ActionReference::new(block_type, i.inner)),
                            file,
                        );

                        let suggestions = self.dump.suggest_actions(&reference, self.resolver);

                        errs.push(SemanticError::ActionNotFound(ActionNotFoundError {
                            token: reference,
                            suggestions,
                        }))
                    }

                    let selection = match self.selectors(s.selection, main_action, file) {
                        Ok(it) => it,
                        Err(err) => {
                            errs.push(err);
                            None
                        }
                    };

                    let tags = match self.tags(s.tags, main_action, file) {
                        Ok(it) => it,
                        Err(err) => {
                            errs.push(err);
                            None
                        }
                    };

                    AstSimpleStatement {
                        type_tok: s.type_tok,
                        action: s.action,
                        resolved: main_action,
                        selection,
                        tags,
                        params: todo!(),
                    }
                })),
                TreeStatement::If(_) => todo!(),
                TreeStatement::Repeat(_) => todo!(),
                TreeStatement::Recovery => todo!(),
            });
        }
        AstStatements::new(ast)
    }
}
