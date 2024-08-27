//! Despite it's name, it is actually for the Parse Tree and the AST

use serde::Serialize;

use crate::lexer::Token;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq, Serialize)]
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
