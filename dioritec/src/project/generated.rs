use crate::{codegen::{self, hcp::HypercubeProject, CodeGenerator}, error::CompilerError};

use super::{analyzed::CheckedProjectFiles, Project};

impl Project<CheckedProjectFiles> {
    pub fn generate(self) -> (HypercubeProject, Vec<CompilerError>) {
        let resolver = self.files.resolver.clone();
        let generator = CodeGenerator::new(self.files.programs, resolver);
        let templates = CodeGenerator::stringify(generator.generate());
        (HypercubeProject {
            hypercube_project: "0.1".to_string(),
            metadata: self.metadata,
            lines: templates
        }, self.files.errors)
    }
}
