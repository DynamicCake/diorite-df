use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionDump {
    pub codeblocks: Vec<Codeblock>,
    pub actions: Vec<Action>,
    pub game_value_categories: Vec<GameValueCategory>,
    pub game_values: Vec<GameValue>,
    pub particle_categories: Vec<ParticleCategory>,
    pub particles: Vec<Particle>,
    pub sound_categories: Vec<SoundCategory>,
    pub sounds: Vec<Sound>,
    pub potions: Vec<Potion>,
    pub cosmetics: Vec<Cosmetic>,
    pub shops: Vec<Shop>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Codeblock {
    pub name: String,
    pub identifier: String,
    pub item: Item,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub material: String,
    pub name: String,
    pub deprecated_note: Vec<Value>,
    pub description: Vec<String>,
    pub example: Vec<String>,
    pub works_with: Vec<Value>,
    pub additional_info: Vec<Vec<String>>,
    pub required_rank: String,
    pub require_tokens: bool,
    pub require_rank_and_tokens: bool,
    pub advanced: bool,
    pub loaded_item: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub name: String,
    pub codeblock_name: String,
    pub tags: Vec<Tag>,
    pub aliases: Vec<String>,
    pub icon: ActionIcon,
    #[serde(default)]
    pub sub_action_blocks: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub name: String,
    pub options: Vec<Choice>,
    pub default_option: String,
    pub slot: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "option")]
pub struct Choice {
    pub name: String,
    pub icon: ActionTagIcon,
    pub aliases: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionTagIcon {
    pub material: String,
    pub name: String,
    pub deprecated_note: Vec<Value>,
    pub description: Vec<String>,
    pub example: Vec<Value>,
    pub works_with: Vec<Value>,
    pub additional_info: Vec<Vec<String>>,
    pub required_rank: String,
    pub require_tokens: bool,
    pub require_rank_and_tokens: bool,
    pub advanced: bool,
    pub loaded_item: String,
    pub head: Option<String>,
    pub color: Option<Color>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    pub red: i64,
    pub green: i64,
    pub blue: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionIcon {
    pub material: String,
    pub name: String,
    pub deprecated_note: Vec<String>,
    pub description: Vec<String>,
    pub example: Vec<String>,
    pub works_with: Vec<String>,
    pub additional_info: Vec<Vec<String>>,
    pub required_rank: String,
    pub require_tokens: bool,
    pub require_rank_and_tokens: bool,
    pub advanced: bool,
    pub loaded_item: String,
    pub cancellable: Option<bool>,
    pub cancelled_automatically: Option<bool>,
    pub color: Option<Color2>,
    pub tags: Option<i64>,
    #[serde(default)]
    pub arguments: Vec<Argument>,
    #[serde(default)]
    pub return_values: Vec<ReturnValue>,
    pub head: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color2 {
    pub red: i64,
    pub green: i64,
    pub blue: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Argument {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub plural: Option<bool>,
    pub optional: Option<bool>,
    #[serde(default)]
    pub description: Vec<String>,
    #[serde(default)]
    pub notes: Vec<Vec<String>>,
    pub text: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReturnValue {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(default)]
    pub description: Vec<String>,
    pub text: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameValueCategory {
    pub identifier: String,
    pub gui_slot: i64,
    pub icon: GameValueCategoryIcon,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameValueCategoryIcon {
    pub material: String,
    pub name: String,
    pub deprecated_note: Vec<Value>,
    pub description: Vec<String>,
    pub example: Vec<Value>,
    pub works_with: Vec<Value>,
    pub additional_info: Vec<Value>,
    pub required_rank: String,
    pub require_tokens: bool,
    pub require_rank_and_tokens: bool,
    pub advanced: bool,
    pub loaded_item: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameValue {
    pub aliases: Vec<String>,
    pub category: String,
    pub icon: GameValueIcon,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameValueIcon {
    pub material: String,
    pub name: String,
    pub deprecated_note: Vec<Value>,
    pub description: Vec<String>,
    pub example: Vec<Value>,
    pub works_with: Vec<String>,
    pub additional_info: Vec<Vec<String>>,
    pub required_rank: String,
    pub require_tokens: bool,
    pub require_rank_and_tokens: bool,
    pub advanced: bool,
    pub loaded_item: String,
    pub return_type: String,
    pub return_description: Vec<String>,
    pub color: Option<Color3>,
    pub head: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color3 {
    pub red: i64,
    pub green: i64,
    pub blue: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleCategory {
    pub particle: String,
    pub icon: ParticleCategoryIcon,
    pub category: Option<String>,
    pub fields: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleCategoryIcon {
    pub material: String,
    pub name: String,
    pub color: Option<Color4>,
    pub deprecated_note: Vec<Value>,
    pub description: Vec<Value>,
    pub example: Vec<Value>,
    pub works_with: Vec<Value>,
    pub additional_info: Vec<Vec<String>>,
    pub required_rank: String,
    pub require_tokens: bool,
    pub require_rank_and_tokens: bool,
    pub advanced: bool,
    pub loaded_item: String,
    pub head: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color4 {
    pub red: i64,
    pub green: i64,
    pub blue: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Particle {
    pub particle: String,
    pub icon: ParticleIcon,
    pub category: Option<String>,
    pub fields: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleIcon {
    pub material: String,
    pub name: String,
    pub color: Option<Color5>,
    pub deprecated_note: Vec<Value>,
    pub description: Vec<Value>,
    pub example: Vec<Value>,
    pub works_with: Vec<Value>,
    pub additional_info: Vec<Vec<String>>,
    pub required_rank: String,
    pub require_tokens: bool,
    pub require_rank_and_tokens: bool,
    pub advanced: bool,
    pub loaded_item: String,
    pub head: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color5 {
    pub red: i64,
    pub green: i64,
    pub blue: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoundCategory {
    pub identifier: String,
    pub icon: SoundCategoryIcon,
    pub has_sub_categories: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoundCategoryIcon {
    pub material: String,
    pub name: String,
    pub head: Option<String>,
    pub deprecated_note: Vec<Value>,
    pub description: Vec<String>,
    pub example: Vec<Value>,
    pub works_with: Vec<Value>,
    pub additional_info: Vec<Value>,
    pub required_rank: String,
    pub require_tokens: bool,
    pub require_rank_and_tokens: bool,
    pub advanced: bool,
    pub loaded_item: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sound {
    pub sound: String,
    pub icon: SoundIcon,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoundIcon {
    pub material: String,
    pub name: String,
    pub deprecated_note: Vec<Value>,
    pub description: Vec<String>,
    pub example: Vec<Value>,
    pub works_with: Vec<Value>,
    pub additional_info: Vec<Value>,
    pub required_rank: String,
    pub require_tokens: bool,
    pub require_rank_and_tokens: bool,
    pub advanced: bool,
    pub loaded_item: String,
    pub head: Option<String>,
    pub color: Option<Color6>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color6 {
    pub red: i64,
    pub green: i64,
    pub blue: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Potion {
    pub potion: String,
    pub icon: PotionIcon,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PotionIcon {
    pub material: String,
    pub name: String,
    pub color: Color7,
    pub deprecated_note: Vec<Value>,
    pub description: Vec<String>,
    pub example: Vec<Value>,
    pub works_with: Vec<Value>,
    pub additional_info: Vec<Value>,
    pub required_rank: String,
    pub require_tokens: bool,
    pub require_rank_and_tokens: bool,
    pub advanced: bool,
    pub loaded_item: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color7 {
    pub red: i64,
    pub green: i64,
    pub blue: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cosmetic {
    pub id: String,
    pub icon: CosmeticIcon,
    pub name: String,
    pub slot: i64,
    pub category: Category,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmeticIcon {
    pub material: String,
    pub name: String,
    pub deprecated_note: Vec<Value>,
    pub description: Vec<Value>,
    pub example: Vec<Value>,
    pub works_with: Vec<Value>,
    pub additional_info: Vec<Value>,
    pub required_rank: String,
    pub require_tokens: bool,
    pub require_rank_and_tokens: bool,
    pub advanced: bool,
    pub loaded_item: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shop {
    pub id: String,
    pub slot: Option<i64>,
    pub name: Option<String>,
    pub icon: Option<ShopIcon>,
    pub purchasables: Vec<Purchasable>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShopIcon {
    pub material: String,
    pub name: String,
    pub deprecated_note: Vec<Value>,
    pub description: Vec<String>,
    pub example: Vec<Value>,
    pub works_with: Vec<Value>,
    pub additional_info: Vec<Value>,
    pub required_rank: String,
    pub require_tokens: bool,
    pub require_rank_and_tokens: bool,
    pub advanced: bool,
    pub loaded_item: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Purchasable {
    pub item: Item2,
    pub id: Option<String>,
    pub price: Option<i64>,
    pub currency_type: Option<String>,
    pub one_time_purchase: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item2 {
    pub material: String,
    pub name: String,
    pub deprecated_note: Vec<Value>,
    pub description: Vec<String>,
    pub example: Vec<Value>,
    pub works_with: Vec<Value>,
    pub additional_info: Vec<Value>,
    pub required_rank: String,
    pub require_tokens: bool,
    pub require_rank_and_tokens: bool,
    pub advanced: bool,
    pub loaded_item: String,
    pub head: Option<String>,
}
