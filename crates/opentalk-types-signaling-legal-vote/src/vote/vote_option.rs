// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

/// Represents the possible choices a voter can make in the voting process.
///
/// The `Abstain` option can be disabled through the vote parameters (See
/// [`UserParameters`](crate::user_parameters::UserParameters)).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(
    feature = "redis",
    derive(redis_args::ToRedisArgs, redis_args::FromRedisValue),
    to_redis_args(serde),
    from_redis_value(serde)
)]
pub enum VoteOption {
    /// Indicates a vote in favor of the proposal.
    Yes,

    /// Indicates a vote against the proposal.
    No,

    /// Indicates the voter is abstaining from voting.
    ///
    /// This option can be disabled based on vote parameters.
    Abstain,
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn serialization_yes_vote_option() {
        let produced = serde_json::to_value(VoteOption::Yes).unwrap();

        let expected = json!("yes");

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_yes_vote_option() {
        let produced: VoteOption = serde_json::from_value(json!("yes")).unwrap();

        let expected = VoteOption::Yes;

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_no_vote_option() {
        let produced = serde_json::to_value(VoteOption::No).unwrap();

        let expected = json!("no");

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_no_vote_option() {
        let produced: VoteOption = serde_json::from_value(json!("no")).unwrap();

        let expected = VoteOption::No;

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_abstain_vote_option() {
        let produced = serde_json::to_value(VoteOption::Abstain).unwrap();

        let expected = json!("abstain");

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_abstain_vote_option() {
        let produced: VoteOption = serde_json::from_value(json!("abstain")).unwrap();

        let expected = VoteOption::Abstain;

        assert_eq!(produced, expected);
    }
}
