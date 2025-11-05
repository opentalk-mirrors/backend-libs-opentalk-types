// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use opentalk_types_common::training_participation_report::TimeRange;

/// Incoming websocket messages
#[derive(Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case", tag = "action")
)]
pub enum TrainingParticipationReportCommand {
    /// Enable presence logging
    EnablePresenceLogging {
        /// The time range definition for the initial checkpoint delay.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        initial_checkpoint_delay: Option<TimeRange>,

        /// The time range definition for the subsequent checkpoints.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        checkpoint_interval: Option<TimeRange>,
    },

    /// Disable presence logging
    DisablePresenceLogging,

    /// Confirm presence
    ConfirmPresence,
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use std::time::Duration;

    use opentalk_types_common::training_participation_report::TimeRange;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::TrainingParticipationReportCommand;

    #[test]
    fn enable_presence_logging() {
        let json = json!({
            "action": "enable_presence_logging",
        });

        assert_eq!(
            serde_json::from_value::<TrainingParticipationReportCommand>(json).unwrap(),
            TrainingParticipationReportCommand::EnablePresenceLogging {
                initial_checkpoint_delay: None,
                checkpoint_interval: None
            }
        );
    }

    #[test]
    fn enable_presence_logging_with_params() {
        let json = json!({
            "action": "enable_presence_logging",
            "initial_checkpoint_delay": {
                "after": 600,
                "within": 1200,
            },
            "checkpoint_interval": {
                "after": 6300,
                "within": 1800,
            }
        });

        assert_eq!(
            serde_json::from_value::<TrainingParticipationReportCommand>(json).unwrap(),
            TrainingParticipationReportCommand::EnablePresenceLogging {
                initial_checkpoint_delay: Some(TimeRange::new_with_clamped_durations(
                    Duration::from_mins(10),
                    Duration::from_mins(20)
                )),
                checkpoint_interval: Some(TimeRange::new_with_clamped_durations(
                    Duration::from_mins(105),
                    Duration::from_mins(30)
                ))
            }
        );
    }

    #[test]
    fn disable_presence_logging() {
        let json = json!({
            "action": "disable_presence_logging",
        });

        assert_eq!(
            serde_json::from_value::<TrainingParticipationReportCommand>(json).unwrap(),
            TrainingParticipationReportCommand::DisablePresenceLogging
        );
    }

    #[test]
    fn confirm_presence() {
        let json = json!({
            "action": "confirm_presence",
        });

        assert_eq!(
            serde_json::from_value::<TrainingParticipationReportCommand>(json).unwrap(),
            TrainingParticipationReportCommand::ConfirmPresence
        );
    }
}
