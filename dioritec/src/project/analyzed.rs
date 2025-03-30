use crate::{
    error::CompilerError,
    semantic::{AnalyzedFile, Analyzer},
};

use super::{
    parsed::ParsedProjectFiles, FileResolution, Project, ProjectFile, ProjectFiles, StarterSet,
};
// TODO: Implement analysis

impl Project<ParsedProjectFiles> {
    pub async fn analyze<'d>(self) -> Project<CheckedProjectFiles<'d>> {
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

        let analyzer = Analyzer::new(&files.resolver, &self.resources.actiondump);
        let resolved = analyzer.resolve_self(files.parsed).await;
        resolved
            .errors
            .into_iter()
            .for_each(|err| errors.push(CompilerError::Semantic(err)));
        Project {
            resources: self.resources,
            hash: self.hash,
            files: CheckedProjectFiles {
                programs: todo!(),
                errors
            }
        }
    }
}

impl FileResolution for AnalyzedFile<'_> {}

#[derive(Debug, PartialEq)]
pub struct CheckedProjectFiles<'d> {
    pub programs: Vec<ProjectFile<AnalyzedFile<'d>>>,
    pub errors: Vec<CompilerError<'d>>,
}
impl ProjectFiles for CheckedProjectFiles<'_> {}
