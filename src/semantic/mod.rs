use std::sync::Arc;

use crate::{dump::ActionDump, tree::Program};

pub struct Analyzer<'pro> {
    pub dump: Arc<ActionDump>,
    pub program: &'pro Program,
}

impl<'pro> Analyzer<'pro> {
    pub fn new(dump: Arc<ActionDump>, program: &'pro Program) -> Self {
        Self { dump, program }
    }

    pub fn resolve(self) -> String {
        "".to_string()
    }
}
