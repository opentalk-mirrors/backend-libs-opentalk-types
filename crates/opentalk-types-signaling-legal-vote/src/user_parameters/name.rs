// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::convert::TryFrom;

use snafu::{ensure, Snafu};

/// Maximum allowed length for a [`Name`].
pub const MAX_NAME_LENGTH: usize = 150;

/// A validated name string with a maximum length constraint.
#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(try_from = "String")
)]
pub struct Name(String);

/// Error when parsing a [`Name`].
#[derive(Debug, Snafu)]
pub enum ParseNameError {
    /// Name exceeds the maximum allowed length.
    #[snafu(display("Name cannot exceed {max_length} characters."))]
    TooLong {
        /// The max length a name is allowed to be.
        max_length: usize,
    },
}

fn ensure_is_valid(s: &str) -> Result<(), ParseNameError> {
    ensure!(
        s.len() <= MAX_NAME_LENGTH,
        TooLongSnafu {
            max_length: MAX_NAME_LENGTH
        }
    );
    Ok(())
}

impl std::str::FromStr for Name {
    type Err = ParseNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ensure_is_valid(s)?;
        Ok(Self(s.to_string()))
    }
}

impl TryFrom<String> for Name {
    type Error = ParseNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        ensure_is_valid(&value)?;
        Ok(Self(value))
    }
}

impl TryFrom<&str> for Name {
    type Error = ParseNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_max_length() {
        assert!(
            Name::try_from("a".repeat(MAX_NAME_LENGTH + 1)).is_err(),
            "Name should be rejected if it exceeds {MAX_NAME_LENGTH} characters"
        );

        assert!(
            Name::try_from("a".repeat(MAX_NAME_LENGTH)).is_ok(),
            "Name should be accepted if it is within the limit"
        );
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn serialization_name() {
        let produced = serde_json::to_value(Name::try_from("Test Name").unwrap()).unwrap();
        let expected = json!("Test Name");

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_name() {
        let produced: Name = serde_json::from_value(json!("Test Name")).unwrap();
        let expected = Name::try_from("Test Name").unwrap();

        assert_eq!(produced, expected);

        let produced: Result<Name, _> =
            serde_json::from_value(json!("a".repeat(MAX_NAME_LENGTH + 1)));

        assert!(produced.is_err());
    }
}
