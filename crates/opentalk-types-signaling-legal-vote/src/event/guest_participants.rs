// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use opentalk_types_signaling::ParticipantId;

/// A list of provided guest participants.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GuestParticipants {
    /// The identifiers of the guest participants.
    pub guests: Vec<ParticipantId>,
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn serialization() {
        let produced = serde_json::to_value(GuestParticipants {
            guests: vec![ParticipantId::from_u128(1)],
        })
        .unwrap();

        let expected = json!({ "guests": ["00000000-0000-0000-0000-000000000001"] });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization() {
        let produced: GuestParticipants =
            serde_json::from_value(json!({ "guests": ["00000000-0000-0000-0000-000000000001"] }))
                .unwrap();

        let expected = GuestParticipants {
            guests: vec![ParticipantId::from_u128(1)],
        };

        assert_eq!(produced, expected);
    }
}
