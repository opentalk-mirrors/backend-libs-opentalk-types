// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

/// Represents the different kinds of votes available in the voting system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(
    feature = "redis",
    derive(redis_args::ToRedisArgs, redis_args::FromRedisValue),
    from_redis_value(serde),
    to_redis_args(serde)
)]
pub enum VoteKind {
    /// A pseudonymous vote where all tokens used will be published with the voting results.
    Pseudonymous,

    /// A roll call vote where all participants and their votes will be published with the results.
    RollCall,

    /// A live roll call vote where votes are sent live to all participants as they are cast.
    LiveRollCall,
}

impl VoteKind {
    /// Returns whether the vote kind is hidden, meaning voter identities are not disclosed.
    ///
    /// - `Pseudonymous`: Hidden (returns `true`).
    /// - `RollCall`: Not hidden (returns `false`).
    /// - `LiveRollCall`: Not hidden (returns `false`).
    pub const fn is_hidden(&self) -> bool {
        match self {
            VoteKind::Pseudonymous => true,
            VoteKind::RollCall => false,
            VoteKind::LiveRollCall => false,
        }
    }

    /// Returns whether the vote kind is live, meaning votes are communicated in real-time to participants.
    ///
    /// - `Pseudonymous`: Not live (returns `false`).
    /// - `RollCall`: Live (returns `true`).
    /// - `LiveRollCall`: Live (returns `true`).
    pub const fn is_live(&self) -> bool {
        match self {
            VoteKind::Pseudonymous => false,
            VoteKind::RollCall => true,
            VoteKind::LiveRollCall => true,
        }
    }

    /// Returns whether the voting report contains a list of votes, which is only applicable to live votes.
    ///
    /// - `Pseudonymous`: Does not contain a list of votes (returns `false`).
    /// - `RollCall`: Does not contain a list of votes (returns `false`).
    /// - `LiveRollCall`: Contains a list of votes (returns `true`).
    pub const fn report_contains_votes_list(&self) -> bool {
        match self {
            VoteKind::Pseudonymous => false,
            VoteKind::RollCall => false,
            VoteKind::LiveRollCall => true,
        }
    }
}

impl std::fmt::Display for VoteKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Pseudonymous => "pseudonymous",
                Self::RollCall => "roll call",
                Self::LiveRollCall => "live roll call",
            }
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_hidden() {
        assert!(VoteKind::Pseudonymous.is_hidden());
        assert!(!VoteKind::RollCall.is_hidden());
        assert!(!VoteKind::LiveRollCall.is_hidden());
    }

    #[test]
    fn is_live() {
        assert!(!VoteKind::Pseudonymous.is_live());
        assert!(VoteKind::RollCall.is_live());
        assert!(VoteKind::LiveRollCall.is_live());
    }

    #[test]
    fn report_contains_votes_list() {
        assert!(!VoteKind::Pseudonymous.report_contains_votes_list());
        assert!(!VoteKind::RollCall.report_contains_votes_list());
        assert!(VoteKind::LiveRollCall.report_contains_votes_list());
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn serialization_pseudonymous_vote_kind() {
        let produced = serde_json::to_value(VoteKind::Pseudonymous).unwrap();

        let expected = json!("pseudonymous");

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_pseudonymous_vote_kind() {
        let produced: VoteKind = serde_json::from_value(json!("pseudonymous")).unwrap();

        let expected = VoteKind::Pseudonymous;

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_roll_call_vote_kind() {
        let produced = serde_json::to_value(VoteKind::RollCall).unwrap();

        let expected = json!("roll_call");

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_roll_call_vote_kind() {
        let produced: VoteKind = serde_json::from_value(json!("roll_call")).unwrap();

        let expected = VoteKind::RollCall;

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_live_roll_call_vote_kind() {
        let produced = serde_json::to_value(VoteKind::LiveRollCall).unwrap();

        let expected = json!("live_roll_call");

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_live_roll_call_vote_kind() {
        let produced: VoteKind = serde_json::from_value(json!("live_roll_call")).unwrap();

        let expected = VoteKind::LiveRollCall;

        assert_eq!(produced, expected);
    }
}
