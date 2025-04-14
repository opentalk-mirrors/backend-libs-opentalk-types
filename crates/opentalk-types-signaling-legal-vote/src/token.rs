// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Signaling token for the `legal-vote` namespace.

use std::{fmt, str::FromStr};

use basen::BASE58;
use snafu::{OptionExt, Snafu};

/// A `u64` token that is Base58-encoded.
#[derive(Debug, Clone, Default, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "redis",
    derive(redis_args::ToRedisArgs, redis_args::FromRedisValue),
    to_redis_args(serde),
    from_redis_value(serde)
)]
pub struct Token(u64);

impl Token {
    /// Creates a new `Token` from a `u64` value.
    pub fn new(v: u64) -> Self {
        Token(v)
    }

    /// Generates a random `Token` (requires `rand` feature).
    #[cfg(feature = "rand")]
    pub fn generate() -> Self {
        use rand::RngCore;
        Self::new(rand::rng().next_u64())
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", BASE58.encode_const_len(&self.0))
    }
}

/// Error that can occurre during the decoding process.
#[derive(Debug, Snafu)]
pub enum ParseTokenError {
    /// Error while decoding the value.
    #[snafu(display("Not a base58-encoded u64 value: {value}."))]
    InvalidEncoding {
        /// The value which failed to decode.
        value: String,
    },
}

impl FromStr for Token {
    type Err = ParseTokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = BASE58.decode_const_len(s).context(InvalidEncodingSnafu {
            value: s.to_string(),
        })?;
        Ok(Self(v))
    }
}

#[allow(unused)]
struct TokenVisitor;

#[cfg(feature = "serde")]
impl serde::de::Visitor<'_> for TokenVisitor {
    type Value = Token;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A base58-encoded u64 value")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Token::from_str(s).map_err(|e| E::custom(e.to_string()))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Token {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Token {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(TokenVisitor)
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn from_str() {
        assert_eq!(
            Token::from_str("1111Cn8eVZg").unwrap(),
            Token::new(0x68656c6c6f)
        );
    }

    #[test]
    fn serialization() {
        let t = Token::new(0x0);
        assert_eq!(serde_json::to_value(t).unwrap(), json!("11111111111"));

        let t = Token::new(0x30);
        assert_eq!(serde_json::to_value(t).unwrap(), json!("1111111111q"));

        let t = Token::new(0x68656c6c6f);
        assert_eq!(serde_json::to_value(t).unwrap(), json!("1111Cn8eVZg"));
    }

    #[test]
    fn deserialization() {
        let t: Token = serde_json::from_value(json!("11111111111")).unwrap();
        assert_eq!(t, Token::new(0));

        let t: Token = serde_json::from_value(json!("1111111111q")).unwrap();
        assert_eq!(t, Token::new(0x30));

        let t: Token = serde_json::from_value(json!("1111Cn8eVZg")).unwrap();
        assert_eq!(t, Token::new(0x68656c6c6f));
    }
}
