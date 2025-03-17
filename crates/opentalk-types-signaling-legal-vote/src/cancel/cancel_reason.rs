// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Signaling cancel reason for the `legal-vote` namespace.

use crate::cancel::CustomCancelReason;

/// The reason for a cancel.
#[derive(Debug, Clone, Eq, PartialOrd, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case", tag = "reason", content = "custom")
)]
pub enum CancelReason {
    /// The room got destroyed and the server canceled the vote.
    RoomDestroyed,

    /// The initiator left the room and the server canceled the vote.
    InitiatorLeft,

    /// Custom reason for a cancel.
    Custom(CustomCancelReason),
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn serialization_room_destroyed_cancel_reason() {
        let produced = serde_json::to_value(CancelReason::RoomDestroyed).unwrap();

        let expected = json!({
            "reason": "room_destroyed",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_room_destroyed_cancel_reason() {
        let produced: CancelReason = serde_json::from_value(json!({
            "reason": "room_destroyed",
        }))
        .unwrap();

        let expected = CancelReason::RoomDestroyed;

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_initiator_left_cancel_reason() {
        let produced = serde_json::to_value(CancelReason::InitiatorLeft).unwrap();

        let expected = json!({
            "reason": "initiator_left",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_initiator_left_cancel_reason() {
        let produced: CancelReason = serde_json::from_value(json!({
            "reason": "initiator_left",
        }))
        .unwrap();

        let expected = CancelReason::InitiatorLeft;

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_custom_cancel_reason() {
        let produced = serde_json::to_value(CancelReason::Custom(
            CustomCancelReason::try_from("Test Reason").unwrap(),
        ))
        .unwrap();

        let expected = json!({
            "reason": "custom",
            "custom": "Test Reason",
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_custom_cancel_reason() {
        let produced: CancelReason = serde_json::from_value(json!({
            "reason": "custom",
            "custom": "Test Reason",
        }))
        .unwrap();

        let expected = CancelReason::Custom(CustomCancelReason::try_from("Test Reason").unwrap());

        assert_eq!(produced, expected);
    }
}
