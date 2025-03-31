use crate::codegen::{self, hcp::HypercubeProject, CodeGenerator};

use super::{analyzed::CheckedProjectFiles, Project};

impl Project<CheckedProjectFiles> {
    pub fn generate(self) -> HypercubeProject {
        let resolver = self.files.resolver.clone();
        let generator = CodeGenerator::new(self.files, resolver);
        let templates = CodeGenerator::stringify(generator.generate());
        HypercubeProject {
            hypercube_project: "0.1".to_string(),
            metadata: self.metadata,
            lines: templates
        }
    }
}
