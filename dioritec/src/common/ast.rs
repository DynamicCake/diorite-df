
use enum_assoc::Assoc;
use serde::{Serialize, Serializer};

use super::tree::EventType;

/// A fixed point [i64] with 3 decimal digits of precision
#[derive(Debug, PartialEq)]
pub struct DfNumber(i64);
impl DfNumber {
    /// Constructs a new instance of [DfNumber]
    /// ```
    /// DfNumber::new(12345) // equal to 12.345
    /// ```
    pub fn new(value: i64) -> Self {
        Self(value)
    }
    pub fn value(&self) -> i64 {
        self.0
    }
    pub fn stringify(&self) -> String {
        let value = self.0;
        format!(
            "{}{}.{:0>3}",
            // Monkey brain solution: make number bigger, make range bigger by 1
            if value.is_negative() { "-" } else { "" },
            (value as i128).abs() / 1000i128,
            (value as i128 % 1000i128).abs()
        )
    }
}

impl TryFrom<DfNumber> for f32 {
    type Error = ();

    fn try_from(value: DfNumber) -> Result<Self, Self::Error> {
        // Oh the misery
        value.stringify().parse().map_err(|_| ())
    }
}

impl TryFrom<DfNumber> for f64 {
    type Error = ();

    fn try_from(value: DfNumber) -> Result<Self, Self::Error> {
        // Oh the misery
        value.stringify().parse().map_err(|_| ())
    }
}

impl Serialize for DfNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let num = self.stringify();
        serializer.serialize_str(&num)
    }
}

impl TryFrom<&str> for DfNumber {
    type Error = DfNumberParseError;

    // 9223372036854775.808
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut negative = false;
        let mut total: i64 = 0;
        let mut chars = value.chars().peekable();

        if let Some(minus) = chars.peek() {
            if *minus == '-' {
                chars.next();
                total = total.checked_neg().expect("0 is not MIN");
                negative = true; // this is to allow it to be negated when the value is 0 (always)
            }
        } else {
            return Err(DfNumberParseError::EmptyInput);
        }
        let abs_add = |num: i64, val: i64| {
            if negative {
                num.checked_sub(val)
            } else {
                num.checked_add(val)
            }
        };

        while {
            let peek = chars.peek();
            if let Some(it) = peek {
                it.is_ascii_digit()
            } else {
                false
            }
        } {
            let next = chars.next().expect("is_some passed before");
            total = if let Some(it) = total.checked_mul(10) {
                it
            } else {
                return Err(DfNumberParseError::TooBig);
            };
            total = if let Some(it) = abs_add(
                total,
                next.to_digit(10).expect("Is digit passed before").into(),
            ) {
                it
            } else {
                return Err(DfNumberParseError::TooBig);
            };
        }

        let mut digit_count = 0u32;
        if let Some(next) = chars.next() {
            if next == '.' {
                while {
                    let peek = chars.peek();
                    if let Some(it) = peek {
                        it.is_ascii_digit()
                    } else {
                        false
                    }
                } {
                    digit_count += 1;
                    if digit_count > 3 {
                        return Err(DfNumberParseError::TooPercise);
                    }
                    let next = chars.next().expect("is_some passed before");
                    total = if let Some(it) = total.checked_mul(10) {
                        it
                    } else {
                        return Err(DfNumberParseError::TooBig);
                    };
                    total = if let Some(it) = abs_add(
                        total,
                        next.to_digit(10).expect("Is digit passed before").into(),
                    ) {
                        it
                    } else {
                        return Err(DfNumberParseError::TooBig);
                    };
                }
            } else {
                return Err(DfNumberParseError::UnexpectedChar);
            }
        };

        total = if let Some(it) = total.checked_mul(10_i64.pow(3 - digit_count)) {
            it
        } else {
            return Err(DfNumberParseError::TooBig);
        };

        Ok(DfNumber(total))
    }
}

#[derive(Debug)]
pub enum DfNumberParseError {
    TooBig,
    TooPercise,
    UnexpectedChar,
    EmptyInput,
}

#[derive(Assoc, PartialEq, Debug)]
#[func(pub const fn caps(&self) -> &'static str)]
#[func(pub const fn iden(&self) -> &'static str)]
pub enum BlockType {
    #[assoc(caps = "PLAYER ACTION")]
    #[assoc(iden = "player_action")]
    PlayerAction,
    #[assoc(caps = "IF PLAYER")]
    #[assoc(iden = "if_player")]
    IfPlayer,
    #[assoc(caps = "START PROCESS")]
    #[assoc(iden = "start_process")]
    StartProcess,
    #[assoc(caps = "CALL FUNCTION")]
    #[assoc(iden = "call_func")]
    CallFunction,
    #[assoc(caps = "CONTROL")]
    #[assoc(iden = "control")]
    Control,
    #[assoc(caps = "SET VARIABLE")]
    #[assoc(iden = "set_var")]
    SetVariable,
    #[assoc(caps = "ENTITY EVENT")]
    #[assoc(iden = "entity_event")]
    EntityEvent,
    #[assoc(caps = "PLAYER EVENT")]
    #[assoc(iden = "event")]
    PlayerEvent,
    #[assoc(caps = "FUNCTION")]
    #[assoc(iden = "func")]
    Function,
    #[assoc(caps = "IF ENTITY")]
    #[assoc(iden = "if_entity")]
    IfEntity,
    #[assoc(caps = "ENTITY ACTION")]
    #[assoc(iden = "entity_action")]
    EntityAction,
    #[assoc(caps = "IF VARIABLE")]
    #[assoc(iden = "if_var")]
    IfVar,
    #[assoc(caps = "SELECT OBJECT")]
    #[assoc(iden = "select_obj")]
    SelectObject,
    #[assoc(caps = "GAME ACTION")]
    #[assoc(iden = "game_action")]
    GameAction,
    #[assoc(caps = "ELSE")]
    #[assoc(iden = "else")]
    Else,
    #[assoc(caps = "PROCESS")]
    #[assoc(iden = "process")]
    Process,
    #[assoc(caps = "REPEAT")]
    #[assoc(iden = "repeat")]
    Repeat,
    #[assoc(caps = "IF GAME")]
    #[assoc(iden = "if_game")]
    IfGame,
}

impl BlockType {
    pub fn from_iden(str: &str) -> Option<Self> {
        Some(match str {
            "player_action" => Self::PlayerAction,
            "if_player" => Self::IfPlayer,
            "start_process" => Self::StartProcess,
            "call_func" => Self::CallFunction,
            "control" => Self::Control,
            "set_var" => Self::SetVariable,
            "entity_event" => Self::EntityEvent,
            "event" => Self::PlayerEvent,
            "func" => Self::Function,
            "if_entity" => Self::IfEntity,
            "entity_action" => Self::EntityAction,
            "if_var" => Self::IfVar,
            "select_obj" => Self::SelectObject,
            "game_action" => Self::GameAction,
            "else" => Self::Else,
            "process" => Self::Process,
            "repeat" => Self::Repeat,
            "if_game" => Self::IfGame,
            _ => return None,
        })
    }
}

impl Serialize for BlockType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.iden())
    }
}

impl From<EventType> for BlockType {
    fn from(value: EventType) -> Self {
        match value {
            EventType::Player => Self::PlayerEvent,
            EventType::Entity => Self::EntityEvent,
        }
    }
}
