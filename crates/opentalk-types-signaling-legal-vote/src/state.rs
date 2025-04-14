// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Legal vote state for `legal-vote` namespace.

use crate::vote::VoteSummary;

/// Data sent to the frontend on `join_success`, when legal-vote is active.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LegalVoteState {
    /// Current public summary of all votes.
    pub votes: Vec<VoteSummary>,
}

#[cfg(feature = "serde")]
impl opentalk_types_signaling::SignalingModuleFrontendData for LegalVoteState {
    const NAMESPACE: Option<crate::ModuleId> = Some(crate::MODULE_ID);
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use chrono::{TimeZone, Utc};
    use opentalk_types_signaling::ParticipantId;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;
    use crate::{
        parameters::Parameters,
        user_parameters::{AllowedParticipants, Name, UserParameters},
        vote::{LegalVoteId, VoteKind, VoteState},
    };

    #[test]
    fn serialization() {
        let produced = serde_json::to_value(LegalVoteState {
            votes: vec![VoteSummary {
                parameters: Parameters {
                    initiator_id: ParticipantId::from_u128(2),
                    legal_vote_id: LegalVoteId::from_u128(3),
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
                },
                state: VoteState::Started,
                end_time: None,
            }],
        })
        .unwrap();

        let expected = json!({
            "votes": [
                {
                    "state": "started",
                    "initiator_id": "00000000-0000-0000-0000-000000000002",
                    "legal_vote_id": "00000000-0000-0000-0000-000000000003",
                    "start_time": "2025-01-01T00:00:00Z",
                    "max_votes": 5,
                    "kind": "roll_call",
                    "name": "Test Name",
                    "allowed_participants": ["00000000-0000-0000-0000-000000000001"],
                    "enable_abstain": false,
                    "auto_close": false,
                    "create_pdf": false,
                }
            ],
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization() {
        let produced: LegalVoteState = serde_json::from_value(json!({
            "votes": [
                {
                    "state": "started",
                    "initiator_id": "00000000-0000-0000-0000-000000000002",
                    "legal_vote_id": "00000000-0000-0000-0000-000000000003",
                    "start_time": "2025-01-01T00:00:00Z",
                    "max_votes": 5,
                    "kind": "roll_call",
                    "name": "Test Name",
                    "allowed_participants": ["00000000-0000-0000-0000-000000000001"],
                    "enable_abstain": false,
                    "auto_close": false,
                    "create_pdf": false,
                }
            ],
        }))
        .unwrap();

        let expected = LegalVoteState {
            votes: vec![VoteSummary {
                parameters: Parameters {
                    initiator_id: ParticipantId::from_u128(2),
                    legal_vote_id: LegalVoteId::from_u128(3),
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
                },
                state: VoteState::Started,
                end_time: None,
            }],
        };

        assert_eq!(produced, expected);
    }
}
