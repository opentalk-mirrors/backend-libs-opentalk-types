// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use crate::{cancel::Cancel, event::Results, invalid::Invalid, vote::StopKind};

/// Represents the various states a vote can be in during its lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(tag = "state", rename_all = "snake_case")
)]
pub enum VoteState {
    /// The vote has started but not yet finished.
    Started,

    /// The vote has finished, with results and a specified stop kind.
    Finished {
        /// The reason or kind of stop event that caused the vote to finish.
        #[cfg_attr(feature = "serde", serde(flatten))]
        stop_kind: StopKind,

        /// The results of the vote once it has concluded.
        #[cfg_attr(feature = "serde", serde(flatten))]
        results: Results,
    },

    /// The vote was canceled, with details about the cancellation.
    Canceled(Cancel),

    /// The vote is considered invalid, with additional invalidation details.
    Invalid(Invalid),
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use std::collections::HashMap;

    use opentalk_types_common::users::UserId;
    use opentalk_types_signaling::ParticipantId;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;
    use crate::{cancel::CancelReason, event::VotingRecord, tally::Tally, vote::VoteOption};

    #[test]
    fn serialization_started_vote_state() {
        let produced = serde_json::to_value(VoteState::Started).unwrap();

        let expected = json!({"state":"started"});

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_started_vote_state() {
        let produced: VoteState = serde_json::from_value(json!({"state":"started"})).unwrap();

        let expected = VoteState::Started;

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_finished_vote_state() {
        let produced = serde_json::to_value(VoteState::Finished {
            stop_kind: StopKind::Auto,
            results: Results {
                tally: Tally {
                    yes: 1,
                    no: 0,
                    abstain: None,
                },
                voting_record: VotingRecord::UserVotes(
                    vec![(ParticipantId::from_u128(1), VoteOption::Yes)]
                        .into_iter()
                        .collect::<HashMap<ParticipantId, VoteOption>>(),
                ),
            },
        })
        .unwrap();

        let expected = json!({
            "state": "finished",
            "stop_kind": "auto",
            "yes": 1,
            "no": 0,
            "voting_record": {
                "00000000-0000-0000-0000-000000000001": "yes"
            },
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_finished_vote_state() {
        let produced: VoteState = serde_json::from_value(json!({
            "state": "finished",
            "stop_kind": "auto",
            "yes": 1,
            "no": 0,
            "voting_record": {
                "00000000-0000-0000-0000-000000000001": "yes"
            },
        }))
        .unwrap();

        let expected = VoteState::Finished {
            stop_kind: StopKind::Auto,
            results: Results {
                tally: Tally {
                    yes: 1,
                    no: 0,
                    abstain: None,
                },
                voting_record: VotingRecord::UserVotes(
                    vec![(ParticipantId::from_u128(1), VoteOption::Yes)]
                        .into_iter()
                        .collect::<HashMap<ParticipantId, VoteOption>>(),
                ),
            },
        };

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_canceled_vote_state() {
        let produced = serde_json::to_value(VoteState::Canceled(Cancel {
            issuer: UserId::from_u128(1),
            reason: CancelReason::RoomDestroyed,
        }))
        .unwrap();

        let expected = json!({
            "state": "canceled",
            "issuer": "00000000-0000-0000-0000-000000000001",
            "reason": "room_destroyed",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_abstain_vote_option() {
        let produced: VoteState = serde_json::from_value(json!({
            "state": "canceled",
            "issuer": "00000000-0000-0000-0000-000000000001",
            "reason": "room_destroyed",
        }))
        .unwrap();

        let expected = VoteState::Canceled(Cancel {
            issuer: UserId::from_u128(1),
            reason: CancelReason::RoomDestroyed,
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_invalid_vote_state() {
        let produced = serde_json::to_value(VoteState::Invalid(Invalid::AbstainDisabled)).unwrap();

        let expected = json!({
            "state": "invalid",
            "reason": "abstain_disabled",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_invalid_vote_option() {
        let produced: VoteState = serde_json::from_value(json!({
            "state": "invalid",
            "reason": "abstain_disabled",
        }))
        .unwrap();

        let expected = VoteState::Invalid(Invalid::AbstainDisabled);

        assert_eq!(produced, expected);
    }
}
