// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::convert::TryFrom;

use opentalk_types_signaling::ParticipantId;
use snafu::{ensure, Snafu};

/// Minimum required number of participants.
pub const MIN_PARTICIPANTS: usize = 1;

/// A validated list of allowed participants, ensuring at least one participant.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(try_from = "Vec<ParticipantId>")
)]
pub struct AllowedParticipants(Vec<ParticipantId>);

/// Error when parsing [`AllowedParticipants`].
#[derive(Debug, Snafu)]
pub enum TryFromAllowedParticipantsError {
    /// The list of participants is empty.
    #[snafu(display("AllowedParticipants must contain at least {min_length} participant(s)."))]
    TooFew {
        /// The minimum length the participant list has to be.
        min_length: usize,
    },
}

impl TryFrom<Vec<ParticipantId>> for AllowedParticipants {
    type Error = TryFromAllowedParticipantsError;

    fn try_from(value: Vec<ParticipantId>) -> Result<Self, Self::Error> {
        ensure!(
            value.len() >= MIN_PARTICIPANTS,
            TooFewSnafu {
                min_length: MIN_PARTICIPANTS
            }
        );
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allowed_participants_min_length() {
        assert!(
            AllowedParticipants::try_from(vec![]).is_err(),
            "AllowedParticipants must contain at least {MIN_PARTICIPANTS} participant(s)."
        );

        assert!(
            AllowedParticipants::try_from(vec![ParticipantId::from_u128(1)]).is_ok(),
            "AllowedParticipants should accept a list with at least one participant."
        );
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    struct TestStruct {
        participants: AllowedParticipants,
    }

    #[test]
    fn serialization() {
        let produced = serde_json::to_value(TestStruct {
            participants: AllowedParticipants::try_from(vec![ParticipantId::from_u128(1)]).unwrap(),
        })
        .unwrap();
        let expected = json!({ "participants": vec![ParticipantId::from_u128(1)] });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization() {
        let produced: TestStruct =
            serde_json::from_value(json!({ "participants": vec![ParticipantId::from_u128(1)] }))
                .unwrap();
        let expected = TestStruct {
            participants: AllowedParticipants::try_from(vec![ParticipantId::from_u128(1)]).unwrap(),
        };
        assert_eq!(produced, expected);

        let produced: Result<TestStruct, _> = serde_json::from_value(json!({ "participants": [] }));
        assert!(produced.is_err());
    }
}
