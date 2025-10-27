// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>

// SPDX-License-Identifier: EUPL-1.2

use snafu::{Snafu, ensure};

use crate::{
    training_participation_report::{MINUTES_PER_HOUR, SECONDS_PER_MINUTE},
    utils::ExampleData,
};

const DEFAULT_VALUE: i64 = 5 * SECONDS_PER_MINUTE;
const MIN_VALUE: i64 = 0;
const MAX_VALUE: i64 = 10 * MINUTES_PER_HOUR * SECONDS_PER_MINUTE;

/// Error when parsing a [`TimeRangeWindow`].
#[derive(Debug, Snafu)]
pub enum TryFromTimeRangeWindowError {
    /// The window duration within which a checkpoint can be created after the `after` value is outside of the allowed range.
    #[snafu(display(
        "The window duration within which a checkpoint can be created after the `after` value is outside of the allowed range ({min}..={max})"
    ))]
    OutOfRange {
        /// The minimum window duration within which a checkpoint can be created after the `after` value, in seconds.
        min: i64,
        /// The maximum window duration within which a checkpoint can be created after the `after` value, in seconds.
        max: i64,
    },
}

/// The window duration within which a checkpoint can be created after the `after` value, in seconds.
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema), schema(example = json!(TimeRangeWindow::example_data())))]
pub struct TimeRangeWindow(i64);

impl TimeRangeWindow {
    /// The minimum value of the type.
    pub const MIN: Self = Self(MIN_VALUE);

    /// The maximum value of the type.
    pub const MAX: Self = Self(MAX_VALUE);

    /// The default value of the type.
    pub const DEFAULT: Self = Self(DEFAULT_VALUE);

    /// Create a new [`TimeRangeWindow`] from an i64 value, clamped to the valid range.
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

impl Default for TimeRangeWindow {
    fn default() -> Self {
        Self(DEFAULT_VALUE)
    }
}

impl TryFrom<u64> for TimeRangeWindow {
    type Error = TryFromTimeRangeWindowError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let value = i64::try_from(value).map_err(|_e| TryFromTimeRangeWindowError::OutOfRange {
            min: MIN_VALUE,
            max: MAX_VALUE,
        })?;
        Self::try_from(value)
    }
}

impl TryFrom<usize> for TimeRangeWindow {
    type Error = TryFromTimeRangeWindowError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let value = i64::try_from(value).map_err(|_e| TryFromTimeRangeWindowError::OutOfRange {
            min: MIN_VALUE,
            max: MAX_VALUE,
        })?;
        Self::try_from(value)
    }
}

impl TryFrom<i32> for TimeRangeWindow {
    type Error = TryFromTimeRangeWindowError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::try_from(i64::from(value))
    }
}

impl TryFrom<i64> for TimeRangeWindow {
    type Error = TryFromTimeRangeWindowError;

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

impl From<TimeRangeWindow> for usize {
    fn from(TimeRangeWindow(value): TimeRangeWindow) -> Self {
        usize::try_from(value).unwrap()
    }
}

impl ExampleData for TimeRangeWindow {
    fn example_data() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq, assert_matches};

    use super::{MAX_VALUE, MIN_VALUE, TimeRangeWindow, TryFromTimeRangeWindowError};

    #[test]
    fn saturating_add() {
        assert_eq!(
            TimeRangeWindow::try_from(142).unwrap().saturating_add(3),
            TimeRangeWindow::try_from(145).unwrap()
        );
        assert_eq!(TimeRangeWindow::MAX.saturating_add(3), TimeRangeWindow::MAX);
    }

    #[test]
    fn saturating_sub() {
        assert_eq!(
            TimeRangeWindow::try_from(142).unwrap().saturating_sub(3),
            TimeRangeWindow::try_from(139).unwrap()
        );
        assert_eq!(TimeRangeWindow::MIN.saturating_sub(3), TimeRangeWindow::MIN);
    }

    #[test]
    fn try_from() {
        assert_eq!(
            TimeRangeWindow::try_from(114usize).unwrap(),
            TimeRangeWindow(114)
        );
        assert_eq!(
            TimeRangeWindow::try_from(115i32).unwrap(),
            TimeRangeWindow(115)
        );
        assert_eq!(
            TimeRangeWindow::try_from(116i64).unwrap(),
            TimeRangeWindow(116)
        );
        assert_eq!(
            TimeRangeWindow::try_from(118u64).unwrap(),
            TimeRangeWindow(118)
        );

        assert_matches!(
            TimeRangeWindow::try_from(-42i64),
            Err(TryFromTimeRangeWindowError::OutOfRange {
                min: MIN_VALUE,
                max: MAX_VALUE
            })
        );
        assert_matches!(
            TimeRangeWindow::try_from((i64::MAX as usize) + 1),
            Err(TryFromTimeRangeWindowError::OutOfRange {
                min: MIN_VALUE,
                max: MAX_VALUE
            })
        );
        assert_matches!(
            TimeRangeWindow::try_from((i64::MAX as u64) + 1),
            Err(TryFromTimeRangeWindowError::OutOfRange {
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

    use super::TimeRangeWindow;

    #[test]
    fn serialize_default() {
        let example = TimeRangeWindow::default();
        assert_eq!(json!(example), json!(300));
    }

    #[test]
    fn serialize() {
        let example = TimeRangeWindow::try_from(500).unwrap();
        assert_eq!(json!(example), json!(500));
    }

    #[test]
    fn deserialize_valid_zero() {
        let example = TimeRangeWindow::try_from(0).unwrap();
        assert_eq!(
            example,
            serde_json::from_value::<TimeRangeWindow>(json!(0)).unwrap()
        );
    }

    #[test]
    fn deserialize_too_low() {
        assert!(serde_json::from_value::<TimeRangeWindow>(json!(-1)).is_err());
    }

    #[test]
    fn deserialize_default() {
        let example = TimeRangeWindow::default();
        assert_eq!(
            example,
            serde_json::from_value::<TimeRangeWindow>(json!(300)).unwrap()
        );
    }

    #[test]
    fn deserialize() {
        let example = TimeRangeWindow::try_from(60).unwrap();
        assert_eq!(
            example,
            serde_json::from_value::<TimeRangeWindow>(json!(60)).unwrap()
        );
    }
}
