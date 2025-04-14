// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use crate::{event::Results, invalid::Invalid};

/// The final results for a vote
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case", tag = "results")
)]
pub enum FinalResults {
    /// Valid final results
    Valid(Results),

    /// Invalid final results
    Invalid(Invalid),
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use std::collections::HashMap;

    use opentalk_types_signaling::ParticipantId;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;
    use crate::{event::VotingRecord, tally::Tally, vote::VoteOption};

    #[test]
    fn serialization_valid_final_results() {
        let produced = serde_json::to_value(FinalResults::Valid(Results {
            tally: Tally {
                yes: 1,
                no: 0,
                abstain: None,
            },
            voting_record: VotingRecord::UserVotes(
                vec![(ParticipantId::from_u128(1), VoteOption::Yes)]
                    .into_iter()
                    .collect::<HashMap<ParticipantId, VoteOption>>(),
            ),
        }))
        .unwrap();

        let expected = json!({
            "results": "valid",
            "yes": 1,
            "no": 0,
            "voting_record": {
                "00000000-0000-0000-0000-000000000001": "yes"
            },
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_valid_final_results() {
        let produced: FinalResults = serde_json::from_value(json!({
            "results": "valid",
            "yes": 1,
            "no": 0,
            "voting_record": {
                "00000000-0000-0000-0000-000000000001": "yes"
            },
        }))
        .unwrap();

        let expected = FinalResults::Valid(Results {
            tally: Tally {
                yes: 1,
                no: 0,
                abstain: None,
            },
            voting_record: VotingRecord::UserVotes(
                vec![(ParticipantId::from_u128(1), VoteOption::Yes)]
                    .into_iter()
                    .collect::<HashMap<ParticipantId, VoteOption>>(),
            ),
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_invalid_final_results() {
        let produced =
            serde_json::to_value(FinalResults::Invalid(Invalid::AbstainDisabled)).unwrap();

        let expected = json!({
            "results": "invalid",
            "reason": "abstain_disabled",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_invalid_final_results() {
        let produced: FinalResults = serde_json::from_value(json!({
            "results": "invalid",
            "reason": "abstain_disabled",
        }))
        .unwrap();

        let expected = FinalResults::Invalid(Invalid::AbstainDisabled);

        assert_eq!(produced, expected);
    }
}
