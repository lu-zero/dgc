use crate::DgcCert;
use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Serialize,
};
use std::collections::HashMap;

const ISSUER: i64 = 1;
const ISSUED_AT: i64 = 6;
const EXPIRATION_TIME: i64 = 4;
const CERTS: i64 = -260;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct DgcCertContainer {
    #[serde(alias = "1")]
    #[serde(rename(serialize = "1"))]
    pub issuer: String,
    #[serde(alias = "6")]
    #[serde(rename(serialize = "6"))]
    pub issued_at: u64,
    #[serde(alias = "4")]
    #[serde(rename(serialize = "4"))]
    pub expiration_time: u64,
    #[serde(alias = "-260")]
    #[serde(rename(serialize = "-260"))]
    pub certs: HashMap<usize, DgcCert>,
}

impl DgcCertContainer {
    pub fn expand_values(&self) -> Self {
        let mut expanded = self.clone();
        expanded.certs = self
            .certs
            .iter()
            .map(|(id, t)| (*id, t.expand_values()))
            .collect();
        expanded
    }
}

struct DgcCertContainerVisitor;

impl<'de> Visitor<'de> for DgcCertContainerVisitor {
    type Value = DgcCertContainer;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct DgcCertContainer")
    }

    fn visit_map<V>(self, mut map: V) -> Result<DgcCertContainer, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut issuer = None;
        let mut issued_at = None;
        let mut expiration_time = None;
        let mut certs = None;

        while let Some(key) = map.next_key()? {
            match key {
                ISSUER => {
                    if issuer.is_some() {
                        return Err(serde::de::Error::duplicate_field("issuer"));
                    }
                    issuer = Some(map.next_value()?);
                }
                ISSUED_AT => {
                    if issued_at.is_some() {
                        return Err(serde::de::Error::duplicate_field("issued_at"));
                    }
                    issued_at = Some(map.next_value()?);
                }
                EXPIRATION_TIME => {
                    if expiration_time.is_some() {
                        return Err(serde::de::Error::duplicate_field("expiration_time"));
                    }
                    expiration_time = Some(map.next_value()?);
                }
                CERTS => {
                    if certs.is_some() {
                        return Err(serde::de::Error::duplicate_field("certs"));
                    }
                    certs = Some(map.next_value()?);
                }
                _ => {
                    // ignore other fields
                }
            }
        }
        let issuer = issuer.ok_or_else(|| serde::de::Error::missing_field("issuer"))?;
        let issued_at = issued_at.ok_or_else(|| serde::de::Error::missing_field("issued_at"))?;
        let expiration_time =
            expiration_time.ok_or_else(|| serde::de::Error::missing_field("expiration_time"))?;
        let certs = certs.ok_or_else(|| serde::de::Error::missing_field("certs"))?;

        Ok(DgcCertContainer {
            issuer,
            issued_at,
            expiration_time,
            certs,
        })
    }
}

/// Needs a specialized deserializer to be able to deal with keys as integers
impl<'de> Deserialize<'de> for DgcCertContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(DgcCertContainerVisitor)
    }
}
