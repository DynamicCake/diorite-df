// TODO: Implement analysis

// impl Project<ParsedProjectFiles> {
//     pub async fn analyze(self) -> Project<AnalyzedProjectFiles> {
//     }
// }

use crate::ast::AstRoot;

use super::{FileResolution, ProjectFiles, StarterSet};

// Semantic Checking
#[derive(Debug)]
pub struct AnalyzedFile<'resources> {
    starters: StarterSet,
    programs: AnalyzedProjectFiles<'resources>,
}

impl FileResolution for AnalyzedFile<'_> {}
#[derive(Debug, PartialEq)]
pub struct AnalyzedProjectFiles<'d> {
    pub programs: Vec<AstRoot<'d>>,
}
impl ProjectFiles for AnalyzedProjectFiles<'_> {}
