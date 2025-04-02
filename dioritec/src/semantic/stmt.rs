use super::Analyzer;
use std::sync::Arc;
use std::vec::IntoIter;

use crate::ast::prelude::*;
use crate::dump::Action;
use crate::error::semantic::{
    ActionNotFoundError, SemanticError, SubActionNotFoundError, SubActionReference,
    TagKeyNotFoundError, TagValueNotFoundError,
};
use crate::error::semantic::{ActionReference, SelectorNotFound};
use crate::tree::prelude::*;

use lasso::Spur;

impl Analyzer {
    pub(super) async fn statements(
        &self,
        stmts: TreeStatements,
        file: Spur,
    ) -> AstStatements {
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

                    if main_action.is_none() {
                        let reference = Referenced::new(
                            s.action
                                .clone()
                                .map_inner(|i| ActionReference::new(block_type, i.inner)),
                            file,
                        );

                        let suggestions = self.dump.suggest_actions(&reference, &self.resolver);

                        errs.push(SemanticError::ActionNotFound(ActionNotFoundError {
                            token: reference,
                            suggestions,
                        }))
                    }

                    let selection = match self.selectors(s.selection, main_action.clone(), file) {
                        Ok(it) => it,
                        Err(err) => {
                            errs.push(err);
                            None
                        }
                    };

                    let tags = match self.tags(s.tags, main_action.clone(), file) {
                        Ok(it) => it,
                        Err(err) => {
                            errs.push(err);
                            None
                        }
                    };

                    // TODO: Find a way to not need to do this
                    let cry = {
                        let data = &s.params.data;
                        Spanned::new(
                            Wrapped {
                                open: data.open.clone(),
                                tags: MaybeSpan::new(
                                    Parameters::new(Vec::new()),
                                    data.tags.span.clone(),
                                ),
                                close: data.close.clone(),
                            },
                            s.params.span.clone(),
                        )
                    };
                    // let tags: Option<Spanned<AstTags>>
                    // let params: Spanned<Wrapped<AstExpression>>
                    let params = match self.action_params(s.params, file) {
                        Ok(it) => it,
                        Err(mut err) => {
                            errs.append(&mut err);
                            cry
                        }
                    };

                    AstSimpleStatement {
                        type_tok: s.type_tok,
                        action: s.action,
                        resolved: main_action,
                        selection,
                        tags,
                        params,
                    }
                })),
                TreeStatement::If(_) => todo!(),
                TreeStatement::Repeat(_) => todo!(),
                TreeStatement::Recovery => todo!(),
            });
        }
        AstStatements::new(ast)
    }

    pub(super) fn tags(
        & self,
        tags: Option<Spanned<TreeTags>>,
        main_action: Option<Arc<Action>>,
        file: Spur,
    ) -> Result<Option<Spanned<AstTags>>, SemanticError> {
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
                let key = if let Some(it) = main_action.tags.iter().find(|key| key.name == key_str)
                {
                    it.clone()
                } else {
                    return Err(SemanticError::TagKeyNotFound(TagKeyNotFoundError {
                        action: main_action.clone(),
                        token: Referenced::new(tag.key, file),
                        suggestions: main_action.suggest_tags(key_str),
                    }));
                };

                let value =
                    if let Some(it) = key.options.iter().find(|value| value.name == value_str) {
                        it.clone()
                    } else {
                        return Err(SemanticError::TagValueNotFound(TagValueNotFoundError {
                            key: key.clone(),
                            token: Referenced::new(tag.key, file),
                            suggestions: key.suggest_tag_values(key_str),
                        }));
                    };

                ast_tags.push(AstIdenPair {
                    key: tag.key,
                    colon: tag.colon,
                    value: tag.value,
                    tag: key,
                    choice: value
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

    pub(super) fn action_params(
        &self,
        params: Spanned<Wrapped<TreeExpression>>,
        file: Spur,
    ) -> Result<Spanned<Wrapped<AstExpression>>, Vec<SemanticError>> {
        let Spanned {
            data: params_outer,
            span: param_outer_span,
        } = params;
        let MaybeSpan {
            data: params_inner,
            span: param_inner_span,
        } = params_outer.tags;

        let mut params = Vec::new();
        let mut errs: Vec<SemanticError> = Vec::new();
        for expr in params_inner.items {
            params.push(self.param(expr, &mut errs, file));
        }

        Ok(Spanned::new(
            Wrapped::new(
                params_outer.open,
                MaybeSpan::new(Parameters::new(params), param_inner_span),
                params_outer.close,
            ),
            param_outer_span,
        ))
    }

    fn param(
        &self,
        expr: TreeExpression,
        errs: &mut Vec<SemanticError>,
        file: Spur,
    ) -> AstExpression {
        match expr {
            TreeExpression::Literal(lit) => match lit {
                TreeStaticLiteral::String(str) => AstExpression::String(AstString {
                    name: str.data.inner,
                }),
                TreeStaticLiteral::Number(num) => {
                    let Spanned {
                        data: num_inner,
                        ref span,
                    } = num;
                    let num = match DfNumber::try_from(self.resolver.resolve(&num_inner.inner)) {
                        Ok(it) => it,
                        Err(err) => {
                            errs.push(SemanticError::from_num(
                                Referenced::new(
                                    Spanned::new(num_inner.inner, span.clone()),
                                    file,
                                ),
                                err,
                            ));
                            DfNumber::new(0)
                        }
                    };

                    AstExpression::Number(AstNumber { name: num })
                }
            },
            TreeExpression::Expr(expr) => {
                let TreeExprLiteral { literal_type, args } = expr;
                let type_str = self.resolver.resolve(&literal_type.data.inner);
                match type_str {
                    "loc" => {
                        let params: IntoIter<TreeExprValue> = args.data.tags.data.items.into_iter();

                        let is_block = true;
                        let loc = ChestLocationData {
                            x: todo!(),
                            y: todo!(),
                            z: todo!(),
                            pitch: todo!(),
                            yaw: todo!(),
                        };
                        AstExpression::Location(AstLocation { is_block, loc });
                    }
                    _ => todo!(),
                }
            }
        }
    }

    // Warning: This function's is very messy and the names are horrible, maybe rename later™?
    pub(super) fn selectors(
        &self,
        selection: Option<Spanned<TreeSelection>>,
        main_action: Option<Arc<Action>>,
        file: Spur,
    ) -> Result<Option<Spanned<AstSelection>>, SemanticError> {
        if let Some(span_selection) = selection {
            // Destructure spanned for later reconstruction, not using map_inner(f)
            // because you cannot `return` errors to escape out of this function
            let Spanned {
                data: tree_selection,
                span: span_selection_span,
            } = span_selection;

            // If (parse tree) selection exists, do some modifications, otherwise, return null
            // Equivilent of Option.map()
            let selection = if let Some(selection_some) = &tree_selection.selection {
                let name = self.resolver.resolve(&selection_some.data);
                // Attempt convert to basic
                let mut selection_inner: ActionSelector = ActionSelector::basic_from_str(name);
                // It selection is ActionSelector::Other if it doesn't have a usual name
                // either being invalid or a subaction
                if let ActionSelector::Other(_) = selection_inner {
                    // Is the main action resolved?
                    if let Some(main_action) = main_action {
                        let mut found_subaction = None;
                        // Find action using sub_action_blocks and name
                        let blocks: Vec<_> = main_action
                            .sub_action_blocks
                            .iter()
                            .map(|block_name| {
                                BlockType::from_iden(block_name).unwrap_or_else(|| {
                                    panic!("Actiondump does not contain block {}", block_name)
                                })
                            })
                            .collect();
                        for block_type in &blocks {
                            if let Some(action) = self.dump.search_action(name, block_type.caps()) {
                                found_subaction = Some(action);
                                break;
                            };
                        }

                        if found_subaction.is_none() {
                            let suggestions = self.dump.suggest_sub_actions(
                                &blocks,
                                selection_some.data,
                                &self.resolver,
                            );
                            let reference = Referenced::new(
                                Spanned::new(
                                    SubActionReference::new(
                                        blocks,
                                        selection_some.data,
                                    ),
                                    span_selection_span.clone(),
                                ),
                                file,
                            );

                            return Err(SemanticError::SubactionNotFound(SubActionNotFoundError {
                                token: reference,
                                suggestions,
                            }));
                        }

                        selection_inner = ActionSelector::Other(found_subaction);
                    } else {
                        return Err(SemanticError::SelectorNotFound(SelectorNotFound {
                            selector: Referenced::new(
                                Spanned::new(selection_some.data, span_selection_span),
                                file,
                            ),
                        }));
                    }
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
}
