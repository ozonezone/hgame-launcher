mod r#enum;
pub mod error;
mod map;

use serde::de::{self, Visitor};
use serde::forward_to_deserialize_any;

use error::{DeserializeError, Result};

pub struct Deserializer<'de> {
    headers: &'de [String],
    contents: &'de [String],
    index: usize,
}

impl<'de> Deserializer<'de> {
    pub fn new(headers: &'de [String], contents: &'de [String]) -> Self {
        Deserializer {
            contents,
            headers,
            index: 0,
        }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = DeserializeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let content = self
            .contents
            .get(self.index)
            .ok_or_else(|| DeserializeError::Parse("No content".to_string()))?;
        let v = if content == "t" {
            visitor.visit_bool(true)
        } else if content == "f" {
            visitor.visit_bool(false)
        } else if let Ok(num) = content.parse::<i32>() {
            visitor.visit_i32(num)
        } else if let Ok(num) = content.parse::<f64>() {
            visitor.visit_f64(num)
        } else if content.is_empty() {
            visitor.visit_none()
        } else {
            visitor.visit_string(content.clone())
        };

        v
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    fn deserialize_map<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(self)
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        match self.contents.get(self.index) {
            None => visitor.visit_none(),
            Some(f) if f.is_empty() => visitor.visit_none(),
            Some(_) => visitor.visit_some(self),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let content = self
            .contents
            .get(self.index)
            .ok_or_else(|| DeserializeError::Parse("No content".to_string()))?;
        visitor.visit_string(content.clone())
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str
        bytes byte_buf unit unit_struct newtype_struct seq tuple
        tuple_struct identifier ignored_any
    }
}
