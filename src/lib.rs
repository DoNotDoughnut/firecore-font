use serde::{Deserialize, Serialize};

pub type MessagePages = Vec<MessagePage>;
pub type Lines = Vec<String>; // maybe use cow<'_ str>

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    pub pages: MessagePages,

    #[serde(default)]
    pub color: TextColor,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessagePage {
    pub lines: Lines,
    pub wait: Option<f32>,
}

#[derive(Debug, Copy, Clone, Hash, Deserialize, Serialize)]
pub enum TextColor {
    Black,
    White,
    Gray,
    Red,
    Blue,
}

impl Default for TextColor {
    fn default() -> Self {
        Self::Black
    }
}