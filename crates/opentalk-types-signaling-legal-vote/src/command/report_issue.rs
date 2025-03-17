// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use crate::{issue::Issue, vote::LegalVoteId};

/// Represents a reported issue with a vote.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReportIssue {
    /// The identifier of the affected vote.
    pub legal_vote_id: LegalVoteId,

    /// The details of the reported issue.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub issue: Issue,
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;
    use crate::issue::{TechnicalIssue, TechnicalIssueKind};

    #[test]
    fn serialization() {
        let produced = serde_json::to_value(ReportIssue {
            legal_vote_id: LegalVoteId::from_u128(1),
            issue: Issue::Technical(TechnicalIssue {
                kind: TechnicalIssueKind::Video,
                description: None,
            }),
        })
        .unwrap();

        let expected = json!({
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "kind": "video"
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization() {
        let produced: ReportIssue = serde_json::from_value(json!({
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "kind": "video"
        }))
        .unwrap();

        let expected = ReportIssue {
            legal_vote_id: LegalVoteId::from_u128(1),
            issue: Issue::Technical(TechnicalIssue {
                kind: TechnicalIssueKind::Video,
                description: None,
            }),
        };

        assert_eq!(produced, expected);
    }
}
