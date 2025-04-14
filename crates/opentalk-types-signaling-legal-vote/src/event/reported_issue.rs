// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use opentalk_types_signaling::ParticipantId;

use crate::{issue::Issue, vote::LegalVoteId};

/// Represents an issue reported during a vote.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReportedIssue {
    /// The identifier of the affected vote.
    pub legal_vote_id: LegalVoteId,

    /// The participant who reported the issue, if applicable.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub participant_id: Option<ParticipantId>,

    /// Details of the reported issue.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub issue: Issue,
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;
    use crate::issue::OtherIssue;

    #[test]
    fn serialization() {
        let produced = serde_json::to_value(ReportedIssue {
            legal_vote_id: LegalVoteId::from_u128(1),
            participant_id: Some(ParticipantId::from_u128(2)),
            issue: Issue::Other(OtherIssue {
                description: "Test Description".to_string(),
            }),
        })
        .unwrap();

        let expected = json!({
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "participant_id": "00000000-0000-0000-0000-000000000002",
            "description": "Test Description",
        });

        assert_eq!(produced, expected);

        let produced = serde_json::to_value(ReportedIssue {
            legal_vote_id: LegalVoteId::from_u128(1),
            participant_id: None,
            issue: Issue::Other(OtherIssue {
                description: "Test Description".to_string(),
            }),
        })
        .unwrap();

        let expected = json!({
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "description": "Test Description",

        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization() {
        let produced: ReportedIssue = serde_json::from_value(json!({
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "participant_id": "00000000-0000-0000-0000-000000000002",
            "description": "Test Description",
        }))
        .unwrap();

        let expected = ReportedIssue {
            legal_vote_id: LegalVoteId::from_u128(1),
            participant_id: Some(ParticipantId::from_u128(2)),
            issue: Issue::Other(OtherIssue {
                description: "Test Description".to_string(),
            }),
        };

        assert_eq!(produced, expected);

        let produced: ReportedIssue = serde_json::from_value(json!({
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "description": "Test Description",
        }))
        .unwrap();

        let expected = ReportedIssue {
            legal_vote_id: LegalVoteId::from_u128(1),
            participant_id: None,
            issue: Issue::Other(OtherIssue {
                description: "Test Description".to_string(),
            }),
        };

        assert_eq!(produced, expected);
    }
}
