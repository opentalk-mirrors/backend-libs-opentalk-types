// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! HTTP API types for the transcription service

use opentalk_types_common::{
    rooms::{BreakoutRoomId, RoomId},
    utils::ExampleData,
};

/// Body of the transcription start request
#[derive(Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "message", rename_all = "snake_case")
)]
pub struct StartTranscriptionRequestBody {
    /// ID of the room to which the transcription service should connect.
    pub room_id: RoomId,

    /// Optional breakout room ID.
    pub breakout_room: Option<BreakoutRoomId>,

    /// Target language of the transcription, specified as a two-letter language code (e.g., `de`, `en`).
    ///
    /// Set to `None` for auto-detection.
    pub language: Option<String>,
}

impl ExampleData for StartTranscriptionRequestBody {
    fn example_data() -> Self {
        Self {
            room_id: RoomId::example_data(),
            breakout_room: None,
            language: Some("de".into()),
        }
    }
}
