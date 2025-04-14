// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use crate::event::{VoteFailed, VoteSuccess};

/// Vote request response
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case", tag = "response")
)]
pub enum Response {
    /// Response for a successful vote request
    Success(VoteSuccess),

    /// Response for a failed vote request
    Failed(VoteFailed),
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use std::str::FromStr;

    use opentalk_types_signaling::ParticipantId;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;
    use crate::{token::Token, vote::VoteOption};

    #[test]
    fn serialization_success_response() {
        let produced = serde_json::to_value(Response::Success(VoteSuccess {
            vote_option: VoteOption::Yes,
            issuer: ParticipantId::from_u128(1),
            consumed_token: Token::from_str("1111Cn8eVZg").unwrap(),
        }))
        .unwrap();

        let expected = json!({
            "response": "success",
            "vote_option": "yes",
            "issuer": "00000000-0000-0000-0000-000000000001",
            "consumed_token": "1111Cn8eVZg",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_success_response() {
        let produced: Response = serde_json::from_value(json!({
            "response": "success",
            "vote_option": "yes",
            "issuer": "00000000-0000-0000-0000-000000000001",
            "consumed_token": "1111Cn8eVZg",

        }))
        .unwrap();

        let expected = Response::Success(VoteSuccess {
            vote_option: VoteOption::Yes,
            issuer: ParticipantId::from_u128(1),
            consumed_token: Token::from_str("1111Cn8eVZg").unwrap(),
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_failed_response() {
        let produced = serde_json::to_value(Response::Failed(VoteFailed::Ineligible)).unwrap();

        let expected = json!({
            "response": "failed",
            "reason": "ineligible",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_failed_response() {
        let produced: Response = serde_json::from_value(json!({
            "response": "failed",
            "reason": "ineligible",
        }))
        .unwrap();

        let expected = Response::Failed(VoteFailed::Ineligible);

        assert_eq!(produced, expected);
    }
}
