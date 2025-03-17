// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use crate::{event::Response, vote::LegalVoteId};

/// The direct response to an issued vote request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VoteResponse {
    /// The vote id of the requested vote
    pub legal_vote_id: LegalVoteId,

    /// The response to the vote request
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub response: Response,
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use std::str::FromStr;

    use opentalk_types_signaling::ParticipantId;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;
    use crate::{event::VoteSuccess, token::Token, vote::VoteOption};

    #[test]
    fn serialization() {
        let produced = serde_json::to_value(VoteResponse {
            legal_vote_id: LegalVoteId::from_u128(1),
            response: Response::Success(VoteSuccess {
                vote_option: VoteOption::No,
                issuer: ParticipantId::from_u128(2),
                consumed_token: Token::from_str("1111Cn8eVZg").unwrap(),
            }),
        })
        .unwrap();

        let expected = json!({
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "response": "success",
            "vote_option": "no",
            "issuer": "00000000-0000-0000-0000-000000000002",
            "consumed_token": "1111Cn8eVZg",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization() {
        let produced: VoteResponse = serde_json::from_value(json!({
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "response": "success",
            "vote_option": "no",
            "issuer": "00000000-0000-0000-0000-000000000002",
            "consumed_token": "1111Cn8eVZg",
        }))
        .unwrap();

        let expected = VoteResponse {
            legal_vote_id: LegalVoteId::from_u128(1),
            response: Response::Success(VoteSuccess {
                vote_option: VoteOption::No,
                issuer: ParticipantId::from_u128(2),
                consumed_token: Token::from_str("1111Cn8eVZg").unwrap(),
            }),
        };

        assert_eq!(produced, expected);
    }
}
