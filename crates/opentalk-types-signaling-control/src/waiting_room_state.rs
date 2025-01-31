// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use opentalk_types_common::modules::{module_id, ModuleId};

/// The namespace string for the waiting room state
pub const MODULE_ID: ModuleId = module_id!("waiting_room_state");

/// The waiting room state of a meeting participant
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum WaitingRoomState {
    /// The participant currently is in the waiting room
    Waiting,

    /// The participant has been accepted into the meeting
    Accepted,
}

#[cfg(feature = "serde")]
impl opentalk_types_signaling::SignalingModulePeerFrontendData for WaitingRoomState {
    const NAMESPACE: Option<ModuleId> = Some(MODULE_ID);
}
