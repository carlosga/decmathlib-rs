use std::fmt::Formatter;
use std::str::FromStr;
use crate::d128::d128;

#[cfg(feature = "serde")]
impl serde::ser::Serialize for d128 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::ser::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::de::Deserialize<'de> for d128 {
    fn deserialize<D>(deserializer: D) -> Result<d128, D::Error> where D: serde::de::Deserializer<'de> {
        deserializer.deserialize_str(Decimal128Visitor)
    }
}

#[cfg(feature = "serde")]
struct Decimal128Visitor;

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for Decimal128Visitor {
    type Value = d128;

    fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "a d128 value")
    }

    fn visit_str<E>(self, s: &str) -> Result<d128, E>
        where E: serde::de::Error
    {
        use serde::de::Unexpected;
        d128::from_str(s).map_err(|_| E::invalid_value(Unexpected::Str(s), &self))
    }
}