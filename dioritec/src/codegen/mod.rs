//! Moule for the code generation step

use std::sync::Arc;

use block::{Arguments, Block, CodeBlock, GeneratedCode, Item};
use data::{
    ChestBlockTag, ChestFunctionParam, ChestGameValue, ChestLocation, ChestNumber, ChestParticle,
    ChestPotion, ChestSound, ChestString, ChestStyledText, ChestValue, ChestVariable, ChestVec3D,
};
use lasso::RodeoResolver;
use lasso::Spur;

use crate::{
    ast::{prelude::*, top::AstTopLevel},
    dump::Argument,
    project::ProjectFile,
    semantic::AnalyzedFile,
};

pub mod block;
pub mod data;
pub mod hcp;
pub mod test;

pub struct CodeGenerator {
    pub programs: Vec<ProjectFile<AnalyzedFile>>,
    resolver: Arc<RodeoResolver>,
}

impl<'src> CodeGenerator {
    pub fn new(files: Vec<ProjectFile<AnalyzedFile>>, resolver: Arc<RodeoResolver>) -> Self {
        Self {
            programs: files,
            resolver,
        }
    }

    pub fn stringify(templates: Vec<GeneratedCode<'src>>) -> Vec<String> {
        templates
            .iter()
            .map(|template| serde_json::to_string(template).expect("Serialization shouldn't fail"))
            .collect()
    }
    pub fn generate(&'src self) -> Vec<GeneratedCode<'src>> {
        let mut templates = Vec::new();
        for file in &self.programs {
            let root = &file.resolution.root.top_statements;
            for top in root {
                templates.push(self.gen_top(top));
            }
        }
        // self.program.resolution.root.top_statements;
        templates
    }
    fn gen_top(&'src self, root: &AstTopLevel) -> GeneratedCode<'src> {
        let mut blocks = Vec::new();

        // TODO: Make use of the statements, the bulk of the program
        let stmts = match root {
            AstTopLevel::Event(event) => {
                blocks.push(CodeBlock::Block(Block {
                    block: "event",
                    action: self.resolver.resolve(&event.name.data.inner),
                    args: Arguments { items: Vec::new() },
                }));
                &event.statements
            }
            AstTopLevel::FuncDef(func) => {
                let args = func
                    .params
                    .tags
                    .data
                    .items
                    .iter()
                    .enumerate()
                    .map(|(i, arg)| Item {
                        item: ChestValue::FunctionParam {
                            data: ChestFunctionParam {
                                name: self.resolver.resolve(&arg.name.data.inner),
                                // FIXME: this type should be checked earlier in the pipeline
                                typ: self.resolver.resolve(&arg.data_type.data.inner),
                                // TODO: Add these options
                                plural: false,
                                optional: false,
                                description: arg
                                    .description
                                    .as_ref()
                                    .map(|iden| self.resolver.resolve(&iden.data.inner)),
                            },
                        },
                        slot: i as u8,
                    })
                    .collect();

                blocks.push(CodeBlock::Block(Block {
                    block: "func",
                    action: self.resolver.resolve(&func.name.data.inner),
                    args: Arguments { items: args },
                }));
                &func.statements
            }
            AstTopLevel::ProcDef(proc) => {
                blocks.push(CodeBlock::Block(Block {
                    block: "proc",
                    action: self.resolver.resolve(&proc.name.data.inner),
                    args: Arguments { items: Vec::new() },
                }));
                &proc.statements
            }
        };
        let mut stmts = stmts
            .items
            .iter()
            .map(|item| match item {
                AstStatement::Simple(stmt) => CodeBlock::Block(Block {
                    block: stmt.data.type_tok.data.to_str(),
                    action: self.resolver.resolve(&stmt.data.action.data.inner),
                    // jesus christ
                    args: Arguments {
                        items: stmt
                            .data
                            .params
                            .data
                            .tags
                            .data
                            .items
                            .iter()
                            .enumerate()
                            .map(|(i, arg)| Item {
                                item: self.arg_convert(arg),
                                slot: i as u8,
                            })
                            .collect(),
                    },
                }),
                AstStatement::If(_) => todo!(),
                AstStatement::Repeat(_) => todo!(),
            })
            .collect();
        blocks.append(&mut stmts);

        GeneratedCode { blocks }
    }

    fn arg_convert(&'src self, arg: &AstExpression) -> ChestValue<'src> {
        match arg {
            AstExpression::Variable(var) => ChestValue::Variable {
                data: ChestVariable {
                    name: self.resolver.resolve(&var.name.data),
                    scope: var.scope.clone(),
                },
            },
            AstExpression::Potion(pot) => ChestValue::Potion {
                data: ChestPotion {
                    pot: self.resolver.resolve(&pot.pot),
                    dur: pot.amp,
                    amp: pot.amp,
                },
            },
            AstExpression::GameValue(gval) => ChestValue::GameValue {
                data: ChestGameValue {
                    kind: self.resolver.resolve(&gval.kind),
                    target: gval.target.clone(),
                },
            },
            AstExpression::Particle(part) => ChestValue::Particle {
                data: ChestParticle {
                    particle: self.resolver.resolve(&part.particle),
                    cluster: part.cluster.resolve(),

                    data: Box::new(part.data.resolve(&self.resolver)),
                },
            },
            AstExpression::Sound(sound) => ChestValue::Sound {
                data: ChestSound {
                    sound: self.resolver.resolve(&sound.sound),
                    pitch: sound.pitch,
                    vol: sound.vol,
                },
            },
            AstExpression::Location(loc) => ChestValue::Location {
                data: ChestLocation {
                    is_block: loc.is_block,
                    loc: loc.loc.resolve(),
                },
            },
            AstExpression::Vector(vec3d) => ChestValue::Vector {
                data: ChestVec3D {
                    x: vec3d.x,
                    y: vec3d.y,
                    z: vec3d.z,
                },
            },
            AstExpression::String(str) => ChestValue::String {
                data: ChestString {
                    name: self.resolver.resolve(&str.name),
                },
            },
            AstExpression::StyledText(text) => ChestValue::StyledText {
                data: ChestStyledText {
                    name: self.resolver.resolve(&text.name),
                },
            },
            AstExpression::Number(num) => ChestValue::Number {
                data: ChestNumber { name: num.name },
            },
            AstExpression::BlockTag(tag) => {
                let resolve = |spur: Spur| self.resolver.resolve(&spur);
                ChestValue::BlockTag {
                    data: ChestBlockTag {
                        option: resolve(tag.option),
                        tag: resolve(tag.tag),
                        action: resolve(tag.action),
                        block: tag.block.clone()
                    },
                }
            }
        }
    }
    // #[inline]
    // fn resolve_iden(&'src self, iden: Spanned<Iden>) -> &'src str {
    //     self.resolver.resolve(&iden.data.inner)
    // }
}
