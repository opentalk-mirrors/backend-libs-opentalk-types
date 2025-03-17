// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use chrono::{DateTime, Utc};

use crate::{cancel::CancelReason, vote::LegalVoteId};

/// Represents a canceled vote.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Canceled {
    /// The identifier of the canceled vote.
    pub legal_vote_id: LegalVoteId,

    /// The reason for canceling the vote.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub reason: CancelReason,

    /// The timestamp when the vote was canceled.
    pub end_time: DateTime<Utc>,
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use chrono::TimeZone;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn serialization() {
        let produced = serde_json::to_value(Canceled {
            legal_vote_id: LegalVoteId::from_u128(1),
            reason: CancelReason::RoomDestroyed,
            end_time: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
        })
        .unwrap();

        let expected = json!({
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "reason": "room_destroyed",
            "end_time":"2025-01-01T00:00:00Z",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization() {
        let produced: Canceled = serde_json::from_value(json!({
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "reason": "room_destroyed",
            "end_time":"2025-01-01T00:00:00Z",
        }))
        .unwrap();

        let expected = Canceled {
            legal_vote_id: LegalVoteId::from_u128(1),
            reason: CancelReason::RoomDestroyed,
            end_time: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
        };

        assert_eq!(produced, expected);
    }
}
