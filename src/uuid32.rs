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
        Uuid32(Uuid::nil())
    }

    pub fn max() -> Self {
        Uuid32(Uuid::max())
    }

    pub fn inner(&self) -> Uuid {
        self.0
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

fn decode(value: &str) -> Result<Uuid32, DecodeError> {
    match CROCKFORD_LOWER.decode_uuid_str(value) {
        Ok(uuid) => Ok(Uuid32(uuid)),
        Err(e) => match Uuid::try_parse(value) {
            Ok(uuid) => Ok(Uuid32(uuid)),
            Err(_) => Err(e),
        },
    }
}

impl<'a> TryFrom<&'a str> for Uuid32 {
    type Error = DecodeError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        decode(value)
    }
}

impl<'a> TryFrom<&'a String> for Uuid32 {
    type Error = DecodeError;

    fn try_from(value: &'a String) -> Result<Self, Self::Error> {
        decode(value)
    }
}

impl TryFrom<String> for Uuid32 {
    type Error = DecodeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        decode(&value)
    }
}

impl FromStr for Uuid32 {
    type Err = DecodeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        decode(value)
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
                match CROCKFORD_LOWER.decode_uuid_str(value) {
                    Ok(uuid) => Ok(Uuid32(uuid)),
                    Err(e) => match Uuid::try_parse(value) {
                        Ok(uuid) => Ok(Uuid32(uuid)),
                        Err(_) => Err(e).map_err(de_error),
                    },
                }
            }
        }

        deserializer.deserialize_str(Uuid32Visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn try_from() {
        let u = Uuid::from_u128(2107409058113878995600607032774715894);
        let expected = Ok(Uuid32(u));

        let s1 = "1jqfmkr26e00swayb1r4rytfp";
        assert_eq!(Uuid32::try_from(s1), expected);
        assert_eq!(Uuid32::try_from(s1.to_owned()), expected);
        assert_eq!(Uuid32::try_from(&s1.to_owned()), expected);
        assert_eq!(Uuid32::from_str(s1), expected);
        let j = json!(Uuid32::try_from(s1).unwrap());
        assert_eq!(j.as_str().unwrap(), s1);

        let s2 = "0195df49-e046-7001-9e2b-cb0e098f69f6";
        assert_eq!(Uuid32::try_from(s2), expected);
        assert_eq!(Uuid32::try_from(s2.to_owned()), expected);
        assert_eq!(Uuid32::try_from(&s2.to_owned()), expected);
        assert_eq!(Uuid32::from_str(s2), expected);
        let j = json!(Uuid32::try_from(s2).unwrap());
        assert_eq!(j.as_str().unwrap(), s1); // not s2

        let s3 = "?!";
        assert!(Uuid32::try_from(s3).is_err());
        assert!(Uuid32::try_from(s3.to_owned()).is_err());
        assert!(Uuid32::try_from(&s3.to_owned()).is_err());
        assert!(Uuid32::from_str(s3).is_err());
    }
}
