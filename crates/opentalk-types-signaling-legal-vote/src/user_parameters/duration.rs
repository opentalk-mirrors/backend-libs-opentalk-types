// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::convert::TryFrom;

use snafu::{ensure, Snafu};

/// Minimum allowed length for a [`Duration`].
pub const MIN_DURATION_LENGTH: u64 = 5;

/// A validated duration with a minimum length constraint.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(try_from = "u64")
)]
pub struct Duration(u64);

/// Error when parsing a [`Duration`].
#[derive(Debug, Snafu)]
pub enum TryFromDurationError {
    /// Duration is shorter than the required minimum.
    #[snafu(display("Duration must be at least {min_length}."))]
    TooShort {
        /// The minimum length the duration has to be.
        min_length: u64,
    },
}

impl TryFrom<u64> for Duration {
    type Error = TryFromDurationError;

    /// Converts a `u64` into a [`Duration`], enforcing the minimum length.
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        ensure!(
            value >= MIN_DURATION_LENGTH,
            TooShortSnafu {
                min_length: MIN_DURATION_LENGTH
            }
        );
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duration_lenght_min() {
        assert!(
            Duration::try_from(MIN_DURATION_LENGTH - 1).is_err(),
            "Duration should be at least {MIN_DURATION_LENGTH}."
        );

        assert!(
            Duration::try_from(MIN_DURATION_LENGTH).is_ok(),
            "Duration should be accepted if it is equal to {MIN_DURATION_LENGTH}."
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
        let produced = serde_json::to_value(Duration::try_from(10).unwrap()).unwrap();
        let expected = json!(10);

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization() {
        let produced: Duration = serde_json::from_value(json!(10)).unwrap();
        let expected = Duration::try_from(10).unwrap();
        assert_eq!(produced, expected);

        let produced: Result<Duration, _> = serde_json::from_value(json!(4));
        assert!(produced.is_err());
    }
}
