// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use opentalk_types_signaling::ParticipantId;

/// Describes the type of a vote stop
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case", tag = "kind", content = "issuer")
)]
pub enum StopKind {
    /// A normal vote stop issued by a participant. Contains the ParticipantId of the issuer
    ByParticipant(ParticipantId),
    /// The vote has been stopped automatically because all allowed users have voted
    Auto,
    /// The vote expired due to a set duration
    Expired,
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use opentalk_types_signaling::ParticipantId;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn serialization_by_participant_stop_kind() {
        let produced =
            serde_json::to_value(StopKind::ByParticipant(ParticipantId::from_u128(0))).unwrap();

        let expected = json!({
            "kind": "by_participant",
            "issuer": "00000000-0000-0000-0000-000000000000",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_by_participant_stop_kind() {
        let produced: StopKind = serde_json::from_value(json!({
            "kind": "by_participant",
            "issuer": "00000000-0000-0000-0000-000000000000",
        }))
        .unwrap();

        let expected = StopKind::ByParticipant(ParticipantId::from_u128(0));

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_auto_stop_kind() {
        let produced = serde_json::to_value(StopKind::Auto).unwrap();

        let expected = json!({
            "kind": "auto",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_auto_stop_kind() {
        let produced: StopKind = serde_json::from_value(json!({
            "kind": "auto",
        }))
        .unwrap();

        let expected = StopKind::Auto;

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_expired_stop_kind() {
        let produced = serde_json::to_value(StopKind::Expired).unwrap();

        let expected = json!({
            "kind": "expired",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_expired_stop_kind() {
        let produced: StopKind = serde_json::from_value(json!({
            "kind": "expired",
        }))
        .unwrap();

        let expected = StopKind::Expired;

        assert_eq!(produced, expected);
    }
}
