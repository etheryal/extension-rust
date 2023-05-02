//! A library that provides identifiers and namespaced identifiers. Identifiers
//! are unique values that can be used to identify resources, components, and
//! other things. Namespaced identifiers are identifiers that have a namespace
//! to prevent name collisions.
#![deny(missing_docs, clippy::missing_safety_doc)]
use std::fmt;

use bevy_ecs::prelude::*;
use derive_more::{Display, Into};
use getset::{Getters, Setters};
use serde::de::{MapAccess, Visitor};
use serde::{de, Deserialize, Serialize};
use smol_str::SmolStr;
use thiserror::Error;

/// An error that can occur when parsing an [Identifier]
#[derive(Debug, Error, PartialOrd, PartialEq, Eq)]
pub enum IdentifierError {
    /// An error that occurs when an identifier is invalid. Valid identifiers
    /// can only contain lower case letters, numbers, and underscores, and
    /// must start with a letter or underscore.
    #[error(
        "invalid identifier '{0}', identifiers can only contain lower case letters, numbers, and \
         underscores, and must start with a letter or underscore"
    )]
    InvalidIdentifier(SmolStr),

    /// An error that occurs when an identifier is empty
    #[error("empty identifiers are not allowed")]
    EmptyIdentifier,
}

/// An [Identifier] with an additional namespace field to prevent name
/// collisions
#[derive(
    Serialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Component,
    Display,
    Getters,
    Setters,
)]
#[display(fmt = "{namespace}:{value}")]
#[getset(get = "pub", set = "pub")]
pub struct NamespacedIdentifier {
    /// The namespace of this identifier
    namespace: Identifier,

    /// The value itself of this identifier
    value: Identifier,
}

impl NamespacedIdentifier {
    /// Creates a new [NamespacedIdentifier] with the given namespace and value
    #[must_use]
    pub const fn new(namespace: Identifier, value: Identifier) -> Self {
        Self { namespace, value }
    }
}

impl Default for NamespacedIdentifier {
    fn default() -> Self {
        Self::new(Identifier::default(), Identifier::unknown())
    }
}

impl<'de> Deserialize<'de> for NamespacedIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct StringOrStruct;

        #[derive(Deserialize)]
        struct NamedIdentifier {
            namespace: Identifier,
            #[serde(alias = "identifier")]
            value: Identifier,
        }

        impl<'de> Visitor<'de> for StringOrStruct {
            type Value = NamespacedIdentifier;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string or map")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error, {
                NamespacedIdentifier::try_from(value).map_err(de::Error::custom)
            }

            fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>, {
                let id = NamedIdentifier::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(NamespacedIdentifier::new(id.namespace, id.value))
            }
        }

        deserializer.deserialize_any(StringOrStruct)
    }
}

fn split_identifier(identifier: &str) -> Result<NamespacedIdentifier, IdentifierError> {
    let mut split = identifier.split(':').rev();

    let value = split.next().ok_or(IdentifierError::EmptyIdentifier)?;
    let value = Identifier::try_from(value)?;

    let namespace = if let Some(next) = split.next() {
        if split.next().is_some() {
            return Err(IdentifierError::InvalidIdentifier(identifier.into()));
        }
        Identifier::try_from(next)?
    } else {
        Identifier::default()
    };

    Ok(NamespacedIdentifier::new(namespace, value))
}

impl TryFrom<String> for NamespacedIdentifier {
    type Error = IdentifierError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        split_identifier(&value)
    }
}

impl TryFrom<&str> for NamespacedIdentifier {
    type Error = IdentifierError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        split_identifier(value)
    }
}

impl TryFrom<&String> for NamespacedIdentifier {
    type Error = IdentifierError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        split_identifier(value)
    }
}

impl TryFrom<SmolStr> for NamespacedIdentifier {
    type Error = IdentifierError;

    fn try_from(value: SmolStr) -> Result<Self, Self::Error> {
        split_identifier(&value)
    }
}

impl TryFrom<(String, String)> for NamespacedIdentifier {
    type Error = IdentifierError;

    fn try_from((namespace, value): (String, String)) -> Result<Self, Self::Error> {
        let namespace = Identifier::try_from(namespace)?;
        let value = Identifier::try_from(value)?;

        Ok(Self { namespace, value })
    }
}

impl TryFrom<(&str, &str)> for NamespacedIdentifier {
    type Error = IdentifierError;

    fn try_from((namespace, value): (&str, &str)) -> Result<Self, Self::Error> {
        let namespace = Identifier::try_from(namespace)?;
        let value = Identifier::try_from(value)?;

        Ok(Self { namespace, value })
    }
}

impl TryFrom<(&String, &String)> for NamespacedIdentifier {
    type Error = IdentifierError;

    fn try_from((namespace, value): (&String, &String)) -> Result<Self, Self::Error> {
        let namespace = Identifier::try_from(namespace)?;
        let value = Identifier::try_from(value)?;

        Ok(Self { namespace, value })
    }
}

impl From<(Identifier, Identifier)> for NamespacedIdentifier {
    fn from((namespace, value): (Identifier, Identifier)) -> Self {
        Self { namespace, value }
    }
}

/// A named identifier for a registered resource or data in the game.
#[derive(
    Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Display, Into,
)]
#[repr(transparent)]
#[serde(try_from = "SmolStr")]
#[serde(into = "SmolStr")]
pub struct Identifier(SmolStr);

impl Identifier {
    /// An identifier that represents the game itself and its resources.
    pub const ETHERYAL: Self = Self(SmolStr::new_inline("etheryal"));
    /// An identifier that represents an unknown value.
    pub const UNKNOWN: Self = Self(SmolStr::new_inline("unknown"));

    /// Creates a new [Identifier] with the given value.
    pub fn new(value: impl Into<SmolStr>) -> Result<Self, IdentifierError> {
        let value = value.into();

        let mut chars = value.chars();
        let Some(first) = chars.next() else {
            return Err(IdentifierError::EmptyIdentifier);
        };

        if !first.is_ascii_lowercase() && first != '_' {
            return Err(IdentifierError::InvalidIdentifier(value));
        }

        for c in chars {
            if !c.is_ascii_lowercase() && !c.is_ascii_digit() && c != '_' {
                return Err(IdentifierError::InvalidIdentifier(value));
            }
        }

        Ok(Self(value))
    }

    /// An identifier that represents an unknown value.
    pub const fn unknown() -> Self {
        Self::UNKNOWN
    }

    /// Converts the identifier into a &[str].
    #[must_use]
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Default for Identifier {
    fn default() -> Self {
        Self::ETHERYAL
    }
}

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<SmolStr> for Identifier {
    type Error = IdentifierError;

    fn try_from(value: SmolStr) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for Identifier {
    type Error = IdentifierError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for Identifier {
    type Error = IdentifierError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&String> for Identifier {
    type Error = IdentifierError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Identifier> for String {
    fn from(value: Identifier) -> Self {
        value.0.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_identifier() {
        let identifier =
            Identifier::try_from("valid_identifier".to_string()).expect("valid identifier");
        assert_eq!(identifier.as_str(), "valid_identifier");
    }

    #[test]
    fn test_valid_namespace() {
        let identifier = NamespacedIdentifier::try_from("valid:valid_identifier".to_string())
            .expect("valid identifier");

        assert_eq!(identifier.namespace().as_str(), "valid");
        assert_eq!(identifier.value().as_str(), "valid_identifier");
    }

    #[test]
    fn test_valid_without_namespace() {
        let identifier =
            NamespacedIdentifier::try_from("valid_identifier").expect("valid identifier");
        assert_eq!(identifier.value().as_str(), "valid_identifier");
    }

    #[test]
    fn test_invalid_namespace() {
        let identifier = NamespacedIdentifier::try_from("invalid:invalid:identifier");
        assert_eq!(
            identifier,
            Err(IdentifierError::InvalidIdentifier(
                "invalid:invalid:identifier".into()
            ))
        );
    }

    #[test]
    fn test_invalid_identifier() {
        let identifier = Identifier::try_from("InvalidIdentifier");
        assert_eq!(
            identifier,
            Err(IdentifierError::InvalidIdentifier(
                "InvalidIdentifier".into()
            ))
        );
    }

    #[test]
    fn test_empty_identifier() {
        let identifier = Identifier::try_from("".to_string());
        assert_eq!(identifier, Err(IdentifierError::EmptyIdentifier));
    }
}
