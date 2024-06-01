use serde::{Deserialize, Deserializer};

pub(crate) fn deserialize_u32_string<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<u32, D::Error> {
    let value = String::deserialize(deserializer)?;
    value.parse().map_err(serde::de::Error::custom)
}

pub(crate) fn deserialize_f32_string<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<f32, D::Error> {
    let value = String::deserialize(deserializer)?;
    value.parse().map_err(serde::de::Error::custom)
}

pub(crate) fn deserialize_with_trim<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<String, D::Error> {
    let value = String::deserialize(deserializer)?;
    Ok(value.trim().to_string())
}

pub(crate) fn deserialize_empty<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<bool, D::Error> {
    let value = String::deserialize(deserializer)?;
    Ok(!value.trim().is_empty())
}

pub(crate) fn deserialize_optional_string<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    let binding = String::deserialize(deserializer)?;
    let value = binding.trim();
    if value.is_empty() {
        Ok(None)
    } else {
        Ok(Some(value.to_string()))
    }
}
