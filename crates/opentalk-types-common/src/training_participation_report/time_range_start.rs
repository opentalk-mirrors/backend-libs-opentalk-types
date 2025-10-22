// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>

// SPDX-License-Identifier: EUPL-1.2

use snafu::{Snafu, ensure};

use crate::{
    training_participation_report::{MINUTES_PER_HOUR, SECONDS_PER_MINUTE},
    utils::ExampleData,
};

const DEFAULT_VALUE: i64 = 5 * SECONDS_PER_MINUTE;
#[allow(clippy::identity_op)]
const MIN_VALUE: i64 = 1 * SECONDS_PER_MINUTE;
const MAX_VALUE: i64 = 10 * MINUTES_PER_HOUR * SECONDS_PER_MINUTE;

/// Error when parsing a [`TimeRangeStart`].
#[derive(Debug, Snafu)]
pub enum TryFromTimeRangeStartError {
    /// The shortest duration after which a checkpoint can be created is outside of the allowed range.
    #[snafu(display(
        "The shortest duration after which a checkpoint can be created is outside of the allowed range ({min}..={max})"
    ))]
    OutOfRange {
        /// The minimum value of the shortest duration after which a checkpoint can be created, in seconds.
        min: i64,
        /// The maximum value of the shortest duration after which a checkpoint can be created, in seconds.
        max: i64,
    },
}

/// The shortest duration after which a checkpoint can be created, in seconds.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::Display,
    derive_more::AsRef,
    derive_more::Into,
)]
#[cfg_attr(
    feature = "diesel",
    derive(
        opentalk_diesel_newtype::DieselNewtype,
        diesel::expression::AsExpression,
        diesel::deserialize::FromSqlRow
    )
)]
#[cfg_attr(feature="diesel", diesel(sql_type = diesel::sql_types::BigInt))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(try_from = "i64")
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema), schema(example = json!(TimeRangeStart::example_data())))]
pub struct TimeRangeStart(i64);

impl TimeRangeStart {
    /// The minimum value of the type.
    pub const MIN: Self = Self(MIN_VALUE);

    /// The maximum value of the type.
    pub const MAX: Self = Self(MAX_VALUE);

    /// The default value of the type.
    pub const DEFAULT: Self = Self(DEFAULT_VALUE);

    /// Create a new [`TimeRangeStart`] from an i64 value, clamped to the valid range.
    pub const fn from_i64_clamped(value: i64) -> Self {
        let value = if value < MIN_VALUE { MIN_VALUE } else { value };
        let value = if value > MAX_VALUE { MAX_VALUE } else { value };
        Self(value)
    }

    /// Add an offset, saturating.
    pub const fn saturating_add(self, rhs: i64) -> Self {
        Self::from_i64_clamped(self.0.saturating_add(rhs))
    }

    /// Subtract an offset, saturating.
    pub const fn saturating_sub(self, rhs: i64) -> Self {
        Self::from_i64_clamped(self.0.saturating_sub(rhs))
    }
}

impl Default for TimeRangeStart {
    fn default() -> Self {
        Self(DEFAULT_VALUE)
    }
}

impl TryFrom<u64> for TimeRangeStart {
    type Error = TryFromTimeRangeStartError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let value = i64::try_from(value).map_err(|_e| TryFromTimeRangeStartError::OutOfRange {
            min: MIN_VALUE,
            max: MAX_VALUE,
        })?;
        Self::try_from(value)
    }
}

impl TryFrom<usize> for TimeRangeStart {
    type Error = TryFromTimeRangeStartError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let value = i64::try_from(value).map_err(|_e| TryFromTimeRangeStartError::OutOfRange {
            min: MIN_VALUE,
            max: MAX_VALUE,
        })?;
        Self::try_from(value)
    }
}

impl TryFrom<i32> for TimeRangeStart {
    type Error = TryFromTimeRangeStartError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::try_from(i64::from(value))
    }
}

impl TryFrom<i64> for TimeRangeStart {
    type Error = TryFromTimeRangeStartError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        ensure!(
            (MIN_VALUE..=MAX_VALUE).contains(&value),
            OutOfRangeSnafu {
                min: MIN_VALUE,
                max: MAX_VALUE
            }
        );
        Ok(Self(value))
    }
}

impl From<TimeRangeStart> for usize {
    fn from(TimeRangeStart(value): TimeRangeStart) -> Self {
        usize::try_from(value).unwrap()
    }
}

impl ExampleData for TimeRangeStart {
    fn example_data() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq, assert_matches};

    use super::{MAX_VALUE, MIN_VALUE, TimeRangeStart, TryFromTimeRangeStartError};

    #[test]
    fn saturating_add() {
        assert_eq!(
            TimeRangeStart::try_from(142).unwrap().saturating_add(3),
            TimeRangeStart::try_from(145).unwrap()
        );
        assert_eq!(TimeRangeStart::MAX.saturating_add(3), TimeRangeStart::MAX);
    }

    #[test]
    fn saturating_sub() {
        assert_eq!(
            TimeRangeStart::try_from(142).unwrap().saturating_sub(3),
            TimeRangeStart::try_from(139).unwrap()
        );
        assert_eq!(TimeRangeStart::MIN.saturating_sub(3), TimeRangeStart::MIN);
    }

    #[test]
    fn try_from() {
        assert_eq!(
            TimeRangeStart::try_from(114usize).unwrap(),
            TimeRangeStart(114)
        );
        assert_eq!(
            TimeRangeStart::try_from(115i32).unwrap(),
            TimeRangeStart(115)
        );
        assert_eq!(
            TimeRangeStart::try_from(116i64).unwrap(),
            TimeRangeStart(116)
        );
        assert_eq!(
            TimeRangeStart::try_from(118u64).unwrap(),
            TimeRangeStart(118)
        );

        assert_matches!(
            TimeRangeStart::try_from(-42i64),
            Err(TryFromTimeRangeStartError::OutOfRange {
                min: MIN_VALUE,
                max: MAX_VALUE
            })
        );
        assert_matches!(
            TimeRangeStart::try_from((i64::MAX as usize) + 1),
            Err(TryFromTimeRangeStartError::OutOfRange {
                min: MIN_VALUE,
                max: MAX_VALUE
            })
        );
        assert_matches!(
            TimeRangeStart::try_from((i64::MAX as u64) + 1),
            Err(TryFromTimeRangeStartError::OutOfRange {
                min: MIN_VALUE,
                max: MAX_VALUE
            })
        );
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::TimeRangeStart;

    #[test]
    fn serialize_default() {
        let example = TimeRangeStart::default();
        assert_eq!(json!(example), json!(300));
    }

    #[test]
    fn serialize() {
        let example = TimeRangeStart::try_from(500).unwrap();
        assert_eq!(json!(example), json!(500));
    }

    #[test]
    fn deserialize_invalid_zero() {
        assert!(serde_json::from_value::<TimeRangeStart>(json!(0)).is_err());
    }

    #[test]
    fn deserialize_too_low() {
        assert!(serde_json::from_value::<TimeRangeStart>(json!(59)).is_err());
    }

    #[test]
    fn deserialize_default() {
        let example = TimeRangeStart::default();
        assert_eq!(
            example,
            serde_json::from_value::<TimeRangeStart>(json!(300)).unwrap()
        );
    }

    #[test]
    fn deserialize() {
        let example = TimeRangeStart::try_from(60).unwrap();
        assert_eq!(
            example,
            serde_json::from_value::<TimeRangeStart>(json!(60)).unwrap()
        );
    }
}
