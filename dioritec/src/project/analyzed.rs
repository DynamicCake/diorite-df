use std::sync::Arc;

use lasso::RodeoResolver;

use crate::{
    error::CompilerError,
    semantic::{AnalyzedFile, Analyzer},
};

use super::{parsed::ParsedProjectFiles, FileResolution, Project, ProjectFile, ProjectFiles};
// TODO: Implement analysis

impl Project<ParsedProjectFiles> {
    pub async fn analyze<'d>(self) -> Project<CheckedProjectFiles> {
        let resolver = self.files.resolver.clone();
        let files = self.files;
        let mut errors = Vec::new();
        files
            .lex_errs
            .into_iter()
            .for_each(|e| errors.push(CompilerError::Lexer(e)));
        files
            .parse_errs
            .into_iter()
            .for_each(|e| errors.push(CompilerError::Parse(e)));
        files
            .eof_errs
            .into_iter()
            .for_each(|e| errors.push(CompilerError::Eof(e)));

        let resolver = files.resolver.clone();
        let extra_resources = self.resources.clone();

        let analyzer = Analyzer::new(resolver.clone(), extra_resources.actiondump.clone());
        let resolved = analyzer.resolve(files.parsed).await;
        resolved
            .errors
            .into_iter()
            .for_each(|err| errors.push(CompilerError::Semantic(err)));
        Project {
            resources: self.resources,
            hash: self.hash,
            metadata: self.metadata,
            files: CheckedProjectFiles {
                programs: resolved.files,
                resolver,
                errors,
            },
        }
    }
}

impl FileResolution for AnalyzedFile {}

#[derive(Debug, PartialEq)]
pub struct CheckedProjectFiles {
    pub programs: Vec<ProjectFile<AnalyzedFile>>,
    pub errors: Vec<CompilerError>,
    pub resolver: Arc<RodeoResolver>,
}
impl ProjectFiles for CheckedProjectFiles {}
