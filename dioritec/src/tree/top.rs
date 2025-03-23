use crate::{error::semantic::DuplicateLineStarter, tree::prelude::*};
use lasso::Spur;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum TreeTopLevel {
    Event(TreeEvent),
    FuncDef(TreeFuncDef),
    ProcDef(TreeProcDef),
    Recovery(TopLevelRecovery),
}

impl TreeTopLevel {
    pub fn add_starter(
        &self,
        file: Spur,
        starters: &mut StarterSet,
    ) -> Result<(), DuplicateLineStarter> {
        let starter = |hashset: &mut HashSet<Starter>, name: &Spanned<Iden>| {
            let thing = Starter::new(Referenced::new(name.to_owned(), file));
            if hashset.contains(&thing) {
                let thing = thing.clone();
                let replaced = hashset.replace(thing.clone()).expect("Should be some");
                Err(DuplicateLineStarter {
                    original: replaced.0.to_empty(),
                    doppelganger: thing.0.to_empty(),
                })
            } else {
                hashset.insert(thing);
                Ok(())
            }
        };
        match self {
            TreeTopLevel::Event(it) => match it.type_tok.data {
                EventType::Player => starter(&mut starters.player_event, &it.name),
                EventType::Entity => starter(&mut starters.entity_event, &it.name),
            },
            TreeTopLevel::FuncDef(it) => starter(&mut starters.function, &it.name),
            TreeTopLevel::ProcDef(it) => starter(&mut starters.process, &it.name),
            TreeTopLevel::Recovery(_) => panic!("What is a TreeTopLevel::Recovery doing here"),
        }
    }
}

// Function
#[derive(Debug, PartialEq)]
pub struct TreeFuncDef {
    pub type_tok: Spanned<()>,
    pub name: Spanned<Iden>,
    pub params: Wrapped<TreeFuncParamDef>,
    pub statements: TreeStatements,
    pub end_tok: Spanned<()>,
}

#[derive(Debug, PartialEq)]
pub struct TreeFuncParamDef {
    pub name: Spanned<Iden>,
    pub colon: Spanned<()>,
    pub data_type: Spanned<Iden>,
    pub description: Option<Spanned<Iden>>,
}

impl SpanStart for TreeFuncParamDef {
    fn start(&self) -> SpanSize {
        self.name.span.start
    }
}

impl SpanEnd for TreeFuncParamDef {
    fn end(&self) -> SpanSize {
        let desc = &self.description;
        match desc {
            Some(it) => it.span.end,
            None => self.data_type.span.end,
        }
    }
}

// Process
#[derive(Debug, PartialEq)]
pub struct TreeProcDef {
    pub type_tok: Spanned<()>,
    pub name: Spanned<Iden>,
    pub statements: TreeStatements,
    pub end_tok: Spanned<()>,
}

// Event
#[derive(Debug, PartialEq)]
pub struct TreeEvent {
    pub type_tok: Spanned<EventType>,
    pub name: Spanned<Iden>,
    pub statements: TreeStatements,
    pub end_tok: Spanned<()>,
}

impl TreeEvent {
    pub fn new(
        type_tok: Spanned<EventType>,
        name: Spanned<Iden>,
        statements: TreeStatements,
        end_tok: Spanned<()>,
    ) -> Self {
        Self {
            type_tok,
            name,
            statements,
            end_tok,
        }
    }
}
