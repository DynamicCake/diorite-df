use std::sync::Arc;

use crate::{dump::ActionDump, tree::Program};

mod block;
mod data;

pub struct CodeGenerator<'pro> {
    pub dump: Arc<ActionDump>,
    pub program: &'pro Program,
    
}

impl<'pro> CodeGenerator<'pro> {
    pub fn new(dump: Arc<ActionDump>, program: &'pro Program) -> Self {
        Self { dump, program }
    }

    pub fn generate(&self) -> String {
        "".to_string()
    }
}





