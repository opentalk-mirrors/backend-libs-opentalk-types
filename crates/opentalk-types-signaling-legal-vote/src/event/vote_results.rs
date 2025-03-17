// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use crate::{event::Results, vote::LegalVoteId};

/// The results for a vote
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VoteResults {
    /// The vote id
    pub legal_vote_id: LegalVoteId,

    /// The vote results
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub results: Results,
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
    fn serialization() {
        let produced = serde_json::to_value(VoteResults {
            legal_vote_id: LegalVoteId::from_u128(1),
            results: Results {
                tally: Tally {
                    yes: 1,
                    no: 0,
                    abstain: None,
                },
                voting_record: VotingRecord::UserVotes(
                    vec![(ParticipantId::from_u128(2), VoteOption::Yes)]
                        .into_iter()
                        .collect::<HashMap<ParticipantId, VoteOption>>(),
                ),
            },
        })
        .unwrap();

        let expected = json!({
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "yes": 1,
            "no": 0,
            "voting_record": {
                "00000000-0000-0000-0000-000000000002": "yes"
            },
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization() {
        let produced: VoteResults = serde_json::from_value(json!({
            "legal_vote_id": "00000000-0000-0000-0000-000000000001",
            "yes": 1,
            "no": 0,
            "voting_record": {
                "00000000-0000-0000-0000-000000000002": "yes"
            },
        }))
        .unwrap();

        let expected = VoteResults {
            legal_vote_id: LegalVoteId::from_u128(1),
            results: Results {
                tally: Tally {
                    yes: 1,
                    no: 0,
                    abstain: None,
                },
                voting_record: VotingRecord::UserVotes(
                    vec![(ParticipantId::from_u128(2), VoteOption::Yes)]
                        .into_iter()
                        .collect::<HashMap<ParticipantId, VoteOption>>(),
                ),
            },
        };

        assert_eq!(produced, expected);
    }
}
