//! Despite it's name, it is actually for the Parse Tree and the AST

use serde::Serialize;

use crate::lexer::Token;

use super::ast::BlockType;

#[derive(Debug, PartialEq, Clone)]
pub enum EventType {
    Player,
    Entity,
}

#[derive(Debug, PartialEq)]
pub enum IfActionType {
    Player,
    Entity,
    Game,
    Var,
}

impl TryInto<IfActionType> for Token {
    type Error = ();

    fn try_into(self) -> Result<IfActionType, Self::Error> {
        Ok(match self {
            Token::IfPlayer => IfActionType::Player,
            Token::IfEntity => IfActionType::Entity,
            Token::IfGame => IfActionType::Game,
            Token::IfVar => IfActionType::Var,
            _ => return Err(()),
        })
    }
}

impl From<IfActionType> for BlockType {
    fn from(val: IfActionType) -> Self {
        match val {
            IfActionType::Player => BlockType::IfPlayer,
            IfActionType::Entity => BlockType::IfEntity,
            IfActionType::Game => BlockType::IfGame,
            IfActionType::Var => BlockType::IfVar,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    PlayerAction,
    EntityAction,
    GameAction,
    Control,
    CallFunction,
    CallProcess,
    Select,
    Var,
}

impl From<ActionType> for BlockType {
    fn from(val: ActionType) -> Self {
        match val {
            ActionType::PlayerAction => BlockType::PlayerAction,
            ActionType::EntityAction => BlockType::EntityAction,
            ActionType::GameAction => BlockType::GameAction,
            ActionType::Control => BlockType::Control,
            ActionType::CallFunction => BlockType::CallFunction,
            ActionType::CallProcess => BlockType::StartProcess,
            ActionType::Select => BlockType::SelectObject,
            ActionType::Var => BlockType::SetVariable,
        }
    }
}

impl TryInto<ActionType> for Token {
    type Error = ();

    fn try_into(self) -> Result<ActionType, Self::Error> {
        Ok(match self {
            Token::PlayerAction => ActionType::PlayerAction,
            Token::EntityAction => ActionType::EntityAction,
            Token::GameAction => ActionType::GameAction,
            Token::Control => ActionType::Control,
            Token::CallFunction => ActionType::CallFunction,
            Token::CallProcess => ActionType::CallProcess,
            Token::Select => ActionType::Select,
            Token::SetVar => ActionType::Var,
            _ => return Err(()),
        })
    }
}
