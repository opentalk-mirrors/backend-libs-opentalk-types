// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Signaling custom cancel reason for the `legal-vote` namespace.

use snafu::{Snafu, ensure};

/// The maximum allowed length for a custom cancel reason.
pub const MAX_CUSTOM_CANCEL_REASON_LENGTH: usize = 255;

/// A custom reason for canceling a vote.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, derive_more::Display)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(try_from = "String")
)]
pub struct CustomCancelReason(String);

/// Error type for parsing a [`CustomCancelReason`].
#[derive(Debug, Snafu)]
pub enum ParseCustomCancelReasonError {
    /// The input string exceeds the maximum allowed length [`MAX_CUSTOM_CANCEL_REASON_LENGTH`].
    #[snafu(display("CustomCancelReason should not be longer than {max_length} characters"))]
    TooLong {
        /// The maximum allowed length for a custom cancel reason.
        max_length: usize,
    },
}

fn ensure_is_valid(s: &str) -> Result<(), ParseCustomCancelReasonError> {
    ensure!(
        s.len() <= MAX_CUSTOM_CANCEL_REASON_LENGTH,
        TooLongSnafu {
            max_length: MAX_CUSTOM_CANCEL_REASON_LENGTH
        }
    );
    Ok(())
}

impl std::str::FromStr for CustomCancelReason {
    type Err = ParseCustomCancelReasonError;

    fn from_str(s: &str) -> Result<Self, ParseCustomCancelReasonError> {
        ensure_is_valid(s)?;
        Ok(Self(s.to_string()))
    }
}

impl TryFrom<String> for CustomCancelReason {
    type Error = ParseCustomCancelReasonError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        ensure_is_valid(&value)?;
        Ok(Self(value))
    }
}

impl TryFrom<&str> for CustomCancelReason {
    type Error = ParseCustomCancelReasonError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cancel_reason_max_length() {
        assert!(
            CustomCancelReason::try_from("a".repeat(MAX_CUSTOM_CANCEL_REASON_LENGTH)).is_ok(),
            "The CustomCancelReason should be allowed to be {MAX_CUSTOM_CANCEL_REASON_LENGTH} chars long."
        );

        assert!(
            CustomCancelReason::try_from("a".repeat(MAX_CUSTOM_CANCEL_REASON_LENGTH + 1)).is_err(),
            "The CustomCancelReason should not be allowed to be longer than {MAX_CUSTOM_CANCEL_REASON_LENGTH}."
        );
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn serialization() {
        let produced =
            serde_json::to_value(CustomCancelReason::try_from("Test Reason").unwrap()).unwrap();
        let expected = json!("Test Reason");
        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization() {
        let produced: CustomCancelReason = serde_json::from_value(json!("Test Reason")).unwrap();
        let expected = CustomCancelReason::try_from("Test Reason").unwrap();
        assert_eq!(produced, expected);

        let produced: Result<CustomCancelReason, _> =
            serde_json::from_value(json!("a".repeat(MAX_CUSTOM_CANCEL_REASON_LENGTH + 1)));
        assert!(produced.is_err());
    }
}
