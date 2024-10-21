use std::cmp::Eq;
use std::fmt;
use std::str::FromStr;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use uuid::Uuid;

use fast32::{base32::CROCKFORD_LOWER, DecodeError};

#[derive(Default, PartialEq, Clone, Copy, Hash, Eq, PartialOrd, Ord)]
pub struct Uuid32(pub(crate) Uuid);

impl Uuid32 {
    pub fn nil() -> Self {
        Uuid32::from(Uuid::nil())
    }

    pub fn max() -> Self {
        Uuid32::from(Uuid::max())
    }
}

impl fmt::Display for Uuid32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&CROCKFORD_LOWER.encode_uuid(self.0))
    }
}

// https://stackoverflow.com/questions/22243527/how-to-implement-a-custom-fmtdebug-trait
impl fmt::Debug for Uuid32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Uuid32 {} == {}", self, self.0)
    }
}

impl From<Uuid> for Uuid32 {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<Uuid32> for Uuid {
    fn from(value: Uuid32) -> Self {
        value.0
    }
}

impl<'a> TryFrom<&'a str> for Uuid32 {
    type Error = DecodeError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(Self(CROCKFORD_LOWER.decode_uuid_str(value)?))
    }
}

impl<'a> TryFrom<&'a String> for Uuid32 {
    type Error = DecodeError;

    fn try_from(value: &'a String) -> Result<Self, Self::Error> {
        Ok(Self(CROCKFORD_LOWER.decode_uuid_str(value)?))
    }
}

impl TryFrom<String> for Uuid32 {
    type Error = DecodeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(CROCKFORD_LOWER.decode_uuid_str(&value)?))
    }
}

impl FromStr for Uuid32 {
    type Err = DecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(CROCKFORD_LOWER.decode_uuid_str(s)?))
    }
}

// https://serde.rs/impl-serialize.html
impl Serialize for Uuid32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&CROCKFORD_LOWER.encode_uuid(self.0))
    }
}

// https://serde.rs/deserialize-struct.html
// adapted from https://github.com/uuid-rs/uuid/blob/98fc36df4d3f33669d54f1d7b999888f75d8b71f/src/external/serde_support.rs#L40
impl<'de> Deserialize<'de> for Uuid32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        fn de_error<E: de::Error>(e: DecodeError) -> E {
            E::custom(format_args!("Uuid32 parsing failed: {}", e))
        }

        struct Uuid32Visitor;

        impl<'vi> de::Visitor<'vi> for Uuid32Visitor {
            type Value = Uuid32;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "a Crockford base32 string of uuid (u128) bytes")
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Uuid32, E> {
                Ok(Uuid32::from(
                    CROCKFORD_LOWER.decode_uuid_str(value).map_err(de_error)?,
                ))
            }
        }

        deserializer.deserialize_str(Uuid32Visitor)
    }
}
