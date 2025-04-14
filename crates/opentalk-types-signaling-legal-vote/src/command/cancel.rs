// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use crate::{cancel::CustomCancelReason, vote::LegalVoteId};

/// Cancel a vote
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Cancel {
    /// The vote id of the targeted vote
    pub legal_vote_id: LegalVoteId,

    /// The reason for the cancel
    pub reason: CustomCancelReason,
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn serialization() {
        let produced = serde_json::to_value(Cancel {
            legal_vote_id: LegalVoteId::from_u128(1),
            reason: CustomCancelReason::try_from("Test Cancel Reason").unwrap(),
        })
        .unwrap();

        let expected = json!({
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "reason": "Test Cancel Reason"
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization() {
        let produced: Cancel = serde_json::from_value(json!({
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "reason": "Test Cancel Reason"
        }))
        .unwrap();

        let expected = Cancel {
            legal_vote_id: LegalVoteId::from_u128(1),
            reason: CustomCancelReason::try_from("Test Cancel Reason").unwrap(),
        };

        assert_eq!(produced, expected);
    }
}
