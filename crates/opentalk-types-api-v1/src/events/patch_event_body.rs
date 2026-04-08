// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use opentalk_types_common::{
    events::{EventDescription, EventTitle},
    rooms::{GuestAccess, RoomPassword},
    streaming::StreamingTarget,
    training_participation_report::TrainingParticipationReportParameterSet,
    utils::ExampleData,
};

use crate::events::PatchEventDateKind;

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

    /// Guest access mode
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    #[cfg_attr(feature = "utoipa", schema(nullable = false))]
    pub guest_access: Option<GuestAccess>,

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

    /// Patch the date of the event.
    ///
    /// - If `is_time_independent` is `false` the body **must** have
    ///   `is_all_day`, `starts_at` and `ends_at`
    /// - If `is_time_independent` is `true` the body **can** have
    ///   `is_all_day, `starts_at` and `ends_at`
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            flatten,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_some_patch_event_date_kind"
        )
    )]
    // Field is non-required already, utoipa adds a `nullable: true` entry
    // by default which creates a false positive in the spectral linter when
    // combined with example data.
    #[cfg_attr(feature = "utoipa", schema(nullable = false))]
    pub date: Option<PatchEventDateKind>,
}

impl PatchEventBody {
    /// Check if the body is empty
    pub const fn is_empty(&self) -> bool {
        let PatchEventBody {
            title,
            description,
            password,
            waiting_room,
            guest_access,
            e2e_encryption,
            is_adhoc,
            show_meeting_details,
            has_shared_folder,
            streaming_targets,
            training_participation_report,
            date,
        } = self;

        title.is_none()
            && description.is_none()
            && password.is_none()
            && waiting_room.is_none()
            && guest_access.is_none()
            && e2e_encryption.is_none()
            && is_adhoc.is_none()
            && show_meeting_details.is_none()
            && has_shared_folder.is_none()
            && streaming_targets.is_none()
            && training_participation_report.is_none()
            && match date {
                Some(date) => date.is_empty(),
                None => true,
            }
    }

    // special case to only patch the events room
    /// Check if the body only modifies the room
    pub fn only_modifies_room(&self) -> bool {
        let PatchEventBody {
            title,
            description,
            password,
            waiting_room,
            guest_access,
            e2e_encryption,
            is_adhoc,
            show_meeting_details,
            has_shared_folder,
            streaming_targets,
            training_participation_report,
            date,
        } = self;

        title.is_none()
            && description.is_none()
            && is_adhoc.is_none()
            && show_meeting_details.is_none()
            && has_shared_folder.is_none()
            && streaming_targets.is_none()
            && (password.is_some()
                || waiting_room.is_some()
                || guest_access.is_some()
                || e2e_encryption.is_some())
            && training_participation_report.is_none()
            && date.as_ref().is_none_or(|date| date.is_empty())
    }
}

#[cfg(feature = "serde")]
fn deserialize_some_patch_event_date_kind<'de, D>(
    deserializer: D,
) -> Result<Option<PatchEventDateKind>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    let value = PatchEventDateKind::deserialize(deserializer)?;

    // If every field is None, treat the whole thing as absent.
    if value.is_empty() {
        return Ok(None);
    }

    Ok(Some(value))
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
            guest_access: None,
            e2e_encryption: None,
            is_adhoc: None,
            streaming_targets: None,
            show_meeting_details: Some(false),
            has_shared_folder: Some(true),
            training_participation_report: None,
            date: Some(PatchEventDateKind::SET_TIME_INDEPENDENT),
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

    use super::*;
    use crate::events::{EventDate, PatchEventBody, PatchEventDate, TimeDependentMarker};

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
                guest_access: None,
                e2e_encryption: None,
                is_adhoc: None,
                streaming_targets: None,
                show_meeting_details: None,
                has_shared_folder: None,
                training_participation_report: None,
                date: None,
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
                guest_access: None,
                e2e_encryption: None,
                is_adhoc: None,
                streaming_targets: None,
                show_meeting_details: None,
                has_shared_folder: None,
                training_participation_report: None,
                date: None,
            }),
            json!({})
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
            "guest_access" : "direct_access",
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
            guest_access: Some(GuestAccess::DirectAccess),
            e2e_encryption: Some(true),
            is_adhoc: Some(false),
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
            })),
            date: Some(PatchEventDateKind::SetTimeDependent {
                is_time_independent: TimeDependentMarker,
                date: EventDate {
                    is_all_day: false,
                    starts_at: DateTimeTz {
                        datetime: Utc.with_ymd_and_hms(2024, 7, 5, 17, 23, 42).unwrap(),
                        timezone: chrono_tz::Europe::Berlin.into(),
                    },
                    ends_at: DateTimeTz {
                        datetime: Utc.with_ymd_and_hms(2024, 7, 5, 18, 33, 52).unwrap(),
                        timezone: chrono_tz::Europe::Berlin.into(),
                    },
                    recurrence_pattern: RecurrencePattern::default(),
                },
            }),
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
            guest_access: Some(GuestAccess::DirectAccess),
            e2e_encryption: Some(true),
            is_adhoc: Some(false),
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
            date: Some(PatchEventDateKind::SetTimeDependent {
                is_time_independent: TimeDependentMarker,
                date: EventDate {
                    is_all_day: false,
                    starts_at: DateTimeTz {
                        datetime: Utc.with_ymd_and_hms(2024, 7, 5, 17, 23, 42).unwrap(),
                        timezone: chrono_tz::Europe::Berlin.into(),
                    },
                    ends_at: DateTimeTz {
                        datetime: Utc.with_ymd_and_hms(2024, 7, 5, 18, 33, 52).unwrap(),
                        timezone: chrono_tz::Europe::Berlin.into(),
                    },
                    recurrence_pattern: RecurrencePattern::default(),
                },
            }),
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
            "guest_access": "direct_access",
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
                guest_access: None,
                e2e_encryption: None,
                is_adhoc: None,
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
                )),
                date: None,
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
                guest_access: None,
                e2e_encryption: None,
                is_adhoc: None,
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
                )),
                date: None,
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
                guest_access: None,
                e2e_encryption: None,
                is_adhoc: None,
                streaming_targets: None,
                show_meeting_details: None,
                has_shared_folder: None,
                training_participation_report: Some(None),
                date: None,
            }),
            json!({
                "training_participation_report": null,
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
                guest_access: None,
                e2e_encryption: None,
                is_adhoc: None,
                streaming_targets: None,
                show_meeting_details: None,
                has_shared_folder: None,
                training_participation_report: Some(None),
                date: None,
            }
        );
    }

    #[test]
    fn serialize_with_all_day_date() {
        let expected = json!({
            "is_all_day": true,
            "ends_at": {
                "datetime": "2002-04-01T11:41:35Z",
                "timezone": "Europe/Berlin",
            },
        });

        let produced = json!(PatchEventBody {
            title: None,
            description: None,
            password: None,
            waiting_room: None,
            guest_access: None,
            e2e_encryption: None,
            is_adhoc: None,
            streaming_targets: None,
            show_meeting_details: None,
            has_shared_folder: None,
            training_participation_report: None,
            date: Some(PatchEventDateKind::PatchTimeDependent {
                is_time_independent: None,
                date: PatchEventDate {
                    is_all_day: Some(true),
                    starts_at: None,
                    ends_at: Some(DateTimeTz {
                        datetime: Utc.with_ymd_and_hms(2002, 4, 1, 11, 41, 35).unwrap(),
                        timezone: chrono_tz::Europe::Berlin.into(),
                    }),
                    recurrence_pattern: RecurrencePattern::default(),
                },
            })
        });

        assert_eq!(expected, produced);
    }

    #[test]
    fn deserialize_with_all_day_date() {
        let expected = PatchEventBody {
            title: None,
            description: None,
            password: None,
            waiting_room: None,
            guest_access: None,
            e2e_encryption: None,
            is_adhoc: None,
            streaming_targets: None,
            show_meeting_details: None,
            has_shared_folder: None,
            training_participation_report: None,
            date: Some(PatchEventDateKind::PatchTimeDependent {
                is_time_independent: None,
                date: PatchEventDate {
                    is_all_day: Some(true),
                    starts_at: None,
                    ends_at: Some(DateTimeTz {
                        datetime: Utc.with_ymd_and_hms(2002, 4, 1, 11, 41, 35).unwrap(),
                        timezone: chrono_tz::Europe::Berlin.into(),
                    }),
                    recurrence_pattern: RecurrencePattern::default(),
                },
            }),
        };

        let produced = serde_json::from_value(json!({
            "is_all_day": true,
            "ends_at": {
                "datetime": "2002-04-01T11:41:35Z",
                "timezone": "Europe/Berlin",
            },
        }))
        .unwrap();

        assert_eq!(expected, produced);
    }

    #[test]
    #[should_panic]
    fn deserialize_time_independent_event() {
        let _ = serde_json::from_value::<PatchEventBody>(json!({
            "is_time_independent": true,
            "starts_at": {
                "datetime": "2024-07-05T17:23:42Z",
                "timezone": "Europe/Berlin",
            }
        }))
        .unwrap();
    }
}
