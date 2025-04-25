// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use crate::event::GuestParticipants;

/// The error kind sent to the user
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case", tag = "error")
)]
pub enum ErrorKind {
    /// A vote is already active
    VoteAlreadyActive,

    /// No vote is currently taking place
    NoVoteActive,

    /// The provided vote id is invalid in the requested context
    InvalidVoteId,

    /// The requesting user is ineligible
    Ineligible,

    /// The provided allow list contains guest participants
    AllowlistContainsGuests(GuestParticipants),

    /// Failed to set or get permissions
    PermissionError,

    /// The requesting user has insufficient permissions
    InsufficientPermissions,

    /// The requesting user has exceeded their storage
    StorageExceeded,

    /// A internal server error occurred
    ///
    /// This means the legal-vote module is broken, the source of this event are unrecoverable backend errors.
    Internal,
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use opentalk_types_signaling::ParticipantId;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn serialization_vote_already_active_error_kind() {
        let produced = serde_json::to_value(ErrorKind::VoteAlreadyActive).unwrap();
        let expected = json!({"error": "vote_already_active"});
        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_vote_already_active_error_kind() {
        let produced: ErrorKind =
            serde_json::from_value(json!({"error": "vote_already_active"})).unwrap();
        assert_eq!(produced, ErrorKind::VoteAlreadyActive);
    }

    #[test]
    fn serialization_no_vote_active_error_kind() {
        let produced = serde_json::to_value(ErrorKind::NoVoteActive).unwrap();
        let expected = json!({"error": "no_vote_active"});
        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_no_vote_active_error_kind() {
        let produced: ErrorKind =
            serde_json::from_value(json!({"error": "no_vote_active"})).unwrap();
        assert_eq!(produced, ErrorKind::NoVoteActive);
    }

    #[test]
    fn serialization_invalid_vote_id_error_kind() {
        let produced = serde_json::to_value(ErrorKind::InvalidVoteId).unwrap();
        let expected = json!({"error": "invalid_vote_id"});
        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_invalid_vote_id_error_kind() {
        let produced: ErrorKind =
            serde_json::from_value(json!({"error": "invalid_vote_id"})).unwrap();
        assert_eq!(produced, ErrorKind::InvalidVoteId);
    }

    #[test]
    fn serialization_ineligible_error_kind() {
        let produced = serde_json::to_value(ErrorKind::Ineligible).unwrap();
        let expected = json!({"error": "ineligible"});
        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_ineligible_error_kind() {
        let produced: ErrorKind = serde_json::from_value(json!({"error": "ineligible"})).unwrap();
        assert_eq!(produced, ErrorKind::Ineligible);
    }

    #[test]
    fn serialization_allow_list_contains_guests_error_kind() {
        let produced =
            serde_json::to_value(ErrorKind::AllowlistContainsGuests(GuestParticipants {
                guests: vec![ParticipantId::from_u128(1)],
            }))
            .unwrap();

        let expected = json!({
            "error": "allowlist_contains_guests",
            "guests": ["00000000-0000-0000-0000-000000000001"],
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_allow_list_contains_guest_error_kind() {
        let produced: ErrorKind = serde_json::from_value(json!({
            "error": "allowlist_contains_guests",
            "guests": ["00000000-0000-0000-0000-000000000001"],
        }))
        .unwrap();

        let expected = ErrorKind::AllowlistContainsGuests(GuestParticipants {
            guests: vec![ParticipantId::from_u128(1)],
        });

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_permission_error_kind() {
        let produced = serde_json::to_value(ErrorKind::PermissionError).unwrap();
        let expected = json!({"error": "permission_error"});
        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_permission_error_kind() {
        let produced: ErrorKind =
            serde_json::from_value(json!({"error": "permission_error"})).unwrap();
        assert_eq!(produced, ErrorKind::PermissionError);
    }

    #[test]
    fn serialization_internal_error_kind() {
        let produced = serde_json::to_value(ErrorKind::Internal).unwrap();
        let expected = json!({"error": "internal"});
        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_internal_error_kind() {
        let produced: ErrorKind = serde_json::from_value(json!({"error": "internal"})).unwrap();
        assert_eq!(produced, ErrorKind::Internal);
    }
}
