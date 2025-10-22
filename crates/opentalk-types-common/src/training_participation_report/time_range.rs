// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use crate::{
    training_participation_report::{TimeRangeStart, time_range_window::TimeRangeWindow},
    utils::ExampleData,
};

/// A time range within which checkpoints can be randomly created
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(
    feature = "utoipa",
    derive(utoipa::ToSchema),
    schema(example = json!(TimeRange::example_data())),
)]
pub struct TimeRange {
    /// The earliest number of seconds after which the checkpoint can be created.
    pub after: TimeRangeStart,

    /// The number of seconds within which the checkpoint can be created after the `after` value.
    pub within: TimeRangeWindow,
}

impl ExampleData for TimeRange {
    fn example_data() -> Self {
        Self {
            after: TimeRangeStart::from_i64_clamped(1200),
            within: TimeRangeWindow::from_i64_clamped(600),
        }
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use crate::{
        training_participation_report::{
            TimeRange, TimeRangeStart, time_range_window::TimeRangeWindow,
        },
        utils::ExampleData as _,
    };

    #[test]
    fn deserialize_time_range_zero() {
        let json = json!({
            "after": 60,
            "within": 0,
        });

        assert_eq!(
            serde_json::from_value::<TimeRange>(json).unwrap(),
            TimeRange {
                after: TimeRangeStart::from_i64_clamped(60),
                within: TimeRangeWindow::from_i64_clamped(0),
            }
        );
    }

    #[test]
    fn serialize_time_range_zero() {
        assert_eq!(
            json!(TimeRange {
                after: TimeRangeStart::from_i64_clamped(60),
                within: TimeRangeWindow::from_i64_clamped(0)
            }),
            json!({
                "after": 60,
                "within": 0,
            })
        );
    }

    #[test]
    fn deserialize_time_range_example() {
        let json = json!({
            "after": 1200,
            "within": 600,
        });

        assert_eq!(
            serde_json::from_value::<TimeRange>(json).unwrap(),
            TimeRange::example_data()
        );
    }

    #[test]
    fn serialize_time_range_example() {
        assert_eq!(
            json!(TimeRange::example_data()),
            json!({
                "after": 1200,
                "within": 600,
            })
        );
    }

    #[test]
    fn deserialize_invalid_negative_after() {
        let json = json!({
            "after": -1200,
            "within": 600,
        });

        assert!(serde_json::from_value::<TimeRange>(json).is_err());
    }

    #[test]
    fn deserialize_invalid_negative_within() {
        let json = json!({
            "after": 1200,
            "within": -600,
        });

        assert!(serde_json::from_value::<TimeRange>(json).is_err());
    }

    #[test]
    fn deserialize_invalid_missing_after() {
        let json = json!({
            "within": 600,
        });

        assert!(serde_json::from_value::<TimeRange>(json).is_err());
    }

    #[test]
    fn deserialize_invalid_missing_within() {
        let json = json!({
            "after": 1200,
        });

        assert!(serde_json::from_value::<TimeRange>(json).is_err());
    }
}
