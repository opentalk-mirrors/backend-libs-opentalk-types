// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use opentalk_types_common::{
    rooms::{GuestAccess, RoomPassword},
    utils::ExampleData,
};

/// API request parameters to create a new room
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema), schema(example = json!(PostRoomsRequestBody::example_data())))]
pub struct PostRoomsRequestBody {
    /// The password to the room, if any
    pub password: Option<RoomPassword>,

    /// Enable/Disable sip for this room; defaults to false when not set
    #[cfg_attr(feature = "serde", serde(default))]
    pub enable_sip: bool,

    /// Indicates whether the meeting room should have the waiting room enabled.
    /// When not present, the waiting room will be disabled.
    #[cfg_attr(feature = "serde", serde(default))]
    pub waiting_room: bool,

    /// Guest access mode
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    #[cfg_attr(feature = "utoipa", schema(nullable = false))]
    pub guest_access: Option<GuestAccess>,

    /// Enable/Disable e2e encryption for this room; defaults to false when not set
    #[cfg_attr(feature = "serde", serde(default))]
    pub e2e_encryption: bool,
}

impl ExampleData for PostRoomsRequestBody {
    fn example_data() -> Self {
        Self {
            password: None,
            enable_sip: true,
            waiting_room: true,
            guest_access: Some(GuestAccess::example_data()),
            e2e_encryption: false,
        }
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use opentalk_types_common::{rooms::GuestAccess, utils::ExampleData as _};
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::PostRoomsRequestBody;

    #[test]
    fn serialize_example_data() {
        let example = PostRoomsRequestBody::example_data();
        assert_eq!(
            json!(example),
            json!({
                "password": null,
                "enable_sip": true,
                "waiting_room": true,
                "guest_access": GuestAccess::example_data(),
                "e2e_encryption": false,
            })
        );
    }

    #[test]
    fn deserialize_example_data() {
        let example = PostRoomsRequestBody::example_data();
        assert_eq!(
            example,
            serde_json::from_value(json!({
                "password": null,
                "enable_sip": true,
                "waiting_room": true,
                "guest_access": GuestAccess::example_data(),
                "e2e_encryption": false,
            }))
            .unwrap()
        );
    }
}
