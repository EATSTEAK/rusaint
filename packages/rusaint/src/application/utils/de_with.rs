use serde::{Deserialize, Deserializer};

use crate::model::SemesterType;

pub(crate) fn deserialize_u32_string<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<u32, D::Error> {
    let value = String::deserialize(deserializer)?;
    value.trim().parse().map_err(serde::de::Error::custom)
}

pub(crate) fn deserialize_f32_string<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<f32, D::Error> {
    let value = String::deserialize(deserializer)?;
    value.trim().parse().map_err(serde::de::Error::custom)
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

pub(crate) fn deserialize_bool_string<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<bool, D::Error> {
    let value = String::deserialize(deserializer)?;
    Ok(value.trim() == "true")
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

pub(crate) fn deserialize_semester_type<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<SemesterType, D::Error> {
    let value = String::deserialize(deserializer)?;
    match value.trim() {
        "1 학기" | "1학기" => Ok(SemesterType::One),
        "여름학기" | "여름 학기" => Ok(SemesterType::Summer),
        "2 학기" | "2학기" => Ok(SemesterType::Two),
        "겨울학기" | "겨울 학기" => Ok(SemesterType::Winter),
        _ => Err(serde::de::Error::custom("Unknown SemesterType variant")),
    }
}

pub(crate) fn deserialize_comma_u64_string<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<u64, D::Error> {
    let value = String::deserialize(deserializer)?;
    value
        .replace(",", "")
        .trim()
        .parse()
        .map_err(serde::de::Error::custom)
}
