// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use chrono::{DateTime, Utc};

use crate::{
    event::FinalResults,
    vote::{LegalVoteId, StopKind},
};

/// Represents a stop message for a voting process.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Stopped {
    /// The unique identifier of the vote.
    pub legal_vote_id: LegalVoteId,

    /// The final results of the vote.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub results: FinalResults,

    /// The reason for stopping the vote.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub kind: StopKind,

    /// The timestamp when the voting process ended.
    pub end_time: DateTime<Utc>,
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {

    use chrono::TimeZone;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;
    use crate::invalid::Invalid;

    #[test]
    fn serialization() {
        let produced = serde_json::to_value(Stopped {
            legal_vote_id: LegalVoteId::from_u128(1),
            results: FinalResults::Invalid(Invalid::AbstainDisabled),
            kind: StopKind::Auto,
            end_time: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
        })
        .unwrap();

        let expected = json!({
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "results": "invalid",
            "reason": "abstain_disabled",
            "stop_kind": "auto",
            "end_time":"2025-01-01T00:00:00Z",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization() {
        let produced: Stopped = serde_json::from_value(json!({
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "results": "invalid",
            "reason": "abstain_disabled",
            "stop_kind": "auto",
            "end_time":"2025-01-01T00:00:00Z",
        }))
        .unwrap();

        let expected = Stopped {
            legal_vote_id: LegalVoteId::from_u128(1),
            results: FinalResults::Invalid(Invalid::AbstainDisabled),
            kind: StopKind::Auto,
            end_time: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
        };

        assert_eq!(produced, expected);
    }
}
