// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use opentalk_types_common::rooms::RoomId;

/// Request for the `POST /services/roomserver/recording/start` endpoint
#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PostRecordingStartRoomserverRequestBody {
    /// The room id
    pub room_id: RoomId,

    /// The optional breakout room id
    /// This is the internal equivalent to opentalk-roomserver-types's `BreakoutId`
    pub breakout_room: Option<u32>,
}
