//! Implementation [`serde serialize`](serde::ser::Serialize) and [`serde deserialize`](serde::de::Deserialize) for [`crate::client::impl_config::ClientConfig`]

use super::ClientConfig;
use serde::de::{Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeStruct, Serializer};
use serde::{de, Deserialize, Serialize};

mod const_names {
    pub(crate) const NAME_STRUCT: &str = "ClientConfig";
    pub(crate) const ALL_FIELDS: &[&str] = &[ CLIENT_DATA, STORAGE_CRYPTO, ORDER_ADDINHD_CRYPTO ];
    pub(crate) const LEN_FIELDS: usize = ALL_FIELDS.len();

    pub(crate) const CLIENT_DATA: &str = "client_data";
    pub(crate) const STORAGE_CRYPTO: &str = "storage_crypto";
    pub(crate) const ORDER_ADDINHD_CRYPTO: &str = "order_adding_crypto";
}

impl Serialize for ClientConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct(const_names::NAME_STRUCT, const_names::LEN_FIELDS)?;

        state.serialize_field(const_names::CLIENT_DATA, &self.client_data)?;
        state.serialize_field(const_names::STORAGE_CRYPTO, &self.storage_crypto.read().unwrap().0)?;
        state.serialize_field(const_names::ORDER_ADDINHD_CRYPTO, &self.order_adding_crypto)?;

        state.end()
    }
}

impl<'de> Deserialize<'de> for ClientConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug)]
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
                            const_names::CLIENT_DATA => Ok(Field::ClientData),
                            const_names::STORAGE_CRYPTO => Ok(Field::StorageCrypto),
                            const_names::ORDER_ADDINHD_CRYPTO => Ok(Field::OrderAddingCrypto),
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
                write!(formatter, "struct {}", const_names::NAME_STRUCT)
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
                println!("222222222222222222222");
                let mut client_data = None;
                let mut storage_crypto = None;
                let mut order_adding_crypto = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::ClientData => {
                            if client_data.is_some() {
                                return Err(de::Error::duplicate_field(const_names::CLIENT_DATA));
                            }
                            client_data = Some(map.next_value()?);
                        }
                        Field::StorageCrypto => {
                            if storage_crypto.is_some() {
                                return Err(de::Error::duplicate_field(const_names::STORAGE_CRYPTO));
                            }
                            storage_crypto = Some(map.next_value()?);
                        }
                        Field::OrderAddingCrypto => {
                            if order_adding_crypto.is_none() {
                                return Err(de::Error::duplicate_field(const_names::ORDER_ADDINHD_CRYPTO));
                            }
                            order_adding_crypto = Some(map.next_value()?);
                        }
                    }
                    println!("key: {:?}", key);
                }

                let client_data =
                    client_data.ok_or_else(|| de::Error::missing_field(const_names::CLIENT_DATA))?;

                let storage_crypto =
                    storage_crypto.ok_or_else(|| de::Error::missing_field(const_names::STORAGE_CRYPTO))?;
                    
                let order_adding_crypto = order_adding_crypto
                    .ok_or_else(|| de::Error::missing_field(const_names::ORDER_ADDINHD_CRYPTO))?;

                Ok(ClientConfig {
                    client_data,
                    storage_crypto,
                    order_adding_crypto,
                })
            }
        }

        const FIELDS: &[&str] = const_names::ALL_FIELDS;
        deserializer.deserialize_struct(const_names::NAME_STRUCT, FIELDS, ClientConfigVisitor)
    }
}

#[cfg(test)]
mod tests {
    use api_lower_level::client::impl_crypto::ecdh::{EphemeralSecret, EphemeralSecretDef};
    use fcore::rand::get_crypto_rand;

    use super::*;

    #[test]
    fn deserialize() {
        let mut test_data = ClientConfig::default();
        unsafe {
        test_data.order_adding_crypto.insert("sa".to_string(), EphemeralSecretDef::from(EphemeralSecret::random(&mut get_crypto_rand())));
        }
        let json = serde_json::to_string_pretty(&test_data).unwrap();

        println!("{}", json);

        let data_from_json: ClientConfig = serde_json::from_str(&json).unwrap();

        // TODO BUG
        assert_eq!(data_from_json.client_data, test_data.client_data);
    }

    #[test]
    fn serialize() {
    todo!() // TODO
    }
}