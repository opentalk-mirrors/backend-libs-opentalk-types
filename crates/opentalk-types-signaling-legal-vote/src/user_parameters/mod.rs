// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Signaling user parameters for the `legal-vote` namespace.

mod allowed_participants;
mod duration;
mod name;
mod subtitle;
mod topic;

pub use allowed_participants::{AllowedParticipants, TryFromAllowedParticipantsError};
pub use duration::{Duration, TryFromDurationError};
pub use name::{Name, ParseNameError};
use opentalk_types_common::time::TimeZone;
pub use opentalk_types_signaling::ParticipantId;
pub use subtitle::{ParseSubtitleError, Subtitle};
pub use topic::{ParseTopicError, Topic};

use crate::vote::VoteKind;

/// The users parameters to start a new vote
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "redis",
    derive(redis_args::ToRedisArgs, redis_args::FromRedisValue),
    to_redis_args(serde),
    from_redis_value(serde)
)]
pub struct UserParameters {
    /// The kind of vote
    pub kind: VoteKind,

    /// The name of the vote
    pub name: Name,

    /// A Subtitle for the vote
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub subtitle: Option<Subtitle>,

    /// The topic that will be voted on
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub topic: Option<Topic>,

    /// List of participants that are allowed to cast a vote
    pub allowed_participants: AllowedParticipants,

    /// Indicates that the `Abstain` vote option is enabled
    pub enable_abstain: bool,

    /// The vote will automatically stop when every participant voted
    pub auto_close: bool,

    /// The vote will stop when the duration (in seconds) has passed
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub duration: Option<Duration>,

    /// A PDF document will be created when the vote is over
    pub create_pdf: bool,

    /// An optional timezone, defaults to UTC.
    /// Format as standardized by IANA, e.g.\"CET\" or \"Europe/Vienna\".
    /// See: <https://www.iana.org/time-zones>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub timezone: Option<TimeZone>,
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct TestStruct {
        duration: Duration,
    }

    #[test]
    fn serialization() {
        let produced = serde_json::to_value(UserParameters {
            kind: VoteKind::RollCall,
            name: Name::try_from("Test Name").unwrap(),
            subtitle: Some(Subtitle::try_from("Test Subtitle").unwrap()),
            topic: Some(Topic::try_from("Test Topic").unwrap()),
            allowed_participants: AllowedParticipants::try_from(vec![ParticipantId::from_u128(1)])
                .unwrap(),
            enable_abstain: false,
            auto_close: false,
            duration: Some(Duration::try_from(10).unwrap()),
            create_pdf: false,
            timezone: Some(chrono_tz::Tz::Europe__Berlin.into()),
        })
        .unwrap();

        let expected = json!({
            "kind": "roll_call",
            "name": "Test Name",
            "subtitle": "Test Subtitle",
            "topic": "Test Topic",
            "allowed_participants": ["00000000-0000-0000-0000-000000000001"],
            "enable_abstain": false,
            "auto_close": false,
            "duration": 10,
            "create_pdf": false,
            "timezone": "Europe/Berlin",
        });

        assert_eq!(produced, expected);

        let produced = serde_json::to_value(UserParameters {
            kind: VoteKind::RollCall,
            name: Name::try_from("Test Name").unwrap(),
            subtitle: None,
            topic: None,
            allowed_participants: AllowedParticipants::try_from(vec![ParticipantId::from_u128(1)])
                .unwrap(),
            enable_abstain: false,
            auto_close: false,
            duration: None,
            create_pdf: false,
            timezone: None,
        })
        .unwrap();

        let expected = json!({
            "kind": "roll_call",
            "name": "Test Name",
            "allowed_participants": ["00000000-0000-0000-0000-000000000001"],
            "enable_abstain": false,
            "auto_close": false,
            "create_pdf": false,
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization() {
        let produced: UserParameters = serde_json::from_value(json!({
            "kind": "roll_call",
            "name": "Test Name",
            "subtitle": "Test Subtitle",
            "topic": "Test Topic",
            "allowed_participants": ["00000000-0000-0000-0000-000000000001"],
            "enable_abstain": false,
            "auto_close": false,
            "duration": 10,
            "create_pdf": false,
            "timezone": "Europe/Berlin",

        }))
        .unwrap();

        let expected = UserParameters {
            kind: VoteKind::RollCall,
            name: Name::try_from("Test Name").unwrap(),
            subtitle: Some(Subtitle::try_from("Test Subtitle").unwrap()),
            topic: Some(Topic::try_from("Test Topic").unwrap()),
            allowed_participants: AllowedParticipants::try_from(vec![ParticipantId::from_u128(1)])
                .unwrap(),
            enable_abstain: false,
            auto_close: false,
            duration: Some(Duration::try_from(10).unwrap()),
            create_pdf: false,
            timezone: Some(chrono_tz::Tz::Europe__Berlin.into()),
        };

        assert_eq!(produced, expected);

        let produced: UserParameters = serde_json::from_value(json!({
            "kind": "roll_call",
            "name": "Test Name",
            "allowed_participants": ["00000000-0000-0000-0000-000000000001"],
            "enable_abstain": false,
            "auto_close": false,
            "create_pdf": false,

        }))
        .unwrap();

        let expected = UserParameters {
            kind: VoteKind::RollCall,
            name: Name::try_from("Test Name").unwrap(),
            subtitle: None,
            topic: None,
            allowed_participants: AllowedParticipants::try_from(vec![ParticipantId::from_u128(1)])
                .unwrap(),
            enable_abstain: false,
            auto_close: false,
            duration: None,
            create_pdf: false,
            timezone: None,
        };

        assert_eq!(produced, expected);
    }
}
