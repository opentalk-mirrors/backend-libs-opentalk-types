// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Signaling parameters for the `legal-vote` namespace.

use std::collections::BTreeSet;

use chrono::{DateTime, Utc};
use opentalk_types_common::users::UserId;
use opentalk_types_signaling::ParticipantId;

use crate::{token::Token, user_parameters::UserParameters, vote::LegalVoteId};

/// Wraps the [`UserParameters`] with additional server side information
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize,))]
#[cfg_attr(
    feature = "redis",
    derive(redis_args::ToRedisArgs, redis_args::FromRedisValue),
    to_redis_args(serde),
    from_redis_value(serde)
)]
pub struct Parameters {
    /// The participant id of the vote initiator
    pub initiator_id: ParticipantId,

    /// The unique id of this vote
    pub legal_vote_id: LegalVoteId,

    /// The time the vote got started
    pub start_time: DateTime<Utc>,

    /// The maximum amount of votes possible
    pub max_votes: u32,

    /// List of resolved user ID's
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub allowed_users: Option<Vec<UserId>>,

    /// Parameters set by the initiator
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub inner: UserParameters,

    /// Token for users who are allowed to participate
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub token: Option<Token>,
}

impl Parameters {
    /// Get a Set of all user ids in `allowed_users`.
    pub fn get_referenced_user_ids(&self) -> BTreeSet<UserId> {
        self.allowed_users.iter().flatten().cloned().collect()
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use std::str::FromStr;

    use chrono::TimeZone;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;
    use crate::{
        user_parameters::{AllowedParticipants, Name},
        vote::VoteKind,
    };

    #[test]
    fn serialization() {
        let produced = serde_json::to_value(Parameters {
            initiator_id: ParticipantId::from_u128(1),
            legal_vote_id: LegalVoteId::nil(),
            start_time: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
            max_votes: 5,
            allowed_users: Some(vec![UserId::from_u128(1)]),
            inner: UserParameters {
                kind: VoteKind::RollCall,
                name: Name::try_from("Test Name").unwrap(),
                subtitle: None,
                topic: None,
                allowed_participants: AllowedParticipants::try_from(vec![
                    ParticipantId::from_u128(1),
                ])
                .unwrap(),
                enable_abstain: false,
                auto_close: false,
                duration: None,
                create_pdf: false,
                timezone: None,
            },
            token: Some(Token::from_str("1111Cn8eVZg").unwrap()),
        })
        .unwrap();

        let expected = json!({
            "initiator_id": "00000000-0000-0000-0000-000000000001",
            "legal_vote_id": "00000000-0000-0000-0000-000000000000",
            "start_time":"2025-01-01T00:00:00Z",
            "max_votes": 5,
            "allowed_users": [
               "00000000-0000-0000-0000-000000000001",
            ],
            "kind": "roll_call",
            "name": "Test Name",
            "allowed_participants": [
               "00000000-0000-0000-0000-000000000001",
            ],
            "enable_abstain": false,
            "auto_close": false,
            "create_pdf": false,
            "token": "1111Cn8eVZg",
        });

        assert_eq!(produced, expected);

        let produced = serde_json::to_value(Parameters {
            initiator_id: ParticipantId::from_u128(1),
            legal_vote_id: LegalVoteId::nil(),
            start_time: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
            max_votes: 5,
            allowed_users: None,
            inner: UserParameters {
                kind: VoteKind::RollCall,
                name: Name::try_from("Test Name").unwrap(),
                subtitle: None,
                topic: None,
                allowed_participants: AllowedParticipants::try_from(vec![
                    ParticipantId::from_u128(1),
                ])
                .unwrap(),
                enable_abstain: false,
                auto_close: false,
                duration: None,
                create_pdf: false,
                timezone: None,
            },
            token: None,
        })
        .unwrap();

        let expected = json!({
            "initiator_id": "00000000-0000-0000-0000-000000000001",
            "legal_vote_id": "00000000-0000-0000-0000-000000000000",
            "start_time":"2025-01-01T00:00:00Z",
            "max_votes": 5,
            "kind": "roll_call",
            "name": "Test Name",
            "allowed_participants": [
               "00000000-0000-0000-0000-000000000001",
            ],
            "enable_abstain": false,
            "auto_close": false,
            "create_pdf": false,
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization() {
        let produced: Parameters = serde_json::from_value(json!({
            "initiator_id": "00000000-0000-0000-0000-000000000001",
            "legal_vote_id": "00000000-0000-0000-0000-000000000000",
            "start_time":"2025-01-01T00:00:00Z",
            "max_votes": 5,
            "allowed_users": [
               "00000000-0000-0000-0000-000000000001",
            ],
            "kind": "roll_call",
            "name": "Test Name",
            "allowed_participants": [
               "00000000-0000-0000-0000-000000000001",
            ],
            "enable_abstain": false,
            "auto_close": false,
            "create_pdf": false,
            "token": "1111Cn8eVZg",
        }))
        .unwrap();

        let expected = Parameters {
            initiator_id: ParticipantId::from_u128(1),
            legal_vote_id: LegalVoteId::nil(),
            start_time: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
            max_votes: 5,
            allowed_users: Some(vec![UserId::from_u128(1)]),
            inner: UserParameters {
                kind: VoteKind::RollCall,
                name: Name::try_from("Test Name").unwrap(),
                subtitle: None,
                topic: None,
                allowed_participants: AllowedParticipants::try_from(vec![
                    ParticipantId::from_u128(1),
                ])
                .unwrap(),
                enable_abstain: false,
                auto_close: false,
                duration: None,
                create_pdf: false,
                timezone: None,
            },
            token: Some(Token::from_str("1111Cn8eVZg").unwrap()),
        };

        assert_eq!(produced, expected);

        let produced: Parameters = serde_json::from_value(json!({
            "initiator_id": "00000000-0000-0000-0000-000000000001",
            "legal_vote_id": "00000000-0000-0000-0000-000000000000",
            "start_time":"2025-01-01T00:00:00Z",
            "max_votes": 5,
            "kind": "roll_call",
            "name": "Test Name",
            "allowed_participants": [
               "00000000-0000-0000-0000-000000000001",
            ],
            "enable_abstain": false,
            "auto_close": false,
            "create_pdf": false,
        }))
        .unwrap();

        let expected = Parameters {
            initiator_id: ParticipantId::from_u128(1),
            legal_vote_id: LegalVoteId::nil(),
            start_time: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
            max_votes: 5,
            allowed_users: None,
            inner: UserParameters {
                kind: VoteKind::RollCall,
                name: Name::try_from("Test Name").unwrap(),
                subtitle: None,
                topic: None,
                allowed_participants: AllowedParticipants::try_from(vec![
                    ParticipantId::from_u128(1),
                ])
                .unwrap(),
                enable_abstain: false,
                auto_close: false,
                duration: None,
                create_pdf: false,
                timezone: None,
            },
            token: None,
        };

        assert_eq!(produced, expected);
    }
}
