// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use crate::{
    event::{Canceled, ErrorKind, PdfAsset, ReportedIssue, Stopped, VoteResponse, VoteResults},
    parameters::Parameters,
};

/// A message sent to a participant via a WebSocket connection.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case", tag = "message")
)]
pub enum LegalVoteEvent {
    /// The vote has started.
    Started(Parameters),

    /// Response to a previous vote request.
    Voted(VoteResponse),

    /// The results of a specific vote have changed.
    Updated(VoteResults),

    /// The vote has been stopped.
    Stopped(Stopped),

    /// The vote has been canceled.
    Canceled(Canceled),

    /// A participant has reported an issue.
    ReportedIssue(ReportedIssue),

    /// An error message caused by invalid requests or internal errors.
    Error(ErrorKind),

    /// A PDF asset related to the vote.
    PdfAsset(PdfAsset),
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use std::{collections::HashMap, str::FromStr};

    use chrono::{TimeZone, Utc};
    use opentalk_types_common::assets::AssetId;
    use opentalk_types_signaling::ParticipantId;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;
    use crate::{
        cancel::CancelReason,
        event::{FinalResults, Response, Results, StopKind, VoteSuccess, VotingRecord},
        invalid::Invalid,
        issue::{Issue, OtherIssue},
        tally::Tally,
        token::Token,
        user_parameters::{AllowedParticipants, Name, UserParameters},
        vote::{LegalVoteId, VoteKind, VoteOption},
    };

    #[test]
    fn serialization_started_legal_vote_event() {
        let produced = serde_json::to_value(LegalVoteEvent::Started(Parameters {
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
        }))
        .unwrap();

        let expected = json!({
            "message": "started",
            "initiator_id": "00000000-0000-0000-0000-000000000001",
            "legal_vote_id": "00000000-0000-0000-0000-000000000000",
            "start_time":"2025-01-01T00:00:00Z",
            "max_votes": 5,
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
    fn deserialization_started_legal_vote_event() {
        let produced: LegalVoteEvent = serde_json::from_value(json!({
            "message": "started",
            "initiator_id": "00000000-0000-0000-0000-000000000001",
            "legal_vote_id": "00000000-0000-0000-0000-000000000000",
            "start_time":"2025-01-01T00:00:00Z",
            "max_votes": 5,
            "kind": "roll_call",
            "name": "Test Name",
            "allowed_participants": ["00000000-0000-0000-0000-000000000001"],
            "enable_abstain": false,
            "auto_close": false,
            "create_pdf": false,
        }))
        .unwrap();

        let expected = LegalVoteEvent::Started(Parameters {
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
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_voted_legal_vote_event() {
        let produced = serde_json::to_value(LegalVoteEvent::Voted(VoteResponse {
            legal_vote_id: LegalVoteId::from_u128(1),
            response: Response::Success(VoteSuccess {
                vote_option: VoteOption::Yes,
                issuer: ParticipantId::from_u128(1),
                consumed_token: Token::from_str("1111Cn8eVZg").unwrap(),
            }),
        }))
        .unwrap();

        let expected = json!({
            "message": "voted",
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "response": "success",
            "vote_option": "yes",
            "issuer": "00000000-0000-0000-0000-000000000001",
            "consumed_token": "1111Cn8eVZg",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_voted_legal_vote_event() {
        let produced: LegalVoteEvent = serde_json::from_value(json!({
            "message": "voted",
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "response": "success",
            "vote_option": "yes",
            "issuer": "00000000-0000-0000-0000-000000000001",
            "consumed_token": "1111Cn8eVZg",
        }))
        .unwrap();

        let expected = LegalVoteEvent::Voted(VoteResponse {
            legal_vote_id: LegalVoteId::from_u128(1),
            response: Response::Success(VoteSuccess {
                vote_option: VoteOption::Yes,
                issuer: ParticipantId::from_u128(1),
                consumed_token: Token::from_str("1111Cn8eVZg").unwrap(),
            }),
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_updated_legal_vote_event() {
        let produced = serde_json::to_value(LegalVoteEvent::Updated(VoteResults {
            legal_vote_id: LegalVoteId::from_u128(1),
            results: Results {
                tally: Tally {
                    yes: 1,
                    no: 0,
                    abstain: None,
                },
                voting_record: VotingRecord::UserVotes(
                    vec![(ParticipantId::from_u128(2), VoteOption::Yes)]
                        .into_iter()
                        .collect::<HashMap<ParticipantId, VoteOption>>(),
                ),
            },
        }))
        .unwrap();

        let expected = json!({
            "message": "updated",
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "yes": 1,
            "no": 0,
            "voting_record": {
                "00000000-0000-0000-0000-000000000002": "yes",
            },
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_updated_legal_vote_event() {
        let produced: LegalVoteEvent = serde_json::from_value(json!({
            "message": "updated",
               "legal_vote_id": "00000000-0000-0000-0000-000000000001",
               "yes": 1,
               "no": 0,
               "voting_record": {
                   "00000000-0000-0000-0000-000000000002": "yes",
               },
        }))
        .unwrap();

        let expected = LegalVoteEvent::Updated(VoteResults {
            legal_vote_id: LegalVoteId::from_u128(1),
            results: Results {
                tally: Tally {
                    yes: 1,
                    no: 0,
                    abstain: None,
                },
                voting_record: VotingRecord::UserVotes(
                    vec![(ParticipantId::from_u128(2), VoteOption::Yes)]
                        .into_iter()
                        .collect::<HashMap<ParticipantId, VoteOption>>(),
                ),
            },
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_stopped_legal_vote_event() {
        let produced = serde_json::to_value(LegalVoteEvent::Stopped(Stopped {
            legal_vote_id: LegalVoteId::from_u128(1),
            results: FinalResults::Invalid(Invalid::AbstainDisabled),
            kind: StopKind::Auto,
            end_time: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
        }))
        .unwrap();

        let expected = json!({
            "message": "stopped",
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "results": "invalid",
            "reason": "abstain_disabled",
            "kind": "auto",
            "end_time":"2025-01-01T00:00:00Z",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_stopped_legal_vote_event() {
        let produced: LegalVoteEvent = serde_json::from_value(json!({
            "message": "stopped",
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "results": "invalid",
            "reason": "abstain_disabled",
            "kind": "auto",
            "end_time":"2025-01-01T00:00:00Z",
        }))
        .unwrap();

        let expected = LegalVoteEvent::Stopped(Stopped {
            legal_vote_id: LegalVoteId::from_u128(1),
            results: FinalResults::Invalid(Invalid::AbstainDisabled),
            kind: StopKind::Auto,
            end_time: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_canceled_legal_vote_event() {
        let produced = serde_json::to_value(LegalVoteEvent::Canceled(Canceled {
            legal_vote_id: LegalVoteId::from_u128(1),
            reason: CancelReason::RoomDestroyed,
            end_time: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
        }))
        .unwrap();

        let expected = json!({
            "message": "canceled",
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "reason": "room_destroyed",
            "end_time":"2025-01-01T00:00:00Z",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_canceled_legal_vote_event() {
        let produced: LegalVoteEvent = serde_json::from_value(json!({
            "message": "canceled",
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "reason": "room_destroyed",
            "end_time":"2025-01-01T00:00:00Z",
        }))
        .unwrap();

        let expected = LegalVoteEvent::Canceled(Canceled {
            legal_vote_id: LegalVoteId::from_u128(1),
            reason: CancelReason::RoomDestroyed,
            end_time: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_reported_issue_legal_vote_event() {
        let produced = serde_json::to_value(LegalVoteEvent::ReportedIssue(ReportedIssue {
            legal_vote_id: LegalVoteId::from_u128(1),
            participant_id: None,
            issue: Issue::Other(OtherIssue {
                description: "Test Description".to_string(),
            }),
        }))
        .unwrap();

        let expected = json!({
            "message": "reported_issue",
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "description": "Test Description",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_reported_issue_legal_vote_event() {
        let produced: LegalVoteEvent = serde_json::from_value(json!({
            "message": "reported_issue",
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "description": "Test Description",
        }))
        .unwrap();

        let expected = LegalVoteEvent::ReportedIssue(ReportedIssue {
            legal_vote_id: LegalVoteId::from_u128(1),
            participant_id: None,
            issue: Issue::Other(OtherIssue {
                description: "Test Description".to_string(),
            }),
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_error_legal_vote_event() {
        let produced = serde_json::to_value(LegalVoteEvent::Error(ErrorKind::Internal)).unwrap();

        let expected = json!({
            "message": "error",
            "error": "internal",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_error_issue_legal_vote_event() {
        let produced: LegalVoteEvent = serde_json::from_value(json!({
            "message": "error",
            "error": "internal",
        }))
        .unwrap();

        let expected = LegalVoteEvent::Error(ErrorKind::Internal);

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_pdf_asset_legal_vote_event() {
        let produced = serde_json::to_value(LegalVoteEvent::PdfAsset(PdfAsset {
            filename: "test_filename".to_string(),
            legal_vote_id: LegalVoteId::from_u128(1),
            asset_id: AssetId::from_u128(2),
        }))
        .unwrap();

        let expected = json!({
            "message": "pdf_asset",
            "filename": "test_filename",
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "asset_id": "00000000-0000-0000-0000-000000000002",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_pdf_asset_issue_legal_vote_event() {
        let produced: LegalVoteEvent = serde_json::from_value(json!({
            "message": "pdf_asset",
            "filename": "test_filename",
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "asset_id": "00000000-0000-0000-0000-000000000002",
        }))
        .unwrap();

        let expected = LegalVoteEvent::PdfAsset(PdfAsset {
            filename: "test_filename".to_string(),
            legal_vote_id: LegalVoteId::from_u128(1),
            asset_id: AssetId::from_u128(2),
        });

        assert_eq!(produced, expected);
    }
}
