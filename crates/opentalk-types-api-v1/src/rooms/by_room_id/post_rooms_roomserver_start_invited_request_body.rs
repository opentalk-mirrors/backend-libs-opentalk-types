// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use opentalk_types_common::{
    rooms::{invite_codes::InviteCode, RoomPassword},
    roomserver::DeviceSecret,
    users::DisplayName,
    utils::ExampleData,
};

/// The JSON body expected when making a *POST /rooms/{room_id}/roomserver/start_invited* request
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "utoipa",
    derive(utoipa::ToSchema),
    schema(example = json!(PostRoomsRoomserverStartInvitedRequestBody::example_data()))
)]
pub struct PostRoomsRoomserverStartInvitedRequestBody {
    /// The clients device secret. Used to identify the client across sessions
    pub device_secret: DeviceSecret,

    /// The invited user's password to the room
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    // Field is non-required already, utoipa adds a `nullable: true` entry
    // by default which creates a false positive in the spectral linter when
    // combined with example data.
    #[cfg_attr(feature = "utoipa", schema(nullable = false))]
    pub password: Option<RoomPassword>,

    /// The invite code
    pub invite_code: String,

    /// The clients display name
    pub display_name: DisplayName,
}

impl ExampleData for PostRoomsRoomserverStartInvitedRequestBody {
    fn example_data() -> Self {
        Self {
            device_secret: DeviceSecret::example_data(),
            password: Some(RoomPassword::example_data()),
            invite_code: InviteCode::example_data().to_string(),
            display_name: DisplayName::example_data(),
        }
    }
}
