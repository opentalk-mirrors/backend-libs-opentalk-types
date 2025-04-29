// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Signaling invalid message for the `legal-vote` namespace.

/// Describes the reason for invalid vote results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case", tag = "reason")
)]
pub enum Invalid {
    /// An abstain vote was found when the vote itself has abstain disabled.
    AbstainDisabled,

    /// The protocols vote count is not equal to the votes vote count.
    VoteCountInconsistent,

    /// The protocol entries are inconsistent.
    ProtocolInconsistent,
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn serialization_abstain_disabled_invalid() {
        let produced = serde_json::to_value(Invalid::AbstainDisabled).unwrap();

        let expected = json!({
            "reason": "abstain_disabled",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_abstain_disabled_invalid() {
        let produced: Invalid = serde_json::from_value(json!({
            "reason": "abstain_disabled",
        }))
        .unwrap();

        let expected = Invalid::AbstainDisabled;

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_vote_count_inconsistent_invalid() {
        let produced = serde_json::to_value(Invalid::VoteCountInconsistent).unwrap();

        let expected = json!({
            "reason": "vote_count_inconsistent",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_vote_count_inconsistent_invalid() {
        let produced: Invalid = serde_json::from_value(json!({
            "reason": "vote_count_inconsistent",
        }))
        .unwrap();

        let expected = Invalid::VoteCountInconsistent;

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_protocol_inconsistent_invalid() {
        let produced = serde_json::to_value(Invalid::ProtocolInconsistent).unwrap();

        let expected = json!({
            "reason": "protocol_inconsistent",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_protocol_inconsistent_invalid() {
        let produced: Invalid = serde_json::from_value(json!({
            "reason": "protocol_inconsistent",
        }))
        .unwrap();

        let expected = Invalid::ProtocolInconsistent;

        assert_eq!(produced, expected);
    }
}
