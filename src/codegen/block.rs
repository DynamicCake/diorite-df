use super::data::*;
use arrayvec::ArrayString;
use serde::{Serialize, Serializer};
use serde_json::Number;

#[derive(Serialize)]
pub struct GeneratedCode<'src> {
    blocks: Vec<CodeBlock<'src>>,
}

#[derive(Serialize)]
#[serde(tag = "id", rename_all = "snake_case")]
pub enum CodeBlock<'src> {
    Block(Block<'src>),
    Bracket(Bracket<'src>),
}

#[derive(Serialize)]
pub struct Bracket<'src> {
    direct: &'src str,
    #[serde(rename = "type")]
    kind: BracketKind,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BracketKind {
    Norm,
    Repeat,
}

#[derive(Serialize)]
#[serde(rename = "snake_case")]
pub enum BracketState {
    Open,
    Close,
}

#[derive(Serialize)]
pub struct Block<'src> {
    id: &'static str,
    block: &'static str,
    args: Arguments<'src>,
    action: &'src str,
}

#[derive(Serialize)]
pub struct Arguments<'src> {
    items: Vec<Item<'src>>,
}

#[derive(Serialize)]
pub struct Item<'src> {
    item: Data<'src>,
    slot: u8,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct DfNumber(#[serde(serialize_with = "fixed")] i64);

// WARING: This implementation is very cursed, proceed with caution
impl DfNumber {
    pub fn new(value: i64) -> Result<Self, ()> {
        Ok(Self(value))
    }
}

fn fixed<S>(value: &i64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{:0>4}", value.to_string());
    let (first_part, last_part) = s.split_at(s.len() - 3);
    let val = format!("{}.{}", first_part, last_part);
    println!("{}", val);

    let num: Number = serde_json::from_str(&val).unwrap();
    num.serialize(serializer)
}

impl TryFrom<&str> for DfNumber {
    type Error = ();

    // 9223372036854775.808
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut start: ArrayString<20> = ArrayString::new();
        let mut count: u8 = 17;
        let mut decimal = false;

        for char in value.chars() {
            if count == 17 || char == '-' {
                start.push(char);
                count -= 1;
                continue;
            }

            if count == 0 {
                return Err(());
            }
            count -= 1;

            if char.is_ascii_digit() {
                start.push(char);
                continue;
            } else if char == '.' {
                if decimal == true {
                    return Err(());
                }
                count = 3;
                decimal = true;
            } else {
                return Err(());
            }
        }

        if decimal {
            for _ in 0..count {
                start.push('0');
            }
        }

        println!("{}", start);
        let num = start.parse::<i64>().map_err(|_| ())?;
        Ok(DfNumber(num))
    }
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockType {
    PlayerAction,
    IfPlayer,
    StartProcess,
    CallFunc,
    Control,
    SetVar,
    EntityEvent,
    Event,
    Func,
    IfEntity,
    EntityAction,
    IfVar,
    SelectObj,
    GameAction,
    Else,
    Process,
    Repeat,
    IfGame,
}
