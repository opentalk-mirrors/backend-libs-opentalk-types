// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use crate::vote::LegalVoteId;

/// Stop a vote
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Stop {
    /// The vote id of the targeted vote
    pub legal_vote_id: LegalVoteId,
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn serialization() {
        let produced = serde_json::to_value(Stop {
            legal_vote_id: LegalVoteId::from_u128(1),
        })
        .unwrap();

        let expected = json!({
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization() {
        let produced: Stop = serde_json::from_value(json!({
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
        }))
        .unwrap();

        let expected = Stop {
            legal_vote_id: LegalVoteId::from_u128(1),
        };

        assert_eq!(produced, expected);
    }
}
