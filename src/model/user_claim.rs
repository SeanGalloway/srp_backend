use std::collections::HashMap;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub struct UserClaim {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    #[serde(serialize_with = "serialize_as_timestamp")]
    #[serde(deserialize_with = "deserialize_from_timestamp")]
    pub exp: DateTime<Utc>
}

fn serialize_as_timestamp<S>(
    value: &DateTime<Utc>,
    serializer: S
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_i64(value.timestamp())
}

fn deserialize_from_timestamp<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = i64::deserialize(deserializer)?;
    Ok(DateTime::from_timestamp_millis(s).unwrap_or(DateTime::UNIX_EPOCH))
}

impl UserClaim {
    pub fn to_map(&self) -> HashMap<String, Value> {
        let mut claims = HashMap::new();
        claims.insert("id".to_owned(), Value::Number(serde_json::Number::from(self.id.clone())));
        claims.insert("first_name".to_owned(), Value::String(self.first_name.clone()));
        claims.insert("last_name".to_owned(), Value::String(self.last_name.clone()));
        claims.insert("exp".to_owned(), Value::Number(serde_json::Number::from(self.exp.timestamp())));
        claims
    }
}