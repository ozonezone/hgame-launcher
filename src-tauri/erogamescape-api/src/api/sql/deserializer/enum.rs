use serde::de::{self, value::BorrowedStrDeserializer, EnumAccess};

use super::{error::DeserializeError, Deserializer};

pub struct UnitOnlyVariantAccess;

impl<'de> de::VariantAccess<'de> for UnitOnlyVariantAccess {
    type Error = DeserializeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        Err(Self::Error::Unsupported("newtype variant".to_string()))
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::Unsupported("tuple variant".to_string()))
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::Unsupported("struct variant".to_string()))
    }
}

impl<'a, 'de: 'a> EnumAccess<'de> for &'a mut Deserializer<'de> {
    type Error = DeserializeError;
    type Variant = UnitOnlyVariantAccess;

    fn variant_seed<V>(self, seed: V) -> std::result::Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let content = self
            .contents
            .get(self.index)
            .ok_or_else(|| DeserializeError::Parse("No content".to_string()))?;

        let variant = seed.deserialize(BorrowedStrDeserializer::new(content))?;
        Ok((variant, UnitOnlyVariantAccess))
    }
}
