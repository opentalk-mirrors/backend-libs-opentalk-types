// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use opentalk_types_common::assets::AssetId;

use crate::vote::LegalVoteId;

/// Represents a PDF asset associated with a vote.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PdfAsset {
    /// The filename of the PDF.
    pub filename: String,

    /// The identifier of the related vote.
    pub legal_vote_id: LegalVoteId,

    /// The unique identifier of the asset.
    pub asset_id: AssetId,
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn serialization() {
        let produced = serde_json::to_value(PdfAsset {
            filename: "test_filename".to_string(),
            legal_vote_id: LegalVoteId::from_u128(1),
            asset_id: AssetId::from_u128(2),
        })
        .unwrap();

        let expected = json!({
            "filename": "test_filename",
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "asset_id": "00000000-0000-0000-0000-000000000002",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization() {
        let produced: PdfAsset = serde_json::from_value(json!({
            "filename": "test_filename",
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "asset_id": "00000000-0000-0000-0000-000000000002",
        }))
        .unwrap();

        let expected = PdfAsset {
            filename: "test_filename".to_string(),
            legal_vote_id: LegalVoteId::from_u128(1),
            asset_id: AssetId::from_u128(2),
        };

        assert_eq!(produced, expected);
    }
}
