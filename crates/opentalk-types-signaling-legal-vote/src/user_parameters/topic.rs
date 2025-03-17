// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::convert::TryFrom;

use snafu::{ensure, Snafu};

/// Maximum allowed length for a [`Topic`].
pub const MAX_TOPIC_LENGTH: usize = 500;

/// A validated topic string with a maximum length constraint.
#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(try_from = "String")
)]
pub struct Topic(String);

/// Error when parsing a [`Topic`].
#[derive(Debug, Snafu)]
pub enum ParseTopicError {
    /// Topic exceeds the maximum allowed length.
    #[snafu(display("Topic cannot exceed {max_length} characters."))]
    TooLong {
        /// The max length which should not be exceeded.
        max_length: usize,
    },
}

fn ensure_is_valid(s: &str) -> Result<(), ParseTopicError> {
    ensure!(
        s.len() <= MAX_TOPIC_LENGTH,
        TooLongSnafu {
            max_length: MAX_TOPIC_LENGTH
        }
    );
    Ok(())
}

impl std::str::FromStr for Topic {
    type Err = ParseTopicError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ensure_is_valid(s)?;
        Ok(Self(s.to_string()))
    }
}

impl TryFrom<String> for Topic {
    type Error = ParseTopicError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        ensure_is_valid(&value)?;
        Ok(Self(value))
    }
}

impl TryFrom<&str> for Topic {
    type Error = ParseTopicError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn topic_max_length() {
        assert!(
            Topic::try_from("a".repeat(MAX_TOPIC_LENGTH + 1)).is_err(),
            "Topic should be rejected if it exceeds {MAX_TOPIC_LENGTH} characters"
        );

        assert!(
            Topic::try_from("a".repeat(MAX_TOPIC_LENGTH)).is_ok(),
            "Topic should be accepted if it is within the limit"
        );
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn serialization_topic() {
        let produced = serde_json::to_value(Topic::try_from("Test Topic").unwrap()).unwrap();
        let expected = json!("Test Topic");

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_topic() {
        let produced: Topic = serde_json::from_value(json!("Test Topic")).unwrap();
        let expected = Topic::try_from("Test Topic").unwrap();

        assert_eq!(produced, expected);

        let produced: Result<Topic, _> =
            serde_json::from_value(json!("a".repeat(MAX_TOPIC_LENGTH + 1)));

        assert!(produced.is_err());
    }
}
