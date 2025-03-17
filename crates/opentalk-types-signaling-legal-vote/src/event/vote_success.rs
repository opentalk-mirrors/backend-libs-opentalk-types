// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use opentalk_types_signaling::ParticipantId;

use crate::{token::Token, vote::VoteOption};

/// Gets send if a participant votes successfully.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct VoteSuccess {
    /// The vote option the participant chose.
    pub vote_option: VoteOption,

    /// The participant who issued the vote.
    pub issuer: ParticipantId,

    /// The token that was consumed during the vote.
    pub consumed_token: Token,
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use std::str::FromStr;

    use opentalk_types_signaling::ParticipantId;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;
    use crate::vote::VoteOption;

    #[test]
    fn serialization() {
        let produced = serde_json::to_value(VoteSuccess {
            vote_option: VoteOption::No,
            issuer: ParticipantId::from_u128(1),
            consumed_token: Token::from_str("1111Cn8eVZg").unwrap(),
        })
        .unwrap();

        let expected = json!({
            "vote_option": "no",
            "issuer": "00000000-0000-0000-0000-000000000001",
            "consumed_token": "1111Cn8eVZg",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization() {
        let produced: VoteSuccess = serde_json::from_value(json!({
            "vote_option": "no",
            "issuer": "00000000-0000-0000-0000-000000000001",
            "consumed_token": "1111Cn8eVZg",
        }))
        .unwrap();

        let expected = VoteSuccess {
            vote_option: VoteOption::No,
            issuer: ParticipantId::from_u128(1),
            consumed_token: Token::from_str("1111Cn8eVZg").unwrap(),
        };

        assert_eq!(produced, expected);
    }
}
