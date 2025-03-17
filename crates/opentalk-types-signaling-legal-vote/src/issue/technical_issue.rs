// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use crate::issue::TechnicalIssueKind;

/// Represents a technical issue reported during the vote.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TechnicalIssue {
    /// The kind of technical issue.
    pub kind: TechnicalIssueKind,

    /// A description of the technical issue, if available.
    ///
    /// Is `None` if no description is provided.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<String>,
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn serialization() {
        let produced = serde_json::to_value(TechnicalIssue {
            kind: TechnicalIssueKind::Audio,
            description: Some("Test Description".to_string()),
        })
        .unwrap();

        let expected = json!({
            "kind": "audio",
            "description": "Test Description",
        });

        assert_eq!(produced, expected);

        let produced = serde_json::to_value(TechnicalIssue {
            kind: TechnicalIssueKind::Audio,
            description: None,
        })
        .unwrap();

        let expected = json!({
            "kind": "audio",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization() {
        let produced: TechnicalIssue = serde_json::from_value(json!({
            "kind": "audio",
            "description": "Test Description",
        }))
        .unwrap();

        let expected = TechnicalIssue {
            kind: TechnicalIssueKind::Audio,
            description: Some("Test Description".to_string()),
        };

        assert_eq!(produced, expected);

        let produced: TechnicalIssue = serde_json::from_value(json!({
            "kind": "audio",
        }))
        .unwrap();

        let expected = TechnicalIssue {
            kind: TechnicalIssueKind::Audio,
            description: None,
        };

        assert_eq!(produced, expected);
    }
}
