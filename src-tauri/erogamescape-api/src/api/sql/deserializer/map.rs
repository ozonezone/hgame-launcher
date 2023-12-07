use serde::de::{self, value::BorrowedStrDeserializer, MapAccess};

use super::{error::DeserializeError, Deserializer};

impl<'a, 'de: 'a> MapAccess<'de> for &'a mut Deserializer<'de> {
    type Error = DeserializeError;
    fn next_key_seed<K>(&mut self, seed: K) -> std::result::Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        let header = self.headers.get(self.index);
        if let Some(header) = header {
            seed.deserialize(BorrowedStrDeserializer::new(header))
                .map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let res = seed.deserialize(&mut **self);
        self.index += 1;
        res
    }
}
