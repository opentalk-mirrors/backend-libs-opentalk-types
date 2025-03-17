// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

/// Reasons for a failed vote request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case", tag = "reason")
)]
pub enum VoteFailed {
    /// The given vote id is not active or does not exist
    InvalidVoteId,

    /// The requesting user already voted or is ineligible to vote. (requires the vote parameter `auto_close` to be true)
    Ineligible,

    /// Invalid vote option
    InvalidOption,
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {

    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn serialization_invalid_vote_id_vote_failed() {
        let produced = serde_json::to_value(VoteFailed::InvalidVoteId).unwrap();

        let expected = json!({"reason": "invalid_vote_id"});

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_invalid_vote_id_vote_failed() {
        let produced: VoteFailed =
            serde_json::from_value(json!({"reason": "invalid_vote_id"})).unwrap();

        let expected = VoteFailed::InvalidVoteId;

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_ineligible_vote_failed() {
        let produced = serde_json::to_value(VoteFailed::Ineligible).unwrap();

        let expected = json!({"reason": "ineligible"});

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_ineligible_vote_failed() {
        let produced: VoteFailed = serde_json::from_value(json!({"reason": "ineligible"})).unwrap();

        let expected = VoteFailed::Ineligible;

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_invalid_option_vote_failed() {
        let produced = serde_json::to_value(VoteFailed::InvalidOption).unwrap();

        let expected = json!({"reason": "invalid_option"});

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_invalid_option_vote_failed() {
        let produced: VoteFailed =
            serde_json::from_value(json!({"reason": "invalid_option"})).unwrap();

        let expected = VoteFailed::InvalidOption;

        assert_eq!(produced, expected);
    }
}
