// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use crate::vote::LegalVoteId;

/// Represents a request to generate a PDF for a specific vote.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GeneratePdf {
    /// The identifier of the targeted vote.
    pub legal_vote_id: LegalVoteId,

    /// An optional timezone for the PDF generation. Defaults to UTC.
    /// The timezone should be in a format standardized by IANA (e.g., "CET" or "Europe/Vienna").
    /// For more details, visit: <https://www.iana.org/time-zones>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub timezone: Option<chrono_tz::Tz>,
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn serialization() {
        let produced = serde_json::to_value(GeneratePdf {
            legal_vote_id: LegalVoteId::from_u128(1),
            timezone: Some(chrono_tz::Tz::Europe__Berlin),
        })
        .unwrap();

        let expected = json!({
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "timezone": "Europe/Berlin",
        });

        assert_eq!(produced, expected);

        let produced = serde_json::to_value(GeneratePdf {
            legal_vote_id: LegalVoteId::from_u128(1),
            timezone: None,
        })
        .unwrap();

        let expected = json!({
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization() {
        let produced: GeneratePdf = serde_json::from_value(json!({
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "timezone": "Europe/Berlin",
        }))
        .unwrap();

        let expected = GeneratePdf {
            legal_vote_id: LegalVoteId::from_u128(1),
            timezone: Some(chrono_tz::Tz::Europe__Berlin),
        };

        assert_eq!(produced, expected);

        let produced: GeneratePdf = serde_json::from_value(json!({
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
        }))
        .unwrap();

        let expected = GeneratePdf {
            legal_vote_id: LegalVoteId::from_u128(1),
            timezone: None,
        };

        assert_eq!(produced, expected);
    }
}
