// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Common types related to event

use crate::{
    events::{EventId, EventTitle},
    rooms::RoomId,
    utils::ExampleData,
};

/// Information about an event
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "utoipa",
    derive(utoipa::ToSchema),
    schema(example = json!(EventInfo::example_data())),
)]
pub struct EventInfo {
    /// The id of the event
    pub id: EventId,

    /// The id of the room belonging to the event
    pub room_id: RoomId,

    /// The title of the event
    pub title: EventTitle,

    /// True if the event was created ad-hoc
    pub is_adhoc: bool,

    /// Indicates whether the meeting room should have e2e encryption enabled.
    pub e2e_encryption: bool,
}

impl ExampleData for EventInfo {
    fn example_data() -> Self {
        Self {
            id: EventId::example_data(),
            room_id: RoomId::example_data(),
            title: EventTitle::example_data(),
            is_adhoc: false,
            e2e_encryption: false,
        }
    }
}
