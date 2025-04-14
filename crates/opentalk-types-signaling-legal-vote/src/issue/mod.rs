// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Signaling issue messages for the `legal-vote` namespace.

mod other_issue;
mod technical_issue;
mod technical_issue_kind;

pub use other_issue::OtherIssue;
pub use technical_issue::TechnicalIssue;
pub use technical_issue_kind::TechnicalIssueKind;

/// Represents an issue reported during the vote.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(untagged)
)]
pub enum Issue {
    /// A technical issue, such as audio or video problems.
    Technical(TechnicalIssue),

    /// A general issue that does not fall under technical problems.
    Other(OtherIssue),
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn serialization_technical_issue() {
        let produced = serde_json::to_value(Issue::Technical(TechnicalIssue {
            kind: TechnicalIssueKind::Audio,
            description: None,
        }))
        .unwrap();

        let expected = json!({
            "kind": "audio",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_technical_issue() {
        let produced: Issue = serde_json::from_value(json!({
            "kind": "audio",
        }))
        .unwrap();

        let expected = Issue::Technical(TechnicalIssue {
            kind: TechnicalIssueKind::Audio,
            description: None,
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_other_issue() {
        let produced = serde_json::to_value(Issue::Other(OtherIssue {
            description: "Test Description".to_string(),
        }))
        .unwrap();

        let expected = json!({
            "description": "Test Description",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_other_issue() {
        let produced: Issue = serde_json::from_value(json!({
            "description": "Test Description",
        }))
        .unwrap();

        let expected = Issue::Other(OtherIssue {
            description: "Test Description".to_string(),
        });

        assert_eq!(produced, expected);
    }
}
