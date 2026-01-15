// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use opentalk_types_common::{
    events::{EventDescription, EventTitle},
    rooms::RoomPassword,
    streaming::StreamingTarget,
    time::{DateTimeTz, RecurrencePattern},
    training_participation_report::TrainingParticipationReportParameterSet,
    utils::ExampleData,
};

/// Body for the `PATCH /events/{event_id}` endpoint
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema), schema(
    example = json!(
        PatchEventBody::example_data()
    )
))]
pub struct PatchEventBody {
    /// Patch the title of th event
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    // Field is non-required already, utoipa adds a `nullable: true` entry
    // by default which creates a false positive in the spectral linter when
    // combined with example data.
    #[cfg_attr(feature = "utoipa", schema(nullable = false))]
    pub title: Option<EventTitle>,

    /// Patch the description of the event
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    // Field is non-required already, utoipa adds a `nullable: true` entry
    // by default which creates a false positive in the spectral linter when
    // combined with example data.
    #[cfg_attr(feature = "utoipa", schema(nullable = false))]
    pub description: Option<EventDescription>,

    /// Patch the password of the event's room
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "::serde_with::rust::double_option",
        )
    )]
    // Field is non-required already, utoipa adds a `nullable: true` entry
    // by default which creates a false positive in the spectral linter when
    // combined with example data.
    #[cfg_attr(feature = "utoipa", schema(nullable = false))]
    pub password: Option<Option<RoomPassword>>,

    /// Patch the presence of a waiting room
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    // Field is non-required already, utoipa adds a `nullable: true` entry
    // by default which creates a false positive in the spectral linter when
    // combined with example data.
    #[cfg_attr(feature = "utoipa", schema(nullable = false))]
    pub waiting_room: Option<bool>,

    /// Patch whether the event is encrypted
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    // Field is non-required already, utoipa adds a `nullable: true` entry
    // by default which creates a false positive in the spectral linter when
    // combined with example data.
    #[cfg_attr(feature = "utoipa", schema(nullable = false))]
    pub e2e_encryption: Option<bool>,

    /// Patch the adhoc flag.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    // Field is non-required already, utoipa adds a `nullable: true` entry
    // by default which creates a false positive in the spectral linter when
    // combined with example data.
    #[cfg_attr(feature = "utoipa", schema(nullable = false))]
    pub is_adhoc: Option<bool>,

    /// Patch the time independence of the event
    ///
    /// If it changes the independence from true false this body has to have
    /// `is_all_day`, `starts_at` and `ends_at` set
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    // Field is non-required already, utoipa adds a `nullable: true` entry
    // by default which creates a false positive in the spectral linter when
    // combined with example data.
    #[cfg_attr(feature = "utoipa", schema(nullable = false))]
    pub is_time_independent: Option<bool>,

    /// Patch if the event is an all-day event
    ///
    /// If it changes the value from false to true this request must ensure
    /// that the `starts_at.datetime` and `ends_at.datetime` have a 00:00 time part.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    // Field is non-required already, utoipa adds a `nullable: true` entry
    // by default which creates a false positive in the spectral linter when
    // combined with example data.
    #[cfg_attr(feature = "utoipa", schema(nullable = false))]
    pub is_all_day: Option<bool>,

    /// Patch the start time of the event
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    // Field is non-required already, utoipa adds a `nullable: true` entry
    // by default which creates a false positive in the spectral linter when
    // combined with example data.
    #[cfg_attr(feature = "utoipa", schema(nullable = false))]
    pub starts_at: Option<DateTimeTz>,

    /// Patch the end time of the event
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    // Field is non-required already, utoipa adds a `nullable: true` entry
    // by default which creates a false positive in the spectral linter when
    // combined with example data.
    #[cfg_attr(feature = "utoipa", schema(nullable = false))]
    pub ends_at: Option<DateTimeTz>,

    /// Patch the events recurrence patterns
    ///
    /// If this list is non empty it override the events current one
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "RecurrencePattern::is_empty")
    )]
    // Field is non-required already, utoipa adds a `nullable: true` entry
    // by default which creates a false positive in the spectral linter when
    // combined with example data.
    #[cfg_attr(feature = "utoipa", schema(nullable = false))]
    pub recurrence_pattern: RecurrencePattern,

    /// The streaming targets of the room associated with the event
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    // Field is non-required already, utoipa adds a `nullable: true` entry
    // by default which creates a false positive in the spectral linter when
    // combined with example data.
    #[cfg_attr(feature = "utoipa", schema(nullable = false))]
    pub streaming_targets: Option<Vec<StreamingTarget>>,

    /// Patch wether the meeting details are displayed or not
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    // Field is non-required already, utoipa adds a `nullable: true` entry
    // by default which creates a false positive in the spectral linter when
    // combined with example data.
    #[cfg_attr(feature = "utoipa", schema(nullable = false))]
    pub show_meeting_details: Option<bool>,

    /// Either add a shared folder to the event, if none existed before or delete the shared folder
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    // Field is non-required already, utoipa adds a `nullable: true` entry
    // by default which creates a false positive in the spectral linter when
    // combined with example data.
    #[cfg_attr(feature = "utoipa", schema(nullable = false))]
    pub has_shared_folder: Option<bool>,

    /// The training participation report parameter set for the event.
    ///
    /// When present, the training participation report will be started
    /// automatically in the meeting.
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "::serde_with::rust::double_option",
        )
    )]
    // Field is non-required already, utoipa adds a `nullable: true` entry
    // by default which creates a false positive in the spectral linter when
    // combined with example data.
    #[cfg_attr(feature = "utoipa", schema(nullable = false))]
    pub training_participation_report: Option<Option<TrainingParticipationReportParameterSet>>,
}

impl PatchEventBody {
    /// Check if the body is empty
    pub fn is_empty(&self) -> bool {
        let PatchEventBody {
            title,
            description,
            password,
            waiting_room,
            e2e_encryption,
            is_adhoc,
            is_time_independent,
            is_all_day,
            starts_at,
            ends_at,
            recurrence_pattern,
            show_meeting_details,
            has_shared_folder,
            streaming_targets,
            training_participation_report,
        } = self;

        title.is_none()
            && description.is_none()
            && password.is_none()
            && waiting_room.is_none()
            && e2e_encryption.is_none()
            && is_adhoc.is_none()
            && is_time_independent.is_none()
            && is_all_day.is_none()
            && starts_at.is_none()
            && ends_at.is_none()
            && recurrence_pattern.is_empty()
            && show_meeting_details.is_none()
            && has_shared_folder.is_none()
            && streaming_targets.is_none()
            && training_participation_report.is_none()
    }

    // special case to only patch the events room
    /// Check if the body only modifies the room
    pub fn only_modifies_room(&self) -> bool {
        let PatchEventBody {
            title,
            description,
            password,
            waiting_room,
            e2e_encryption,
            is_time_independent,
            is_all_day,
            starts_at,
            ends_at,
            recurrence_pattern,
            is_adhoc,
            show_meeting_details,
            has_shared_folder,
            streaming_targets,
            training_participation_report,
        } = self;

        title.is_none()
            && description.is_none()
            && is_time_independent.is_none()
            && is_all_day.is_none()
            && starts_at.is_none()
            && ends_at.is_none()
            && recurrence_pattern.is_empty()
            && is_adhoc.is_none()
            && show_meeting_details.is_none()
            && has_shared_folder.is_none()
            && streaming_targets.is_none()
            && (password.is_some() || waiting_room.is_some() || e2e_encryption.is_some())
            && training_participation_report.is_none()
    }
}

impl ExampleData for PatchEventBody {
    fn example_data() -> Self {
        Self {
            title: Some("The new title".parse().expect("valid event title")),
            description: Some(
                "The new description"
                    .parse()
                    .expect("valid event description"),
            ),
            password: None,
            waiting_room: None,
            e2e_encryption: None,
            is_adhoc: None,
            is_time_independent: None,
            is_all_day: None,
            starts_at: None,
            ends_at: None,
            recurrence_pattern: RecurrencePattern::default(),
            streaming_targets: None,
            show_meeting_details: Some(false),
            has_shared_folder: Some(true),
            training_participation_report: None,
        }
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use std::{str::FromStr, time::Duration};

    use chrono::{TimeZone, Utc};
    use opentalk_types_common::{
        events::{EventDescription, EventTitle},
        rooms::RoomPassword,
        time::{DateTimeTz, RecurrencePattern},
        training_participation_report::{TimeRange, TrainingParticipationReportParameterSet},
    };
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use crate::events::PatchEventBody;

    #[test]
    fn deserialize_empty() {
        let json = json!({});

        assert_eq!(
            serde_json::from_value::<PatchEventBody>(json).unwrap(),
            PatchEventBody {
                title: None,
                description: None,
                password: None,
                waiting_room: None,
                e2e_encryption: None,
                is_adhoc: None,
                is_time_independent: None,
                is_all_day: None,
                starts_at: None,
                ends_at: None,
                recurrence_pattern: RecurrencePattern::default(),
                streaming_targets: None,
                show_meeting_details: None,
                has_shared_folder: None,
                training_participation_report: None
            }
        );
    }

    #[test]
    fn serialize_empty() {
        assert_eq!(
            json!(PatchEventBody {
                title: None,
                description: None,
                password: None,
                waiting_room: None,
                e2e_encryption: None,
                is_adhoc: None,
                is_time_independent: None,
                is_all_day: None,
                starts_at: None,
                ends_at: None,
                recurrence_pattern: RecurrencePattern::default(),
                streaming_targets: None,
                show_meeting_details: None,
                has_shared_folder: None,
                training_participation_report: None
            }),
            json!({})
        );
    }

    #[test]
    fn deserialize_training_participation_report_set_value() {
        let json = json!({
            "training_participation_report": {
                "initial_checkpoint_delay": {
                    "after": 100,
                    "within": 200,
                },
                "checkpoint_interval": {
                    "after": 300,
                    "within": 400,
                },
            }
        });

        assert_eq!(
            serde_json::from_value::<PatchEventBody>(json).unwrap(),
            PatchEventBody {
                title: None,
                description: None,
                password: None,
                waiting_room: None,
                e2e_encryption: None,
                is_adhoc: None,
                is_time_independent: None,
                is_all_day: None,
                starts_at: None,
                ends_at: None,
                recurrence_pattern: RecurrencePattern::default(),
                streaming_targets: None,
                show_meeting_details: None,
                has_shared_folder: None,
                training_participation_report: Some(Some(
                    TrainingParticipationReportParameterSet {
                        initial_checkpoint_delay: TimeRange::new_with_clamped_durations(
                            Duration::from_secs(100),
                            Duration::from_secs(200)
                        ),
                        checkpoint_interval: TimeRange::new_with_clamped_durations(
                            Duration::from_secs(300),
                            Duration::from_secs(400)
                        ),
                    }
                ))
            }
        );
    }

    #[test]
    fn serialize_non_empty() {
        let expected = json!({
            "description": "Test",
            "e2e_encryption": true,
            "has_shared_folder": true,
            "is_adhoc": false,
            "is_all_day": false,
            "is_time_independent": false,
            "show_meeting_details": true,
            "waiting_room": true,
            "password": "v3rys3cr3t",
            "starts_at": {
                "datetime": "2024-07-05T17:23:42Z",
                "timezone": "Europe/Berlin",
            },
            "ends_at":  {
                "datetime": "2024-07-05T18:33:52Z",
                "timezone": "Europe/Berlin",
            },
            "streaming_targets": [],
            "title": "Test",
            "training_participation_report": {
                "initial_checkpoint_delay": {
                    "after": 100,
                    "within": 200,
                },
                "checkpoint_interval": {
                    "after": 300,
                    "within": 400,
                },
            }
        });

        let produced = json!(PatchEventBody {
            title: Some(EventTitle::from_str_lossy("Test")),
            description: Some(EventDescription::from_str_lossy("Test")),
            password: Some(Some(RoomPassword::from_str("v3rys3cr3t").unwrap())),
            waiting_room: Some(true),
            e2e_encryption: Some(true),
            is_adhoc: Some(false),
            is_time_independent: Some(false),
            is_all_day: Some(false),
            starts_at: Some(DateTimeTz {
                datetime: Utc.with_ymd_and_hms(2024, 7, 5, 17, 23, 42).unwrap(),
                timezone: chrono_tz::Europe::Berlin.into(),
            }),
            ends_at: Some(DateTimeTz {
                datetime: Utc.with_ymd_and_hms(2024, 7, 5, 18, 33, 52).unwrap(),
                timezone: chrono_tz::Europe::Berlin.into(),
            }),
            recurrence_pattern: RecurrencePattern::default(),
            streaming_targets: Some(Vec::new()),
            show_meeting_details: Some(true),
            has_shared_folder: Some(true),
            training_participation_report: Some(Some(TrainingParticipationReportParameterSet {
                initial_checkpoint_delay: TimeRange::new_with_clamped_durations(
                    Duration::from_secs(100),
                    Duration::from_secs(200)
                ),
                checkpoint_interval: TimeRange::new_with_clamped_durations(
                    Duration::from_secs(300),
                    Duration::from_secs(400)
                ),
            }))
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialize_non_empty() {
        let expected = PatchEventBody {
            title: Some(EventTitle::from_str_lossy("Test")),
            description: Some(EventDescription::from_str_lossy("Test")),
            password: Some(Some(RoomPassword::from_str("v3rys3cr3t").unwrap())),
            waiting_room: Some(true),
            e2e_encryption: Some(true),
            is_adhoc: Some(false),
            is_time_independent: Some(false),
            is_all_day: Some(false),
            starts_at: Some(DateTimeTz {
                datetime: Utc.with_ymd_and_hms(2024, 7, 5, 17, 23, 42).unwrap(),
                timezone: chrono_tz::Europe::Berlin.into(),
            }),
            ends_at: Some(DateTimeTz {
                datetime: Utc.with_ymd_and_hms(2024, 7, 5, 18, 33, 52).unwrap(),
                timezone: chrono_tz::Europe::Berlin.into(),
            }),
            recurrence_pattern: RecurrencePattern::default(),
            streaming_targets: Some(Vec::new()),
            show_meeting_details: Some(true),
            has_shared_folder: Some(true),
            training_participation_report: Some(Some(TrainingParticipationReportParameterSet {
                initial_checkpoint_delay: TimeRange::new_with_clamped_durations(
                    Duration::from_secs(100),
                    Duration::from_secs(200),
                ),
                checkpoint_interval: TimeRange::new_with_clamped_durations(
                    Duration::from_secs(300),
                    Duration::from_secs(400),
                ),
            })),
        };

        let produced = serde_json::from_value::<PatchEventBody>(json!({
            "description": "Test",
            "e2e_encryption": true,
            "has_shared_folder": true,
            "is_adhoc": false,
            "is_all_day": false,
            "is_time_independent": false,
            "show_meeting_details": true,
            "waiting_room": true,
            "password": "v3rys3cr3t",
            "starts_at": {
                "datetime": "2024-07-05T17:23:42Z",
                "timezone": "Europe/Berlin",
            },
            "ends_at":  {
                "datetime": "2024-07-05T18:33:52Z",
                "timezone": "Europe/Berlin",
            },
            "streaming_targets": [],
            "title": "Test",
            "training_participation_report": {
                "initial_checkpoint_delay": {
                    "after": 100,
                    "within": 200,
                },
                "checkpoint_interval": {
                    "after": 300,
                    "within": 400,
                },
            }
        }))
        .unwrap();

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialize_training_participation_report_set_value() {
        assert_eq!(
            json!(PatchEventBody {
                title: None,
                description: None,
                password: None,
                waiting_room: None,
                e2e_encryption: None,
                is_adhoc: None,
                is_time_independent: None,
                is_all_day: None,
                starts_at: None,
                ends_at: None,
                recurrence_pattern: RecurrencePattern::default(),
                streaming_targets: None,
                show_meeting_details: None,
                has_shared_folder: None,
                training_participation_report: Some(Some(
                    TrainingParticipationReportParameterSet {
                        initial_checkpoint_delay: TimeRange::new_with_clamped_durations(
                            Duration::from_secs(100),
                            Duration::from_secs(200)
                        ),
                        checkpoint_interval: TimeRange::new_with_clamped_durations(
                            Duration::from_secs(300),
                            Duration::from_secs(400)
                        ),
                    }
                ))
            }),
            json!({
                "training_participation_report": {
                    "initial_checkpoint_delay": {
                        "after": 100,
                        "within": 200,
                    },
                    "checkpoint_interval": {
                        "after": 300,
                        "within": 400,
                    },
                }
            })
        );
    }

    #[test]
    fn deserialize_training_participation_report_reset() {
        let json = json!({
            "training_participation_report": null
        });

        assert_eq!(
            serde_json::from_value::<PatchEventBody>(json).unwrap(),
            PatchEventBody {
                title: None,
                description: None,
                password: None,
                waiting_room: None,
                e2e_encryption: None,
                is_adhoc: None,
                is_time_independent: None,
                is_all_day: None,
                starts_at: None,
                ends_at: None,
                recurrence_pattern: RecurrencePattern::default(),
                streaming_targets: None,
                show_meeting_details: None,
                has_shared_folder: None,
                training_participation_report: Some(None)
            }
        );
    }

    #[test]
    fn serialize_training_participation_report_reset() {
        assert_eq!(
            json!(PatchEventBody {
                title: None,
                description: None,
                password: None,
                waiting_room: None,
                e2e_encryption: None,
                is_adhoc: None,
                is_time_independent: None,
                is_all_day: None,
                starts_at: None,
                ends_at: None,
                recurrence_pattern: RecurrencePattern::default(),
                streaming_targets: None,
                show_meeting_details: None,
                has_shared_folder: None,
                training_participation_report: Some(None)
            }),
            json!({
                "training_participation_report": null,
            })
        );
    }
}
