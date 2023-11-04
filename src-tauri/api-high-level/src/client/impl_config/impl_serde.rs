use super::ClientConfig;
use serde::de::{Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeStruct, Serializer};
use serde::{de, Deserialize, Serialize};

impl Serialize for ClientConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ClientConfig", 3)?;

        state.serialize_field("client_data", &self.client_data)?;
        state.serialize_field("storage_crypto", &self.storage_crypto.read().unwrap().0)?;
        state.serialize_field("order_adding_crypto", &self.order_adding_crypto)?;

        state.end()
    }
}

impl<'de> Deserialize<'de> for ClientConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            ClientData,
            StorageCrypto,
            OrderAddingCrypto,
        }

        // This part could also be generated independently by:
        //
        //    #[derive(Deserialize)]
        //    #[serde(field_identifier, rename_all = "lowercase")]
        //    enum Field { Secs, Nanos }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        formatter.write_str("field identifier")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "client_data" => Ok(Field::ClientData),
                            "storage_crypto" => Ok(Field::StorageCrypto),
                            "order_adding_crypto" => Ok(Field::OrderAddingCrypto),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ClientConfigVisitor;

        impl<'de> Visitor<'de> for ClientConfigVisitor {
            type Value = ClientConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ClientConfig")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<ClientConfig, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let client_data = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let storage_crypto = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let order_adding_crypto = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;

                Ok(ClientConfig {
                    client_data,
                    storage_crypto,
                    order_adding_crypto,
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<ClientConfig, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut client_data = None;
                let mut storage_crypto = None;
                let mut order_adding_crypto = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::ClientData => {
                            if client_data.is_some() {
                                return Err(de::Error::duplicate_field("client_data"));
                            }
                            client_data = Some(map.next_value()?);
                        }
                        Field::StorageCrypto => {
                            if storage_crypto.is_some() {
                                return Err(de::Error::duplicate_field("storage_crypto"));
                            }
                            storage_crypto = Some(map.next_value()?);
                        }
                        Field::OrderAddingCrypto => {
                            if order_adding_crypto.is_none() {
                                return Err(de::Error::duplicate_field("order_adding_crypto"));
                            }
                            order_adding_crypto = Some(map.next_value()?);
                        }
                    }
                }

                let client_data =
                    client_data.ok_or_else(|| de::Error::missing_field("client_data"))?;
                let storage_crypto =
                    storage_crypto.ok_or_else(|| de::Error::missing_field("storage_crypto"))?;
                let order_adding_crypto = order_adding_crypto
                    .ok_or_else(|| de::Error::missing_field("order_adding_crypto"))?;

                Ok(ClientConfig {
                    client_data,
                    storage_crypto,
                    order_adding_crypto,
                })
            }
        }

        const FIELDS: &[&str] = &["client_data", "storage_crypto", "order_adding_crypto"];
        deserializer.deserialize_struct("ClientConfig", FIELDS, ClientConfigVisitor)
    }
}
