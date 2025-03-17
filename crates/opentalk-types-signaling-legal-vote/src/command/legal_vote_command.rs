// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use crate::{
    command::{Cancel, GeneratePdf, ReportIssue, Stop, Vote},
    user_parameters::UserParameters,
};

/// An incoming message issued by an participant
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case", tag = "action")
)]
pub enum LegalVoteCommand {
    /// Start a new vote
    Start(UserParameters),

    /// Stop a vote and show results to the participants
    Stop(Stop),

    /// Cancel a vote
    Cancel(Cancel),

    /// Vote for an item on a vote
    Vote(Vote),

    /// Report an issue to the vote creator
    ReportIssue(ReportIssue),

    /// Generate a PDF from a passed vote
    GeneratePdf(GeneratePdf),
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use std::str::FromStr;

    use opentalk_types_signaling::ParticipantId;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;
    use crate::{
        cancel::CustomCancelReason,
        issue::{Issue, TechnicalIssue, TechnicalIssueKind},
        token::Token,
        user_parameters::{AllowedParticipants, Name},
        vote::{LegalVoteId, VoteKind, VoteOption},
    };

    #[test]
    fn serialization_start_command() {
        let produced = serde_json::to_value(LegalVoteCommand::Start(UserParameters {
            kind: VoteKind::RollCall,
            name: Name::try_from("Vote Test").unwrap(),
            subtitle: None,
            topic: None,
            allowed_participants: AllowedParticipants::try_from(vec![ParticipantId::from_u128(1)])
                .unwrap(),
            enable_abstain: false,
            auto_close: false,
            duration: None,
            create_pdf: false,
            timezone: None,
        }))
        .unwrap();

        let expected = json!({
            "action": "start",
            "kind": "roll_call",
            "name": "Vote Test",
            "allowed_participants": ["00000000-0000-0000-0000-000000000001"],
            "enable_abstain": false,
            "auto_close": false,
            "create_pdf": false
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserializiation_start_command() {
        let produced: LegalVoteCommand = serde_json::from_value(json!({
            "action": "start",
            "kind": "roll_call",
            "name": "Vote Test",
            "allowed_participants": ["00000000-0000-0000-0000-000000000001"],
            "enable_abstain": false,
            "auto_close": false,
            "create_pdf": false
        }))
        .unwrap();

        let expected = LegalVoteCommand::Start(UserParameters {
            kind: VoteKind::RollCall,
            name: Name::try_from("Vote Test").unwrap(),
            subtitle: None,
            topic: None,
            allowed_participants: AllowedParticipants::try_from(vec![ParticipantId::from_u128(1)])
                .unwrap(),
            enable_abstain: false,
            auto_close: false,
            duration: None,
            create_pdf: false,
            timezone: None,
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_stop_command() {
        let produced = serde_json::to_value(LegalVoteCommand::Stop(Stop {
            legal_vote_id: LegalVoteId::from_u128(1),
        }))
        .unwrap();

        let expected = json!({
            "action": "stop",
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserializiation_stop_command() {
        let produced: LegalVoteCommand = serde_json::from_value(json!({
            "action": "stop",
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
        }))
        .unwrap();

        let expected = LegalVoteCommand::Stop(Stop {
            legal_vote_id: LegalVoteId::from_u128(1),
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_cancel_command() {
        let produced = serde_json::to_value(LegalVoteCommand::Cancel(Cancel {
            legal_vote_id: LegalVoteId::from_u128(1),
            reason: CustomCancelReason::try_from("Test Reason").unwrap(),
        }))
        .unwrap();

        let expected = json!({
            "action": "cancel",
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "reason": "Test Reason",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserializiation_cancel_command() {
        let produced: LegalVoteCommand = serde_json::from_value(json!({
            "action": "cancel",
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "reason": "Test Reason",
        }))
        .unwrap();

        let expected = LegalVoteCommand::Cancel(Cancel {
            legal_vote_id: LegalVoteId::from_u128(1),
            reason: CustomCancelReason::try_from("Test Reason").unwrap(),
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_vote_command() {
        let produced = serde_json::to_value(LegalVoteCommand::Vote(Vote {
            legal_vote_id: LegalVoteId::from_u128(1),
            option: VoteOption::Yes,
            token: Token::from_str("1111Cn8eVZg").unwrap(),
        }))
        .unwrap();

        let expected = json!({
            "action": "vote",
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "option": "yes",
            "token": "1111Cn8eVZg",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserializiation_vote_command() {
        let produced: LegalVoteCommand = serde_json::from_value(json!({
            "action": "vote",
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "option": "yes",
            "token": "1111Cn8eVZg",
        }))
        .unwrap();

        let expected = LegalVoteCommand::Vote(Vote {
            legal_vote_id: LegalVoteId::from_u128(1),
            option: VoteOption::Yes,
            token: Token::from_str("1111Cn8eVZg").unwrap(),
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_report_issue_command() {
        let produced = serde_json::to_value(LegalVoteCommand::ReportIssue(ReportIssue {
            legal_vote_id: LegalVoteId::from_u128(1),
            issue: Issue::Technical(TechnicalIssue {
                kind: TechnicalIssueKind::Audio,
                description: None,
            }),
        }))
        .unwrap();

        let expected = json!({
            "action": "report_issue",
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "kind": "audio"
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserializiation_report_issue_command() {
        let produced: LegalVoteCommand = serde_json::from_value(json!({
            "action": "report_issue",
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "kind": "audio",
        }))
        .unwrap();

        let expected = LegalVoteCommand::ReportIssue(ReportIssue {
            legal_vote_id: LegalVoteId::from_u128(1),
            issue: Issue::Technical(TechnicalIssue {
                kind: TechnicalIssueKind::Audio,
                description: None,
            }),
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_generate_pdf_command() {
        let produced = serde_json::to_value(LegalVoteCommand::GeneratePdf(GeneratePdf {
            legal_vote_id: LegalVoteId::from_u128(1),
            timezone: None,
        }))
        .unwrap();

        let expected = json!({
            "action": "generate_pdf",
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserializiation_generate_pdf_command() {
        let produced: LegalVoteCommand = serde_json::from_value(json!({
            "action": "generate_pdf",
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
        }))
        .unwrap();

        let expected = LegalVoteCommand::GeneratePdf(GeneratePdf {
            legal_vote_id: LegalVoteId::from_u128(1),
            timezone: None,
        });

        assert_eq!(produced, expected);
    }
}
