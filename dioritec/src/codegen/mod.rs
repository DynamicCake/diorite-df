//! Moule for the code generation step

use std::sync::Arc;

use block::{Arguments, Block, CodeBlock, GeneratedCode};
use lasso::RodeoResolver;

use crate::{
    ast::{top::AstTopLevel, AstRoot},
    dump::ActionDump,
    project::{analyzed::CheckedProjectFiles, Project, ProjectFile},
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
        Self { programs: files, resolver }
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
                blocks.push(CodeBlock::Block(Block {
                    block: "func",
                    action: self.resolver.resolve(&func.name.data.inner),
                    args: Arguments { items: todo!() },
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
        GeneratedCode { blocks }
    }
}
